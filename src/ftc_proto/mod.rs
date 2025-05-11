//! Includes rust types for FTC's protocol between the FTC robot controller app and a control hub

pub mod gamepad_packet;
pub mod heartbeat_packet;
pub mod packet;
pub mod robot_command;
pub mod telemetry_packet;
pub mod time_packet;
pub mod traits;

pub mod test_deserializer {

    use super::{
        gamepad_packet::GamepadPacketData,
        heartbeat_packet::HeartbeatPacketData,
        packet::{Packet, PacketType},
        robot_command::CommandPacketData,
        telemetry_packet::TelemetryPacketData,
        time_packet::TimePacketData,
        traits::Readable,
    };

    /// Tests deserializing packets
    pub async fn test_deserializer() {
        let mut success: usize = 0;
        let mut i: usize = 0;

        let mut binary = tokio::fs::read("secret path to .bin file").await.unwrap();

        let started = tokio::time::Instant::now();

        loop {
            if binary.len() == 0 {
                break;
            }

            match Packet::read_from(&mut binary) {
                None => {
                    println!("Failed to read packet {}!", i);
                    break;
                }
                Some(mut packet) => {
                    i += 1;

                    match packet.packet_type {
                        PacketType::Telemetry => {
                            match TelemetryPacketData::read_from(&mut packet.data) {
                                None => {
                                    println!("Failed to {} read as telemetry packet!", i);
                                }
                                Some(_p) => {
                                    success += 1;
                                    //println!("{} - Telemetry", success);
                                }
                            }
                        }
                        PacketType::Time => match TimePacketData::read_from(&mut packet.data) {
                            None => {
                                println!("Failed to read {} as time packet!", i);
                            }
                            Some(_p) => {
                                success += 1;
                                //println!("{} - Time", success);
                            }
                        },
                        PacketType::Command => {
                            match CommandPacketData::read_from(&mut packet.data) {
                                None => {
                                    println!("Failed to read {} as command packet!", i);
                                }
                                Some(_p) => {
                                    success += 1;
                                    //println!("{} - Command", success);
                                }
                            }
                        }
                        PacketType::Heartbeat => {
                            match HeartbeatPacketData::read_from(&mut packet.data) {
                                None => {
                                    println!("Failed to read {} as heartbeat packet!", i);
                                }
                                Some(_p) => {
                                    success += 1;
                                    //println!("{} - Heartbeat", success);
                                }
                            }
                        }
                        PacketType::Gamepad => match GamepadPacketData::read_from(&mut packet.data)
                        {
                            None => {
                                println!("Failed to read {} as gamepad packet!", i);
                            }
                            Some(_p) => {
                                success += 1;
                                //println!("{} - Gamepad", success);
                            }
                        },
                    };
                }
            }
        }

        println!(
            "Sucessfully deserialized {} / {}, execution took {:?}",
            success,
            i,
            started.elapsed()
        );
    }
}
