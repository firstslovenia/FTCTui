use std::sync::{
    Arc,
    atomic::{AtomicI16, Ordering},
};

use crate::{
    app::get_timestamp_nanos, ftc_proto::{
        gamepad_packet::GamepadPacketData,
        heartbeat_packet::HeartbeatPacketData,
        packet::{Packet, PacketType},
        robot_command::RobotCommandPacketData,
        time_packet::TimePacketData,
        traits::{Readable, Writeable},
    }, input::Gamepad, robot::Robot
};
use tokio::{
    net::UdpSocket,
    sync::RwLock,
};

// Todo: potentially randomly generate this?
pub const SEQUENCE_NUMBER: AtomicI16 = AtomicI16::new(0);

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

/// Attempts to send a packet to the remote socket, first setting its sequence
/// number if needed and incrementing it
pub async fn send_packet(socket: &Arc<UdpSocket>, packet: Packet) {
    let mut packet = packet;

    let mut buffer = Vec::new();

    if packet.packet_type != PacketType::Heartbeat {
        packet.sequence_number = Some(SEQUENCE_NUMBER.load(Ordering::SeqCst));

        SEQUENCE_NUMBER.fetch_add(1, Ordering::SeqCst);
    }

    packet.write_to(&mut buffer);

    send_bytes(socket, buffer).await;
}

/// Attempts to send bytes to the remote socket
pub async fn send_bytes(socket: &Arc<UdpSocket>, bytes: Vec<u8>) {
    match socket.send(&bytes).await {
        Ok(num_bytes) => {
            if num_bytes != bytes.len() {
                log::warn!("Sent only {} bytes of {}!", num_bytes, bytes.len());
            }
        }
        Err(e) => {
            log::error!("Failed to send bytes: {}", e);
        }
    }
}

/// Starts the network thread, returning a handle to read debug data and a copy of the send sink
pub async fn start_network_thread(
    remote_addr: &str,
    robot: Arc<RwLock<Robot>>,
    gamepad_one: Arc<RwLock<Option<Gamepad>>>,
    gamepad_two: Arc<RwLock<Option<Gamepad>>>,
) -> (Arc<RwLock<NetworkDebugData>>, Arc<UdpSocket>) {
    log::debug!("Trying to connect to {}..", remote_addr);

    let sock = UdpSocket::bind("0.0.0.0:20884")
        .await
        .expect("Failed to bind to port 20884");

    sock.connect(remote_addr)
        .await
        .expect("Failed to connect to robot..");

    log::info!("Connected UDP socket!");

    let common_socket = Arc::new(sock);

    let sock = common_socket.clone();

    let debug = Arc::new(RwLock::new(NetworkDebugData {
        state: NetworkStatus::Connected,
    }));

    let debug_copy = debug.clone();

    tokio::task::spawn(async move {
        network_thread(sock, debug_copy, robot, gamepad_one, gamepad_two).await
    });

    log::info!("Spawned network thread!");

    (debug, common_socket)
}

pub const RECEIVE_BUFFER_SIZE: usize = 16000;

