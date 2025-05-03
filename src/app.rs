use std::{sync::Arc, time::SystemTime};

use color_eyre::eyre::Result;
use futures::SinkExt;
use gilrs::Gilrs;
use lazy_static::lazy_static;
use ratatui::DefaultTerminal;
use tokio::{
    net::UdpSocket,
    sync::{Mutex, RwLock},
};

use crate::{
    ftc_dashboard::{
        message::Message,
        robot_status::{OpModeStatus, RobotStatus},
    },
    ftc_proto::{
        packet::{Packet, PacketType},
        robot_command::{
            DEFAULT_OPMODE_GROUP, INIT_OPMODE, OPMODE_STOP, OpModeData, OpModeFlavor, RUN_OPMODE,
            RobotCommandPacketData,
        },
        time_packet::RobotOpmodeState,
    },
    input::Gamepad,
    network::{NetworkDebugData, send_packet},
    robot::Robot,
};

lazy_static! {
    /// When the app launched
    pub static ref STARTED_AT: SystemTime = SystemTime::now();
}

pub const DEBUG_BLOCK_ID: u8 = 0;
pub const OP_MODES_BLOCK_ID: u8 = 1;
pub const ROBOT_BLOCK_ID: u8 = 2;
pub const ACTIVE_OPMODE_BLOCK_ID: u8 = 3;
pub const GAMEPADS_BLOCK_ID: u8 = 4;

/// The main application which holds the state and logic of the application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// Our shared robot data
    pub robot: Arc<RwLock<Robot>>,

    /// A Shared Socket to send messages to the robot
    pub socket: Arc<UdpSocket>,

    /// Network debug data
    pub network_debug_data: Arc<RwLock<NetworkDebugData>>,

    /// The main "block" the user has selected, going from the top left to the bottom right
    pub selected_block: u8,
    /// The opmode from the opmode list the user currently has selected
    pub opmode_list_selected_index: usize,

    /// Handle of our gamepad input handler
    pub gilrs: Gilrs,
    pub gamepad_one: Arc<RwLock<Option<Gamepad>>>,
    pub gamepad_two: Arc<RwLock<Option<Gamepad>>>,
}

impl App {
    /// Construct a new instance of [`App`].
    pub async fn new() -> Self {
        let robot = Arc::new(RwLock::new(Robot {
            active_opmode_state: Some(RobotOpmodeState::Running),
            opmode_list: Some(vec![
                OpModeData {
                    name: OPMODE_STOP.to_string(),
                    group: DEFAULT_OPMODE_GROUP.to_string(),
                    flavor: OpModeFlavor::System,
                    source: None,
                    system_opmode_display_name: None,
                },
                OpModeData {
                    name: "Robot".to_string(),
                    group: DEFAULT_OPMODE_GROUP.to_string(),
                    flavor: OpModeFlavor::Teleop,
                    source: None,
                    system_opmode_display_name: None,
                },
                OpModeData {
                    name: "CoolerRobot".to_string(),
                    group: DEFAULT_OPMODE_GROUP.to_string(),
                    flavor: OpModeFlavor::Teleop,
                    source: None,
                    system_opmode_display_name: None,
                },
            ]),
            active_opmode: OPMODE_STOP.to_string(),
            error_message: None,
            warning_message: Some(String::from("Test warning message")),
            battery_voltage: Some(12.3),
            last_battery_update: std::time::Instant::now(),
        }));

        let gamepad_one = Arc::new(RwLock::new(None));
        let gamepad_two = Arc::new(RwLock::new(None));

        let (network_debug_data, socket) = crate::network::start_network_thread(
            "192.168.43.1:20884",
            robot.clone(),
            gamepad_one.clone(),
            gamepad_two.clone(),
        )
        .await;

        App {
            socket,
            network_debug_data,
            running: false,
            selected_block: 1,
            robot,
            opmode_list_selected_index: 0,
            gilrs: Gilrs::new().unwrap(),
            gamepad_one,
            gamepad_two,
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;

        let mut last_frame = std::time::Instant::now();

        while self.running {
            last_frame = std::time::Instant::now();

            terminal.draw(|frame| futures::executor::block_on(self.render(frame)))?;

            self.handle_crossterm_events().await?;
            self.update_gamepads().await;

            // Lock at 30 fps
            tokio::time::sleep_until((last_frame + std::time::Duration::from_millis(33)).into())
                .await;
        }
        Ok(())
    }

    /// Starts an opmode with the given name
    pub async fn start_opmode(&self, opmode: String) {
        send_packet(
            &self.socket,
            Packet::from_packet_type_and_writable(
                PacketType::Command,
                &RobotCommandPacketData {
                    acknowledged: false,
                    command: RUN_OPMODE.to_string(),
                    data: opmode,
                    timestamp: get_timestamp_nanos(),
                },
            ),
        )
        .await;
    }

    /// Inits an opmode with the given name
    pub async fn init_opmode(&self, opmode: String) {
        send_packet(
            &self.socket,
            Packet::from_packet_type_and_writable(
                PacketType::Command,
                &RobotCommandPacketData {
                    acknowledged: false,
                    command: INIT_OPMODE.to_string(),
                    data: opmode,
                    timestamp: get_timestamp_nanos(),
                },
            ),
        )
        .await;
    }

    /// Stops the current opmode
    pub async fn stop_opmode(&self) {
        send_packet(
            &self.socket,
            Packet::from_packet_type_and_writable(
                PacketType::Command,
                &RobotCommandPacketData {
                    acknowledged: false,
                    command: INIT_OPMODE.to_string(),
                    data: OPMODE_STOP.to_string(),
                    timestamp: get_timestamp_nanos(),
                },
            ),
        )
        .await;
    }

    /// Set running to false to quit the application.
    pub async fn quit(&mut self) {
        self.running = false;
    }
}

/// Gets a millis timestamp of the app's uptime
///
/// Used for certain packets
pub fn get_timestamp_millis() -> u64 {
    STARTED_AT.elapsed().unwrap().as_millis() as u64
}

/// Gets a nanos timestamp of the app's uptime
///
/// Used for certain packets
pub fn get_timestamp_nanos() -> u64 {
    STARTED_AT.elapsed().unwrap().as_nanos() as u64
}
