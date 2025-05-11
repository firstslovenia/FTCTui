use std::sync::{
    Arc,
    atomic::{AtomicI16, Ordering},
};

use crate::{
    app::get_timestamp_nanos,
    ftc_proto::{
        gamepad_packet::GamepadPacketData,
        heartbeat_packet::HeartbeatPacketData,
        packet::{Packet, PacketType},
        robot_command::{
            CommandPacketData, NOTIFY_ACTIVE_CONFIGURATION, NOTIFY_INIT_OPMODE,
            NOTIFY_OP_MODE_STATE, NOTIFY_OP_MODES, NOTIFY_RUN_OPMODE, OpModeData,
            REQUEST_ACTIVE_CONFIGURATION, REQUEST_OP_MODES, RobotConfiguration,
        },
        telemetry_packet::{
            ROBOT_BATTERY_LEVEL_KEY, ROBOT_CONTROLLER_BATTERY_STATUS_KEY, SYSTEM_ERROR_KEY,
            SYSTEM_NONE_KEY, SYSTEM_WARNING_KEY, TelemetryEntry, TelemetryPacketData,
        },
        time_packet::{RobotOpmodeState, TimePacketData},
        traits::{Readable, Writeable},
    },
    input::Gamepad,
    robot::Robot,
};
use tokio::{net::UdpSocket, sync::RwLock};

// Todo: potentially randomly generate this?
pub static SEQUENCE_NUMBER: AtomicI16 = AtomicI16::new(0);

/// How long to wait before retransmitting packets that we didn't get an ack for
pub const RETRANSMISSION_INTERVAL: std::time::Duration = std::time::Duration::from_millis(200);

/// How long to wait before saying the connection is dead
pub const CONNECTION_TIMEOUT_INTERVAL: std::time::Duration = std::time::Duration::from_millis(2000);

#[derive(Debug, Clone, PartialEq, Eq)]
/// Debug data shared from the network thread
pub struct SharedNetworkData {
    pub state: NetworkStatus,

    /// Packets that we sent but didn't receive a response to
    ///
    /// In format of (Packet, last transmission, number of times we transmitted it)
    pub unacknowledged_command_packets: Vec<((i16, CommandPacketData), std::time::Instant, u8)>,

