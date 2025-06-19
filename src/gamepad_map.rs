//! Provides proper mapping for REV's PS4 controller
//!
//! This is reliant on gilrs_core codes, which are different for each platform.

use gilrs::{Axis, Button};

pub static REV_CONTROLLER_CUSTOM_SDL_MAPPING: &str = "0300d388120c0000182e000011010000,REV FTC Gamepad,a:b1,b:b2,x:b0,y:b3,back:b8,guide:b12,start:b9,leftstick:b10,rightstick:b11,leftshoulder:b4,rightshoulder:b5,dpup:h0.1,dpdown:h0.4,dpleft:h0.8,dpright:h0.2,leftx:a0,lefty:a1,rightx:a2,righty:a5,lefttrigger:a3,righttrigger:a4,platform:Linux,crc:88d3,";

/// Custom mapping name for the REV PS4 controller
pub static CUSTOM_MAP_NAME: &str = "REV Robotics FTC Controller";

pub static UNKNOWN_CODE: &str = "UNKNOWN";

pub static BUTTON_MAP: [(&str, Button); 13] = [
    (REV_MAP_CODE_LEFT_STICK_BUTTON, Button::LeftThumb),
    (REV_MAP_CODE_RIGHT_STICK_BUTTON, Button::RightThumb),
    (REV_MAP_CODE_MODE_BUTTON, Button::Mode),
    (REV_MAP_CODE_START_BUTTON, Button::Start),
    (REV_MAP_CODE_SELECT_BUTTON, Button::Select),
    (REV_MAP_CODE_NORTH_BUTTON, Button::North),
    (REV_MAP_CODE_SOUTH_BUTTON, Button::South),
    (REV_MAP_CODE_WEST_BUTTON, Button::West),
    (REV_MAP_CODE_EAST_BUTTON, Button::East),
    (REV_MAP_CODE_LEFT_BUMPER_BUTTON, Button::LeftTrigger),
    (REV_MAP_CODE_RIGHT_BUMPER_BUTTON, Button::RightTrigger),
    (REV_MAP_CODE_LEFT_TRIGGER_BUTTON, Button::LeftTrigger2),
    (REV_MAP_CODE_RIGHT_TRIGGER_BUTTON, Button::RightTrigger2),
];

pub static AXIS_MAP: [(&str, Axis); 6] = [
    (REV_MAP_CODE_LEFT_STICK_X, Axis::LeftStickX),
    (REV_MAP_CODE_LEFT_STICK_Y, Axis::LeftStickY),
    (REV_MAP_CODE_RIGHT_STICK_X, Axis::RightStickX),
    (REV_MAP_CODE_RIGHT_STICK_Y, Axis::RightStickY),
    (REV_MAP_CODE_LEFT_TRIGGER_AXIS, Axis::LeftZ),
    (REV_MAP_CODE_RIGHT_TRIGGER_AXIS, Axis::RightZ),
    // Note: see https://gitlab.com/gilrs-project/gilrs/-/issues/186
    //(REV_MAP_CODE_DPAD_X_AXIS, Axis::DPadX),
    //(REV_MAP_CODE_DPAD_Y_AXIS, Axis::DPadY),
];

// Custom mapping codes for each platform for the REV controller
#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_LEFT_STICK_X: &str = r#"{"kind":3,"code":0}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_LEFT_STICK_X: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_LEFT_STICK_X: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_LEFT_STICK_Y: &str = r#"{"kind":3,"code":1}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_LEFT_STICK_Y: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_LEFT_STICK_Y: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_LEFT_STICK_BUTTON: &str = r#"{"kind":1,"code":314}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_LEFT_STICK_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_LEFT_STICK_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_RIGHT_STICK_X: &str = r#"{"kind":3,"code":2}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_RIGHT_STICK_X: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_RIGHT_STICK_X: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_RIGHT_STICK_Y: &str = r#"{"kind":3,"code":5}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_RIGHT_STICK_Y: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_RIGHT_STICK_Y: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_RIGHT_STICK_BUTTON: &str = r#"{"kind":1,"code":315}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_RIGHT_STICK_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_RIGHT_STICK_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_MODE_BUTTON: &str = r#"{"kind":1,"code":316}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_MODE_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_MODE_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_START_BUTTON: &str = r#"{"kind":1,"code":312}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_START_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_START_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_SELECT_BUTTON: &str = r#"{"kind":1,"code":313}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_SELECT_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_SELECT_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_DPAD_X_AXIS: &str = r#"{"kind":3,"code":16}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_DPAD_X_AXIS: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_DPAD_X_AXIS: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_DPAD_Y_AXIS: &str = r#"{"kind":3,"code":17}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_DPAD_Y_AXIS: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_DPAD_Y_AXIS: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_NORTH_BUTTON: &str = r#"{"kind":1,"code":307}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_NORTH_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_NORTH_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_SOUTH_BUTTON: &str = r#"{"kind":1,"code":305}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_SOUTH_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_SOUTH_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_WEST_BUTTON: &str = r#"{"kind":1,"code":304}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_WEST_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_WEST_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_EAST_BUTTON: &str = r#"{"kind":1,"code":306}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_EAST_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_EAST_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_LEFT_BUMPER_BUTTON: &str = r#"{"kind":1,"code":308}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_LEFT_BUMPER_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_LEFT_BUMPER_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_RIGHT_BUMPER_BUTTON: &str = r#"{"kind":1,"code":309}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_RIGHT_BUMPER_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_RIGHT_BUMPER_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_LEFT_TRIGGER_AXIS: &str = r#"{"kind":3,"code":3}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_LEFT_TRIGGER_AXIS: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_LEFT_TRIGGER_AXIS: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_LEFT_TRIGGER_BUTTON: &str = r#"{"kind":1,"code":310}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_LEFT_TRIGGER_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_LEFT_TRIGGER_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_RIGHT_TRIGGER_AXIS: &str = r#"{"kind":3,"code":4}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_RIGHT_TRIGGER_AXIS: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_RIGHT_TRIGGER_AXIS: &str = UNKNOWN_CODE;

#[cfg(target_os = "linux")]
pub static REV_MAP_CODE_RIGHT_TRIGGER_BUTTON: &str = r#"{"kind":1,"code":311}"#;

#[cfg(target_os = "windows")]
pub static REV_MAP_CODE_RIGHT_TRIGGER_BUTTON: &str = UNKNOWN_CODE;

#[cfg(target_os = "macos")]
pub static REV_MAP_CODE_RIGHT_TRIGGER_BUTTON: &str = UNKNOWN_CODE;
