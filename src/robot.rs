use crate::ftc_proto::{robot_command::OpModeData, time_packet::RobotOpmodeState};

#[derive(Clone, PartialEq, Debug)]
/// Data that we got about the robot itself
pub struct Robot {
    /// The last update we got from the robot
    pub active_opmode_state: Option<RobotOpmodeState>,

    /// The currently active opmode
    pub active_opmode: String,

    // The list of opmodes the robot has
    pub opmode_list: Option<Vec<OpModeData>>,

    /// The last battery voltage we got
    pub battery_voltage: Option<f32>,

    /// When the last battery update was received
    pub last_battery_update: std::time::Instant,

    /// An active system warning message, if any
    pub warning_message: Option<String>,

    /// An active system error message, if any
    pub error_message: Option<String>,

    /// The telemetry display lines last received in a telemetry packet
    pub telemetry_list: Vec<String>,
}
