use crate::ftc_dashboard::robot_status::RobotStatus;

#[derive(Clone, PartialEq, Debug)]
/// Data that we got about the robot itself
pub struct Robot {
	/// The last update we got from the robot
	pub status: Option<RobotStatus>,

	// The list of opmodes the robot has
	pub opmode_list: Option<Vec<String>>,

	/// When the last status update was received
	pub last_status_update: std::time::Instant,
}
