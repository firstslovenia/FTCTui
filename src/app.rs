use std::sync::Arc;

use color_eyre::eyre::Result;
use futures::SinkExt;
use gilrs::Gilrs;
use ratatui::DefaultTerminal;
use tokio::sync::{Mutex, RwLock};

use crate::{
    ftc_dashboard::{message::Message, robot_status::RobotStatus},
    input::Gamepad,
    network::{NetworkDebugData, Sink},
    robot::Robot,
};

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
    pub robot: Arc<RwLock<Option<Robot>>>,

    /// A Shared Sink to send messages to the robot
    pub sink: Arc<Mutex<Sink>>,

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
        let robot = Arc::new(RwLock::new(Some(Robot {
            status: None,
            opmode_list: None,
            last_status_update: std::time::Instant::now(),
        })));

        let gamepad_one = Arc::new(RwLock::new(None));
        let gamepad_two = Arc::new(RwLock::new(None));

        let (network_debug_data, sink) = crate::network::start_network_thread(
            "ws://192.168.43.1:8000",
            robot.clone(),
            gamepad_one.clone(),
            gamepad_two.clone(),
        )
        .await;

        App {
            sink,
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

    /// Sends a websocket message to the robot.
    pub async fn send_message(&self, message: Message) {
        let as_string = serde_json::to_string(&message).unwrap();

        log::debug!("Sending {}", as_string);

        self.sink
            .lock()
            .await
            .send(tokio_tungstenite::tungstenite::Message::text(as_string))
            .await
            .unwrap();
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

    /// Set running to false to quit the application.
    pub async fn quit(&mut self) {
        self.running = false;
    }
}
