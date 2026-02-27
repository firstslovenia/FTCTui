//! Contains types for the CMD_NOTIFY_USER_DEVICE_LIST command,
//! which tells us of every custom XML tag in the robot hardware
//! configuration

use serde::{Deserialize, Serialize};

/// A device as described in JSON in NOTIFY_USER_DEVICE_LIST
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareDeviceType {
    /// Type of device
    pub flavor: DeviceFlavor,
    pub xml_tag: String,

    pub name: String,

    #[serde(default)]
    pub built_in: bool,

    #[serde(default)]
    pub is_deprecated: bool,

    #[serde(default)]
    pub is_external_libraries: bool,

    #[serde(default)]
    pub is_on_bot_java: bool,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub xml_tag_aliases: Vec<String>,

    // Class source and control system and not really needed
    #[serde(default)]
    pub motor_extras: Option<MotorDeviceExtraFields>,

    #[serde(default)]
    pub servo_extras: Option<ServoDeviceExtraFields>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceFlavor {
    AnalogOutput,
    AnalogSensor,
    BuiltIn,

    #[serde(rename = "DIGITAL_IO")]
    DigitalIO,

    #[serde(rename = "ETHERNET_OVER_USB")]
    EthernetOverUSB,

    #[serde(rename = "I2C")]
    I2C,
    Motor,
    Servo,
}

/// Extra fields for a motor configuration device
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MotorDeviceExtraFields {
    #[serde(rename = "maxRPM")]
    pub max_revolutions_per_minute: f64,

    #[serde(rename = "ticksPerRev")]
    pub ticks_per_revolution: f64,
}

/// Extra fields for a servo configuration device
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServoDeviceExtraFields {
    #[serde(rename = "servoFlavor")]
    pub flavor: ServoFlavor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServoFlavor {
    Standard,
    Continuous,
    Custom,
}