pub async fn network_thread(
    socket: Arc<UdpSocket>,
    debug: Arc<RwLock<NetworkDebugData>>,
    robot: Arc<RwLock<Robot>>,
    gamepad_one: Arc<RwLock<Option<Gamepad>>>,
    gamepad_two: Arc<RwLock<Option<Gamepad>>>,
) {
    let mut last_gamepad_update = std::time::Instant::now();
    let mut last_time_request_packet = std::time::Instant::now();
    let mut last_heartbeat_request_packet = std::time::Instant::now();

    let mut last_gamepad_1_packet = GamepadPacketData::default_for_user(1);
    let mut last_gamepad_2_packet = GamepadPacketData::default_for_user(2);

    let mut recv_buffer = [0; RECEIVE_BUFFER_SIZE];

    loop {
		  // Clear the receive buffer, so we can once again receive data into it
        recv_buffer = [0; RECEIVE_BUFFER_SIZE];

        tokio::select! {
            num_bytes_option = socket.recv(&mut recv_buffer) => {
                if let Ok(num_bytes) = num_bytes_option {
                    log::debug!("Received message of {} bytes", num_bytes);

                    if num_bytes == RECEIVE_BUFFER_SIZE {
                        log::warn!("Received full buffer, you may need to increase buffer size");
                    }

                    let mut vec_buffer = recv_buffer[0..num_bytes].to_vec();

                    match Packet::read_from(&mut vec_buffer) {
                        Some(packet) => {
                            match packet.packet_type {
                                         PacketType::Time => {
                                            let Some(data) = TimePacketData::read_from(&mut vec_buffer) else {
                                                log::warn!("Failed to deserialize time packet: {:?}", recv_buffer[0..num_bytes].to_vec());
                                                                continue;
                                            };

                                            log::debug!("Received time packet..");

                                            robot.write().await.active_opmode_state = Some(data.robot_op_mode_state);
                                         }
                                         PacketType::Gamepad => {
                                            log::warn!("Received gamepad packet from the robot, the server is likely incredibly drunk");
                                                          continue;
                                         }
                                         PacketType::Heartbeat => {
                                            let Some(data) = HeartbeatPacketData::read_from(&mut vec_buffer) else {
                                                log::warn!("Failed to deserialize heartbeat packet: {:?}", recv_buffer[0..num_bytes].to_vec());
                                                                continue;
                                            };

                                            log::debug!("Received heartbeat packet (sequence number {}), robot is running on sdk from {}/{} on {}.{}", data.sequence_number, data.sdk_build_month, data.sdk_build_year, data.sdk_major_version, data.sdk_minor_version);
                                         }
                                         PacketType::Command => {
                                            let Some(data) = RobotCommandPacketData::read_from(&mut vec_buffer) else {
                                                                log::warn!("Failed to deserialize command packet: {:?}", recv_buffer[0..num_bytes].to_vec());
                                                                continue;
                                            };

                                            log::debug!("Received command packet ({:?})", data);

                                         }
                                         PacketType::Telemetry => {
                                             let Some(data) = RobotCommandPacketData::read_from(&mut vec_buffer) else {
                                                log::warn!("Failed to deserialize command packet: {:?}", recv_buffer[0..num_bytes].to_vec());
                                                                continue;
                                            };

                                            log::debug!("Received telemetry packet.. ({:?})", data);
                                         }
                            }
                        }
                        None => {
                           log::warn!("Failed to deserialize packet: {:?}", recv_buffer[0..num_bytes].to_vec());
                        }
                    }
                }
            }

            _ = tokio::time::sleep_until((last_gamepad_update + std::time::Duration::from_millis(40)).into()) => {

                let gamepad_1 = if let Some(gp) = &*gamepad_one.read().await {
                        gp.last_state.clone()
                     } else {
                        GamepadPacketData::default_for_user(1)
                     };

                     let gamepad_2 = if let Some(gp) = &*gamepad_two.read().await {
                        gp.last_state.clone()
                     } else {
                        GamepadPacketData::default_for_user(2)
                     };

                            if gamepad_1 != last_gamepad_1_packet {
                        log::info!("Sending gamepad update for user 1..");

                                let packet = Packet::from_packet_type_and_writable(PacketType::Gamepad, &gamepad_1);

                                send_packet(&socket, packet).await;

                                last_gamepad_1_packet = gamepad_1;
                            }

                            if gamepad_2 != last_gamepad_2_packet {
                        log::info!("Sending gamepad update for user 2..");

                                let packet = Packet::from_packet_type_and_writable(PacketType::Gamepad, &gamepad_2);

                                send_packet(&socket, packet).await;

                                last_gamepad_2_packet = gamepad_2;
                            }

                        last_gamepad_update = std::time::Instant::now();
            }

				_ = tokio::time::sleep_until((last_time_request_packet + std::time::Duration::from_millis(100)).into()) => {

                     log::debug!("Sending robot time packet..");

							let unix_millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;

                     let packet = Packet::from_packet_type_and_writable(PacketType::Time, &TimePacketData {timestamp: get_timestamp_nanos(), robot_op_mode_state: 0.into(), unix_millis_sent: unix_millis, unix_millis_received_1: 0, unix_millis_received_2: 0, timezone: String::from("Europe/Ljubljana")});

					 send_packet(&socket, packet).await;

                last_heartbeat_request_packet = std::time::Instant::now();
            }

                _ = tokio::time::sleep_until((last_heartbeat_request_packet + std::time::Duration::from_secs(1)).into()) => {

                     log::debug!("Sending robot heartbeat request..");

                     let packet = Packet::from_packet_type_and_writable(PacketType::Heartbeat, &HeartbeatPacketData {sequence_number: 10003, peer_type: 1, sdk_build_month: 1, sdk_build_year: 2025, sdk_major_version: 10, sdk_minor_version: 2});

					 send_packet(&socket, packet).await;

                last_heartbeat_request_packet = std::time::Instant::now();
            }
        }
    }

    log::error!("Closing network thread..");

	 drop(socket);
    debug.write().await.state = NetworkStatus::Disconnected;
}