    /// The last time we received a packet
    pub last_received: Option<std::time::Instant>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The current state of the network connection
pub enum NetworkStatus {
    Establishing,
    Connected,
    Disconnected,
}

/// Sends a command to the remote socket, setting its sequence
/// number if and storing it if it we expect a response
pub async fn send_command(
    socket: &Arc<UdpSocket>,
    command: CommandPacketData,
    network_data: Arc<RwLock<SharedNetworkData>>,
) {
    let mut packet = Packet::from_packet_type_and_writable(PacketType::Command, &command);

    let sequence_number = SEQUENCE_NUMBER.load(Ordering::Relaxed);

    packet.sequence_number = Some(sequence_number);

    SEQUENCE_NUMBER.fetch_add(1, Ordering::SeqCst);

    log::debug!("Sending command {}", command.command);

    send_packet(socket, packet).await;

    network_data
        .write()
        .await
        .unacknowledged_command_packets
        .push(((sequence_number, command), std::time::Instant::now(), 0));
}

/// Attempts to send a packet to the remote socket, setting its sequence
/// number if needed and incrementing it
///
/// does not use the store
pub async fn send_packet(socket: &Arc<UdpSocket>, packet: Packet) {
    let mut packet = packet;

    if packet.packet_type != PacketType::Heartbeat {
        packet.sequence_number = Some(SEQUENCE_NUMBER.load(Ordering::Relaxed));

        SEQUENCE_NUMBER.fetch_add(1, Ordering::SeqCst);
    }

    send_packet_without_seq(socket, packet).await;
}

/// Attempts to send a packet to the remote socket, without setting its sequence number
pub async fn send_packet_without_seq(socket: &Arc<UdpSocket>, packet: Packet) {
    let mut buffer = Vec::new();

    packet.write_to(&mut buffer);

    /*log::trace!(
        "Sending {:?} (seq {:?}, {} bytes of data)",
        packet.packet_type,
        packet.sequence_number,
        packet.data.len()
    );*/

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

pub struct NetworkHandler {
    pub socket: Arc<UdpSocket>,
    pub shared_data: Arc<RwLock<SharedNetworkData>>,
    pub robot: Arc<RwLock<Robot>>,
    pub gamepad_one: Arc<RwLock<Option<Gamepad>>>,
    pub gamepad_two: Arc<RwLock<Option<Gamepad>>>,
}

/// Starts the network thread, returning a handle to read debug data and a reference to the socket
pub async fn start_network_thread(
    remote_addr: &str,
    robot: Arc<RwLock<Robot>>,
    gamepad_one: Arc<RwLock<Option<Gamepad>>>,
    gamepad_two: Arc<RwLock<Option<Gamepad>>>,
) -> (Arc<RwLock<SharedNetworkData>>, Arc<UdpSocket>) {
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

    let debug = Arc::new(RwLock::new(SharedNetworkData {
        state: NetworkStatus::Establishing,
        unacknowledged_command_packets: Vec::new(),
        last_received: None,
    }));

    let debug_copy = debug.clone();

    let mut network_handler = NetworkHandler {
        shared_data: debug_copy,
        socket: sock,
        robot,
        gamepad_one,
        gamepad_two,
    };

    tokio::task::spawn(async move { network_handler.network_thread().await });

    log::info!("Spawned network thread!");

    (debug, common_socket)
}

pub const RECEIVE_BUFFER_SIZE: usize = 16000;

impl NetworkHandler {
    pub async fn network_thread(&mut self) {
        let mut last_gamepad_update = std::time::Instant::now();
        let mut last_time_request_packet = std::time::Instant::now();
        let mut last_heartbeat_request_packet = std::time::Instant::now();
        let mut last_retransmit_check = std::time::Instant::now();

        let mut last_gamepad_1_packet = GamepadPacketData::default_for_user(1);
        let mut last_gamepad_2_packet = GamepadPacketData::default_for_user(2);

        // Buffer we'll receive bytes into
        //
        // The size of this buffer has to be the largest possible packet size
        //
        // Allocating like 16 kb of ram shouldn't be an issue
        let mut recv_buffer: [u8; RECEIVE_BUFFER_SIZE];

        // As a start, request opmodes and active configuration
        //
        // The official client also requests the hardware configuration, but we haven't implemented
        // that yet
        send_command(
            &self.socket,
            CommandPacketData {
                data: String::new(),
                command: REQUEST_OP_MODES.to_string(),
                timestamp: get_timestamp_nanos(),
                acknowledged: false,
            },
            self.shared_data.clone(),
        )
        .await;

        send_command(
            &self.socket,
            CommandPacketData {
                data: String::new(),
                command: REQUEST_ACTIVE_CONFIGURATION.to_string(),
                timestamp: get_timestamp_nanos(),
                acknowledged: false,
            },
            self.shared_data.clone(),
        )
        .await;

        loop {
            // Clear the receive buffer, so we can once again receive data into it
            recv_buffer = [0; RECEIVE_BUFFER_SIZE];

            // Check network status
            let mut shared_write = self.shared_data.write().await;

            if let Some(last_received) = shared_write.last_received {
                if last_received.elapsed() >= CONNECTION_TIMEOUT_INTERVAL {
                    shared_write.state = NetworkStatus::Disconnected;
                }
            }

            drop(shared_write);

            tokio::select! {
                num_bytes_option = self.socket.recv(&mut recv_buffer) => {
                    if let Ok(num_bytes) = num_bytes_option {
                        log::debug!("Received message of {} bytes", num_bytes);

                        if num_bytes == RECEIVE_BUFFER_SIZE {
                            log::warn!("Received full buffer, you may need to increase buffer size");
                        }

                        let mut vec_buffer = recv_buffer[0..num_bytes].to_vec();

                        // Update network status
                        let mut shared_write = self.shared_data.write().await;
                        shared_write.last_received = Some(std::time::Instant::now());
                        shared_write.state = NetworkStatus::Connected;
                        drop(shared_write);

                        match Packet::read_from(&mut vec_buffer) {
                            Some(packet) => {
                                match packet.packet_type {
                                             PacketType::Time => {
                                                let Some(data) = TimePacketData::read_from(&mut vec_buffer) else {
                                                    log::warn!("Failed to deserialize time packet: {:?}", recv_buffer[0..num_bytes].to_vec());
                                                                    continue;
                                                };

                                                log::debug!("Received time packet..");

                                                self.robot.write().await.active_opmode_state = Some(data.robot_op_mode_state);
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
                                                let Some(data) = CommandPacketData::read_from(&mut vec_buffer) else {
                                                                    log::warn!("Failed to deserialize command packet: {:?}", recv_buffer[0..num_bytes].to_vec());
                                                                    continue;
                                                };

                                                log::debug!("Received command packet ({:?})", data);

                                             }
                                             PacketType::Telemetry => {
                                                 let Some(data) = CommandPacketData::read_from(&mut vec_buffer) else {
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

                    let gamepad_1 = if let Some(gp) = &*self.gamepad_one.read().await {
                            gp.last_state.clone()
                         } else {
                            GamepadPacketData::default_for_user(1)
                         };

                         let gamepad_2 = if let Some(gp) = &*self.gamepad_two.read().await {
                            gp.last_state.clone()
                         } else {
                            GamepadPacketData::default_for_user(2)
                         };

                                if gamepad_1 != last_gamepad_1_packet {
                            log::info!("Sending gamepad update for user 1..");

                                    let packet = Packet::from_packet_type_and_writable(PacketType::Gamepad, &gamepad_1);

                                    send_packet(&self.socket, packet).await;

                                    last_gamepad_1_packet = gamepad_1;
                                }

                                if gamepad_2 != last_gamepad_2_packet {
                            log::info!("Sending gamepad update for user 2..");

                                    let packet = Packet::from_packet_type_and_writable(PacketType::Gamepad, &gamepad_2);

                                    send_packet(&self.socket, packet).await;

                                    last_gamepad_2_packet = gamepad_2;
                                }

                            last_gamepad_update = std::time::Instant::now();
                }

                    _ = tokio::time::sleep_until((last_time_request_packet + std::time::Duration::from_millis(100)).into()) => {

                         log::debug!("Sending robot time packet..");

                                let unix_millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;

                         let packet = Packet::from_packet_type_and_writable(PacketType::Time, &TimePacketData {timestamp: get_timestamp_nanos(), robot_op_mode_state: 0.into(), unix_millis_sent: unix_millis, unix_millis_received_1: 0, unix_millis_received_2: 0, timezone: String::from("Europe/Ljubljana")});

                         send_packet(&self.socket, packet).await;

                    last_time_request_packet = std::time::Instant::now();
                }

                    _ = tokio::time::sleep_until((last_heartbeat_request_packet + std::time::Duration::from_secs(1)).into()) => {

                         log::debug!("Sending robot heartbeat request..");

                         let packet = Packet::from_packet_type_and_writable(PacketType::Heartbeat, &HeartbeatPacketData {sequence_number: 10003, peer_type: 1, sdk_build_month: 1, sdk_build_year: 2025, sdk_major_version: 10, sdk_minor_version: 2});

                         send_packet(&self.socket, packet).await;

                    last_heartbeat_request_packet = std::time::Instant::now();
                }

                _ = tokio::time::sleep_until((last_retransmit_check + std::time::Duration::from_millis(10)).into()) => {
                        let mut shared_write = self.shared_data.write().await;

                         // If we're disconnected, don't bother
                         if shared_write.state == NetworkStatus::Disconnected {
                                    last_retransmit_check = std::time::Instant::now();
                           continue;
                         }

                        let mut packet_i = 0;

                        for mut _j in 0..shared_write.unacknowledged_command_packets.len() {

                            let Some(packet) = shared_write.unacknowledged_command_packets.get_mut(packet_i) else {
                                break;
                            };

                            if packet.2 >= 10 {
                                log::warn!("Giving up on command packet ({}), sent it 10x with no response", packet.0.1.command);

                                let _ = shared_write.unacknowledged_command_packets.remove(packet_i);

                                // Check the same index again, since it's a different entry now
                                continue;
                            }

                            if packet.1.elapsed() >= RETRANSMISSION_INTERVAL {
                                log::info!("Retransmitting {} command packet, since we got no response (x{})", packet.0.1.command, packet.2);

                                          // Transmit with the same sequence number
                                          let mut new_packet = Packet::from_packet_type_and_writable(PacketType::Command, &packet.0.1);
                                          new_packet.sequence_number = Some(packet.0.0);

                                send_packet_without_seq(&self.socket, new_packet).await;

                                          packet.1 = std::time::Instant::now();
                                          packet.2 += 1;
                            }

                            packet_i += 1;
                        }

                        last_retransmit_check = std::time::Instant::now();
                     }
            }
        }

        log::error!("Closing network thread..");

        drop(self.socket);
        self.shared_data.write().await.state = NetworkStatus::Disconnected;
    }

    /// Handles receiving a telemetry packet
    pub async fn handle_telemetry_packet(&mut self, packet: TelemetryPacketData) {
        if !packet.tag.is_empty() {
            match packet.tag.as_str() {
                ROBOT_BATTERY_LEVEL_KEY => {
                    let entry = packet.string_entries[0].clone();

                    self.update_battery_voltage_from_telemetry_entry(entry)
                        .await;
                }

                ROBOT_CONTROLLER_BATTERY_STATUS_KEY => {
                    let entry = packet.string_entries[0].clone();

                    if entry.key != ROBOT_CONTROLLER_BATTERY_STATUS_KEY {
                        log::warn!(
                            "Key mismatch: got {}, expected {}",
                            entry.key,
                            ROBOT_CONTROLLER_BATTERY_STATUS_KEY
                        );
                        return;
                    }

                    log::info!("Got robot controller battery status: {}", entry.value);
                }

                SYSTEM_WARNING_KEY => {
                    let entry = packet.string_entries[0].clone();

                    if entry.key != SYSTEM_WARNING_KEY {
                        log::warn!(
                            "Key mismatch: got {}, expected {}",
                            entry.key,
                            SYSTEM_WARNING_KEY
                        );
                        return;
                    }

                    self.robot.write().await.warning_message = Some(entry.value);
                }

                SYSTEM_ERROR_KEY => {
                    let entry = packet.string_entries[0].clone();

                    if entry.key != SYSTEM_ERROR_KEY {
                        log::warn!(
                            "Key mismatch: got {}, expected {}",
                            entry.key,
                            SYSTEM_ERROR_KEY
                        );
                        return;
                    }

                    self.robot.write().await.error_message = Some(entry.value);
                }

                SYSTEM_NONE_KEY => {
                    let entry = packet.string_entries[0].clone();

                    if entry.key != SYSTEM_NONE_KEY {
                        log::warn!(
                            "Key mismatch: got {}, expected {}",
                            entry.key,
                            SYSTEM_NONE_KEY
                        );
                        return;
                    }

                    if !entry.value.is_empty() {
                        log::warn!("System none data wasn't empty ({})", entry.value);
                    }

                    self.robot.write().await.warning_message = None;
                    self.robot.write().await.error_message = None;
                }

                &_ => {
                    log::warn!("Got unexpected tag: {:?}", packet.tag);
                    return;
                }
            }
        }

        let mut user_telemetry_lines = Vec::new();

        for entry in packet.string_entries {
            if entry.key == ROBOT_BATTERY_LEVEL_KEY {
                self.update_battery_voltage_from_telemetry_entry(entry)
                    .await;
            } else {
                // All user telemetry keys start with the null byte
                if entry.key.starts_with('\0') {
                    user_telemetry_lines.push(entry.value);
                } else {
                    log::warn!(
                        "Unexpected telemetry pair: {:?} - {:?}",
                        entry.key,
                        entry.value
                    );
                }
            }
        }

        self.robot.write().await.telemetry_list = user_telemetry_lines;

        if packet.float_entries.len() > 0 {
            log::warn!("Received some float entries! {:?}", packet.float_entries);
        }
    }

    /// Attempts to update the robot's battery voltage from a telemetry key we got
    pub async fn update_battery_voltage_from_telemetry_entry(&mut self, entry: TelemetryEntry) {
        if entry.key != ROBOT_BATTERY_LEVEL_KEY {
            log::warn!(
                "Key mismatch: got {}, expected {}",
                entry.key,
                ROBOT_BATTERY_LEVEL_KEY
            );
            return;
        }

        match entry.value.parse::<f32>() {
            Ok(battery_voltage) => {
                let mut robot_write = self.robot.write().await;

                robot_write.battery_voltage = Some(battery_voltage);
                robot_write.last_battery_update = std::time::Instant::now();

                drop(robot_write);
            }
            Err(e) => {
                log::warn!(
                    "Failed to parse battery voltage key: {} ({})",
                    e,
                    entry.value
                );
            }
        }
    }

    /// Handles receiving a command packet
    pub async fn handle_command_packet(&mut self, packet: CommandPacketData) {
        if packet.acknowledged {
            log::debug!("Received acknowledge for command {}", packet.command);

            let mut shared_debug = self.shared_data.write().await;

            let mut i = 0;

            for _j in 0..shared_debug.unacknowledged_command_packets.len() {
                let Some(old_packet) = shared_debug.unacknowledged_command_packets.get(i) else {
                    break;
                };

                if old_packet.0.1.command == packet.command {
                    log::debug!("{} command ACK by the server", old_packet.0.1.command);

                    let _ = shared_debug.unacknowledged_command_packets.remove(i);
                    continue;
                }

                i += 1;
            }

            return;
        }

        log::info!("Received command {}", packet.command);

        // Send back an ackowledge for this
        send_packet(
            &self.socket,
            Packet::from_packet_type_and_writable(
                PacketType::Command,
                &CommandPacketData {
                    command: packet.command.clone(),
                    acknowledged: true,
                    data: String::new(),
                    timestamp: get_timestamp_nanos(),
                },
            ),
        )
        .await;

        match packet.command.as_str() {
            NOTIFY_OP_MODE_STATE => match packet.data.parse::<i8>() {
                Ok(i8) => {
                    let state = RobotOpmodeState::from(i8);

                    self.robot.write().await.active_opmode_state = Some(state);
                }
                Err(e) => {
                    log::warn!("Failed to parse opmode state: {} ({})", e, packet.data);
                }
            },
            NOTIFY_OP_MODES => match serde_json::from_str::<Vec<OpModeData>>(&packet.data) {
                Ok(opmode_list) => {
                    self.robot.write().await.opmode_list = Some(opmode_list);
                }
                Err(e) => {
                    log::warn!("Failed to parse opmode list: {} ({})", e, packet.data);
                }
            },
            NOTIFY_ACTIVE_CONFIGURATION => {
                match serde_json::from_str::<RobotConfiguration>(&packet.data) {
                    Ok(config) => {
                        // todo: save this somewhere and show it on the ui
                        log::info!("Received robot configuration: {:?}", config);
                    }
                    Err(e) => {
                        log::warn!(
                            "Failed to parse robot configuration: {} ({})",
                            e,
                            packet.data
                        );
                    }
                }
            }
            NOTIFY_INIT_OPMODE => {
                let mut robot_write = self.robot.write().await;

                robot_write.active_opmode = packet.data;
                robot_write.active_opmode_state = Some(RobotOpmodeState::Initialized);

                drop(robot_write);
            }
            NOTIFY_RUN_OPMODE => {
                let mut robot_write = self.robot.write().await;

                robot_write.active_opmode = packet.data;
                robot_write.active_opmode_state = Some(RobotOpmodeState::Running);

                drop(robot_write);
            }
            _ => {
                log::warn!(
                    "Received unhandled command: {} (data: {})",
                    packet.command,
                    packet.data
                );
            }
        }
    }
}
