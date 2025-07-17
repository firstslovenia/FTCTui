use serde::{Deserialize, Serialize};

use crate::ftc_proto::hardware::device::DeviceFlavor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Possible XML tags used in the XML document.
///
/// Note that some devices (vendor specific) are also sent over by the server.
pub enum ConfigurationXMLTag {
    Accelerometer,
    AdafruitColorSensor,
    ColorSensor,
    Compass,
    #[serde(rename = "EthernetDevice")]
    EthernetOverUSBDevice,
    Gyroscope,
    #[serde(rename = "IrSeeker")]
    IRSeeker,
    #[serde(rename = "IrSeekerV3")]
    IRSeekerV3,
    LightSensor,
    LynxColorSensor,
    LynxModule,
    #[serde(rename = "LynxUsbDevice")]
    LynxUSBDevice,
    Nothing,
    PulseWidthDevice,
    Robot,
    ServoHub,
    UltrasonicSensor,
    #[serde(rename = "<unknown>")]
    Unknown,
    Webcam,
}

/// The XML robot tag, the root tag of our configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Robot {
    pub lynx_usb_device: Option<LynxUSBDevice>,
    pub webcam: Option<u8>,
    pub ethernet_over_usb_device: Option<u8>,
}

/// Generic data for almost every device in our configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigurationDevice {
    pub name: String,
    pub port: Option<u32>,
    pub device_type: DeviceFlavor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Generic data for almost every device which can have child devices
pub struct ConfigurationController {
    pub device_meta: ConfigurationDevice,
    pub serial_number: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Our Control Hub Portal, parent of [LynxModule] and [ServoHub]
pub struct LynxUSBDevice {
    pub controller_meta: ConfigurationController,
    pub parent_module_address: u32,
    pub lynx_module: Option<LynxModule>,
    pub servo_hub: Option<ServoHub>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Our tag which is actually the parent of our motors, servos, ...
pub struct LynxModule {
    pub controller_meta: ConfigurationController,
    pub servos: Vec<ConfigurationDevice>,
    pub motors: Vec<ConfigurationDevice>,
    pub i2c_devices: Vec<ConfigurationDevice>,
    pub digital_devices: Vec<ConfigurationDevice>,
    pub pwm_outputs: Vec<ConfigurationDevice>,
    pub analog_inputs: Vec<ConfigurationDevice>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// ¯\_(ツ)_/¯
pub struct ServoHub {
    pub controller_meta: ConfigurationController,
    pub servos: Vec<ConfigurationDevice>,
}
