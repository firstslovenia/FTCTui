use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OpModeStatus {
	 #[default]
    Init,
    Running,
    Stopped,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
/// See <https://github.com/acmerobotics/ftc-dashboard/blob/master/DashboardCore/src/main/java/com/acmerobotics/dashboard/RobotStatus.java#L6>
pub struct RobotStatus {
    pub enabled: bool,
    pub available: bool,
    pub active_op_mode: String,
    pub active_op_mode_status: OpModeStatus,
    pub warning_message: String,
    pub error_message: String,
    pub battery_voltage: f64,
}
