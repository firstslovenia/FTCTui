use std::sync::Arc;

use futures::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use tokio::{
    net::TcpStream,
    sync::{Mutex, RwLock},
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use crate::{
    ftc_dashboard::gamepad_state::GamepadState,
    ftc_dashboard::message::{Message, ReceiveGamepadState},
    input::Gamepad,
    robot::Robot,
};

use log::{error, info, trace, warn};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Debug data shared from the network thread
pub struct NetworkDebugData {
    pub state: NetworkStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The current state of the network connection
pub enum NetworkStatus {
    Connected,
    Disconnected,
}

pub type Sink =
    SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>;
pub type Stream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

/// Starts the network thread, returning a handle to read debug data and a copy of the send sink
pub async fn start_network_thread(
    url: &str,
    robot: Arc<RwLock<Option<Robot>>>,
    gamepad_one: Arc<RwLock<Option<Gamepad>>>,
    gamepad_two: Arc<RwLock<Option<Gamepad>>>,
) -> (Arc<RwLock<NetworkDebugData>>, Arc<Mutex<Sink>>) {
    log::debug!("Trying to connect to {}..", url);

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    log::info!("Connected websocket!");

    let (write, mut read) = ws_stream.split();

    let shared_write = Arc::new(Mutex::new(write));
    let shared_write_copy = shared_write.clone();

    let debug = Arc::new(RwLock::new(NetworkDebugData {
        state: NetworkStatus::Connected,
    }));

    let debug_copy = debug.clone();

    tokio::task::spawn(async move {
        network_thread(
            shared_write_copy,
            &mut read,
            debug_copy,
            robot,
            gamepad_one,
            gamepad_two,
        )
        .await
    });

    log::info!("Spawned network thread!");

    (debug, shared_write)
}

pub async fn network_thread(
    write: Arc<Mutex<Sink>>,
    read: &mut Stream,
    debug: Arc<RwLock<NetworkDebugData>>,
    robot: Arc<RwLock<Option<Robot>>>,
    gamepad_one: Arc<RwLock<Option<Gamepad>>>,
    gamepad_two: Arc<RwLock<Option<Gamepad>>>,
) {
    let mut last_gamepad_update = std::time::Instant::now();
    let mut last_robot_status_request = std::time::Instant::now();

    let mut last_gamepad_state = ReceiveGamepadState {
        gamepad1: GamepadState::default(),
        gamepad2: GamepadState::default(),
    };

    loop {
        tokio::select! {
            msg = read.next() => {
                if let Some(Ok(message)) = msg {
                    let Ok(as_text) = message.into_text() else {
                        break;
                    };

                          log::debug!("Received message: {}", as_text);

                    match serde_json::from_str::<Message>(&as_text) {
                        Ok(deserialized_message) => {
                            match deserialized_message {
                                          Message::ReceiveRobotStatus(status) => {
                                                                log::info!("Received RobotStatus: {:?}", status);

                                                let mut robot_write = robot.write().await;

                                                if let Some(robot) = &mut *robot_write {
                                                    robot.status = Some(status.status);
                                                    robot.last_status_update = std::time::Instant::now();
                                                }

                                                else {
                                                    *robot_write = Some(Robot {status: Some(status.status), opmode_list: None, last_status_update: std::time::Instant::now()});
                                                }
                                          }

                                                        Message::ReceiveOpModeList(opmodes) => {

                                                                log::info!("Received OpModes: {:?}", opmodes);

                                                let mut robot_write = robot.write().await;

                                                if let Some(robot) = &mut *robot_write {
                                                                    robot.opmode_list = Some(opmodes.op_mode_list);
                                                }

                                                else {
                                                    *robot_write = Some(Robot {status: None, opmode_list: Some(opmodes.op_mode_list), last_status_update: std::time::Instant::now()});
                                                }
                                          }
                                _ => {
                                                log::warn!("Received unrecognized message: {}", as_text);
                                          }
                            }
                        }
                        Err(e) => {
                                     log::warn!("Failed to deserialize message: {}", e);
                        }
                    }
                }
            }

            _ = tokio::time::sleep_until((last_gamepad_update + std::time::Duration::from_micros(10)).into()) => {

                let gamepad1 = if let Some(gp) = &*gamepad_one.read().await {
                        gp.last_state.clone()
                     } else {
                        GamepadState::default()
                     };

                     let gamepad2 = if let Some(gp) = &*gamepad_two.read().await {
                        gp.last_state.clone()
                     } else {
                        GamepadState::default()
                     };

                     let gamepad_state = ReceiveGamepadState { gamepad1, gamepad2 };


                        log::info!("Sending gamepad update..");

                        let packet = Message::ReceiveGamepadState(gamepad_state.clone());

                        let as_string = serde_json::to_string(&packet).unwrap();

                        log::info!("{}", as_string);

                        write.lock().await.send(tokio_tungstenite::tungstenite::Message::text(as_string)).await.unwrap();

                        last_gamepad_update = std::time::Instant::now();
                        last_gamepad_state = gamepad_state;
            }

                _ = tokio::time::sleep_until((last_robot_status_request + std::time::Duration::from_millis(100)).into()) => {

                     log::debug!("Sending robot status request..");

                let packet = Message::GetRobotStatus;

                let as_string = serde_json::to_string(&packet).unwrap();

                     log::debug!("{}", as_string);

                write.lock().await.send(tokio_tungstenite::tungstenite::Message::text(as_string)).await.unwrap();

                last_robot_status_request = std::time::Instant::now();
            }
        }
    }

    log::error!("Closing network thread..");

    let _ = write.lock().await.close().await;
    debug.write().await.state = NetworkStatus::Disconnected;
}
