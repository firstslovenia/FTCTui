
//! Rust mapping of FTC dashboard network messages

use serde::{Deserialize, Serialize};

use super::{gamepad_state::GamepadState, robot_status::RobotStatus, telemetry_packet::TelemetryPacket};

/// Types of messages to send / receive
///
/// See <https://github.com/acmerobotics/ftc-dashboard/blob/master/DashboardCore/src/main/java/com/acmerobotics/dashboard/message/MessageType.java#L21>,
/// <https://github.com/acmerobotics/ftc-dashboard/blob/master/DashboardCore/src/test/java/com/acmerobotics/dashboard/TestDashboardInstance.java#L87> and
/// <https://github.com/acmerobotics/ftc-dashboard/blob/master/FtcDashboard/src/main/java/com/acmerobotics/dashboard/FtcDashboard.java#L609>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum Message {
	/// Sent by the client to receive [Self::ReceiveRobotStatus]
	GetRobotStatus,
	ReceiveRobotStatus(ReceiveRobotStatus),

	InitOpMode(InitOpMode),
	StartOpMode,
	StopOpMode,

	ReceiveOpModeList(ReceiveOpModeList),

	// TODO: Support this
	//GetConfig,
	//ReceiveConfig,
	//SaveConfig,

	ReceiveTelemetry(TelemetryPacket),

	// TODO also support this
	//ReceiveImage,
	ReceiveGamepadState(ReceiveGamepadState)
}

/// Server -> Client packet giving us some info about our robot
///
/// See <https://github.com/acmerobotics/ftc-dashboard/blob/39b55c34e3e814501702fdd8a9710441edaabfad/DashboardCore/src/main/java/com/acmerobotics/dashboard/message/redux/ReceiveRobotStatus.java#L9>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct ReceiveRobotStatus {
	pub status: RobotStatus,
}

/// Client -> Server packet
///
/// See <https://github.com/acmerobotics/ftc-dashboard/blob/master/DashboardCore/src/main/java/com/acmerobotics/dashboard/message/redux/InitOpMode.java#L6>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct InitOpMode {
	pub op_mode_name: String,
}

/// Server -> Client packet giving us a list of opmodes
///
/// See <https://github.com/acmerobotics/ftc-dashboard/blob/master/DashboardCore/src/main/java/com/acmerobotics/dashboard/message/redux/ReceiveOpModeList.java#L7>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct ReceiveOpModeList {
	pub op_mode_list: Vec<String>,
}

/// Server -> Client packet giving us new telemetry packets
///
/// See <https://github.com/acmerobotics/ftc-dashboard/blob/master/DashboardCore/src/main/java/com/acmerobotics/dashboard/message/redux/ReceiveTelemetry.java#L8>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct ReceiveTelemetry {
	/// An empty list tells us to clear telemetry data
	pub telemetry: Vec<TelemetryPacket>,
}

/// Client -> Server packet sending the gamepad state (usually at 60 fps)
///
/// See <https://github.com/acmerobotics/ftc-dashboard/blob/master/DashboardCore/src/main/java/com/acmerobotics/dashboard/message/redux/ReceiveTelemetry.java#L8>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReceiveGamepadState {
	pub gamepad1: GamepadState,
	pub gamepad2: GamepadState,
}
