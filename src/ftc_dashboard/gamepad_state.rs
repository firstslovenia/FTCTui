use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
/// See <https://github.com/acmerobotics/ftc-dashboard/blob/39b55c34e3e814501702fdd8a9710441edaabfad/DashboardCore/src/main/java/com/acmerobotics/dashboard/message/redux/ReceiveGamepadState.java#L6>
pub struct GamepadState {
	pub left_stick_x: f32,
	pub left_stick_y: f32,
	pub left_stick_button: bool,

	pub right_stick_x: f32,
	pub right_stick_y: f32,
	pub right_stick_button: bool,

	pub dpad_up: bool,
	pub dpad_down: bool,
	pub dpad_left: bool,
	pub dpad_right: bool,

	pub a: bool,
	pub b: bool,
	pub x: bool,
	pub y: bool,

	pub guide: bool,
	pub start: bool,
	pub back: bool,

	pub left_bumper: bool,
	pub right_bumper: bool,

	pub left_trigger: f32,
	pub right_trigger: f32,

	#[serde(skip_serializing)]
	pub touchpad: bool,
}
