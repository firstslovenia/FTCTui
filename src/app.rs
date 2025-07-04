use std::{sync::Arc, time::SystemTime};

use color_eyre::eyre::Result;
use gilrs::{Gilrs, GilrsBuilder};
use lazy_static::lazy_static;
use ratatui::{
    DefaultTerminal,
    widgets::{ListState, Paragraph, Wrap},
};
use tokio::{
    net::UdpSocket,
    sync::{Mutex, RwLock},
};

use crate::{
    Args,
    ftc_proto::robot_command::{
        CommandPacketData, INIT_OPMODE, OPMODE_STOP, OpModeData, OpModeFlavor, RUN_OPMODE,
    },
    gamepad_map::REV_CONTROLLER_CUSTOM_SDL_MAPPING_LINUX,
    input::Gamepad,
    network::{SharedNetworkData, TELEMETRY_LOG_FILENAME, send_command},
    popup::{InfoPopup, Popup},
    robot::Robot,
};

lazy_static! {
    /// When the app launched
    pub static ref STARTED_AT: SystemTime = SystemTime::now();
}

pub const DEBUG_BLOCK_ID: u8 = 0;
pub const TELEOP_BLOCK_ID: u8 = 1;
pub const AUTO_BLOCK_ID: u8 = 2;
pub const ROBOT_BLOCK_ID: u8 = 3;
pub const ACTIVE_OPMODE_BLOCK_ID: u8 = 4;
pub const GAMEPADS_BLOCK_ID: u8 = 5;

/// The main application which holds the state and logic of the application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// Our shared robot data
    pub robot: Arc<RwLock<Robot>>,

    /// A Shared Socket to send messages to the robot
    pub socket: Arc<UdpSocket>,

    /// Shared network data
    pub shared_network_data: Arc<RwLock<SharedNetworkData>>,

    /// The main "block" the user has selected, going from the top left to the bottom right
    pub selected_block: u8,

    /// The teleop opmode from the user currently has selected
    pub teleop_list_state: ListState,

    /// The auto opmode from the user currently has selected
    pub auto_list_state: ListState,

    /// Number of lines scrolled in the telemetry display
    pub telemetry_display_scroll: u16,

    /// Our current command buffer, if in [AppMode::InsertCommand]
    pub current_command: String,

    /// What "mode" we're in, mostly used for input handling
    ///
    /// See [AppMode]
    pub mode: AppMode,

    /// Our active popup, if we have one
    pub active_popup: Option<Arc<Mutex<dyn Popup>>>,

    /// Handle of our gamepad input handler
    pub gilrs: Gilrs,
    pub gamepad_one: Arc<RwLock<Option<Gamepad>>>,
    pub gamepad_two: Arc<RwLock<Option<Gamepad>>>,
}

impl App {
    /// Construct a new instance of [`App`].
    pub async fn new(args: Args) -> Self {
        // Clear an existing telemetry log, so we write into it
        if args.export_telemetry {
            let _ = std::fs::remove_file(TELEMETRY_LOG_FILENAME);
        }

        let robot = Arc::new(RwLock::new(Robot::new_empty()));

        let gamepad_one = Arc::new(RwLock::new(None));
        let gamepad_two = Arc::new(RwLock::new(None));

        let (network_debug_data, socket) = crate::network::start_network_thread(
            "192.168.43.1:20884",
            robot.clone(),
            gamepad_one.clone(),
            gamepad_two.clone(),
            args.export_telemetry,
        )
        .await;

        let gilrs = GilrsBuilder::new()
            .add_mappings(REV_CONTROLLER_CUSTOM_SDL_MAPPING_LINUX)
            .build()
            .expect("Failed to build gilrs object");

        App {
            socket,
            shared_network_data: network_debug_data,
            running: false,
            selected_block: 1,
            robot,
            teleop_list_state: ListState::default().with_selected(Some(0)),
            auto_list_state: ListState::default().with_selected(Some(0)),
            telemetry_display_scroll: 0,
            gilrs,
            gamepad_one,
            gamepad_two,
            mode: AppMode::Normal,
            current_command: String::with_capacity(32),
            active_popup: None,
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;

        let mut last_frame;

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

    /// Returns a sorted list of opmodes
    ///
    /// Should always return the same order
    pub async fn get_opmodes(&self) -> Vec<OpModeData> {
        let mut opmodes = self
            .robot
            .read()
            .await
            .opmode_list
            .clone()
            .unwrap_or_default();

        // First sort by group, then by name
        opmodes.sort_by(|a, b| a.group.cmp(&b.group).then_with(|| a.name.cmp(&b.name)));

        opmodes
    }

    /// Returns a sorted list of teleop modes
    ///
    /// Should always return the same order or opmodes
    pub async fn get_teleop_opmodes(&self) -> Vec<OpModeData> {
        let opmodes = self.get_opmodes().await;

        opmodes
            .into_iter()
            .filter(|x| x.flavor == OpModeFlavor::Teleop)
            .collect()
    }

    /// Returns a sorted list of auto modes
    ///
    /// Should always return the same order or opmodes
    pub async fn get_auto_opmodes(&self) -> Vec<OpModeData> {
        let opmodes = self.get_opmodes().await;

        opmodes
            .into_iter()
            .filter(|x| x.flavor == OpModeFlavor::Autonomous)
            .collect()
    }

    /// Fetches the currently selected opmode, if any
    pub async fn get_selected_opmode(&self) -> Option<OpModeData> {
        match self.selected_block {
            TELEOP_BLOCK_ID => self
                .get_teleop_opmodes()
                .await
                .get(self.teleop_list_state.selected().unwrap_or_default())
                .cloned(),

            AUTO_BLOCK_ID => self
                .get_auto_opmodes()
                .await
                .get(self.auto_list_state.selected().unwrap_or_default())
                .cloned(),
            _ => None,
        }
    }

    /// Starts an opmode with the given name
    pub async fn start_opmode(&self, opmode: String) {
        send_command(
            &self.socket,
            CommandPacketData {
                acknowledged: false,
                command: RUN_OPMODE.to_string(),
                data: opmode,
                timestamp: get_timestamp_nanos(),
            },
            self.shared_network_data.clone(),
        )
        .await;
    }

    /// Inits an opmode with the given name
    pub async fn init_opmode(&self, opmode: String) {
        send_command(
            &self.socket,
            CommandPacketData {
                acknowledged: false,
                command: INIT_OPMODE.to_string(),
                data: opmode,
                timestamp: get_timestamp_nanos(),
            },
            self.shared_network_data.clone(),
        )
        .await;
    }

    /// Stops the current opmode
    pub async fn stop_opmode(&self) {
        send_command(
            &self.socket,
            CommandPacketData {
                acknowledged: false,
                command: INIT_OPMODE.to_string(),
                data: OPMODE_STOP.to_string(),
                timestamp: get_timestamp_nanos(),
            },
            self.shared_network_data.clone(),
        )
        .await;
    }

    /// Set running to false to quit the application.
    pub async fn quit(&mut self) {
        self.running = false;
    }
}

/// What mode our UI is in, denotes what different inputs mean
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum AppMode {
    /// Tab switches selected elements, : opens command mode, different keys are hotkeys
    #[default]
    Normal,
    /// Enter submits the current command, escape returns to normal mode, character keys are added
    /// to the command buffer
    InsertCommand,
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
