use crate::ftc_proto::{
    self,
    command_packet::{
        DEFAULT_OPMODE_GROUP, OPMODE_STOP, OpModeData, OpModeFlavor, RobotConfigurationFile,
    },
    hardware::device::DeviceFlavor,
    hardware::device::HardwareDeviceType,
    time_packet::RobotOpmodeState,
};

#[derive(Clone, PartialEq, Debug)]
/// Data that we got about the robot itself
pub struct Robot {
    /// The last update we got from the robot
    pub active_opmode_state: Option<RobotOpmodeState>,

    /// The currently active opmode
    pub active_opmode: String,

    // The list of opmodes the robot has
    pub opmode_list: Option<Vec<OpModeData>>,

    /// The active configuration, if we've received one
    pub active_configuration: Option<RobotConfigurationFile>,

    /// The valid configuration types, if we're received them from the server
    pub configuration_types: Option<Vec<HardwareDeviceType>>,

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

impl Robot {
    /// Creates a new real robot status, that has no info in it yet
    pub fn new_empty() -> Robot {
        Robot {
            active_opmode_state: None,
            active_opmode: OPMODE_STOP.to_string(),
            opmode_list: None,
            active_configuration: None,
            battery_voltage: None,
            last_battery_update: std::time::Instant::now(),
            warning_message: None,
            error_message: None,
            telemetry_list: Vec::new(),
            configuration_types: None,
        }
    }

    /// Creates a new fake robot status, to test the renderers
    pub fn new_fake() -> Robot {
        Robot {
            active_opmode_state: Some(RobotOpmodeState::Running),
            opmode_list: Some(vec![
                OpModeData {
                    name: OPMODE_STOP.to_string(),
                    group: DEFAULT_OPMODE_GROUP.to_string(),
                    flavor: OpModeFlavor::System,
                    source: None,
                    system_opmode_display_name: None,
                },
                OpModeData {
                    name: "Robot".to_string(),
                    group: DEFAULT_OPMODE_GROUP.to_string(),
                    flavor: OpModeFlavor::Teleop,
                    source: None,
                    system_opmode_display_name: None,
                },
                OpModeData {
                    name: "CoolerRobot".to_string(),
                    group: DEFAULT_OPMODE_GROUP.to_string(),
                    flavor: OpModeFlavor::Teleop,
                    source: None,
                    system_opmode_display_name: None,
                },
                OpModeData {
                    name: "Autonomous".to_string(),
                    group: "jože".to_string(),
                    flavor: OpModeFlavor::Autonomous,
                    source: None,
                    system_opmode_display_name: None,
                },
                OpModeData {
                    name: "CoolerAutonomous".to_string(),
                    group: "jože".to_string(),
                    flavor: OpModeFlavor::Autonomous,
                    source: None,
                    system_opmode_display_name: None,
                },
                OpModeData {
                    name: "Bautonomous".to_string(),
                    group: DEFAULT_OPMODE_GROUP.to_string(),
                    flavor: OpModeFlavor::Autonomous,
                    source: None,
                    system_opmode_display_name: None,
                },
            ]),
            active_opmode: "Bautonomous".to_string(),
            active_configuration: None,
            error_message: None,
            warning_message: Some(String::from("Test warning message")),
            battery_voltage: Some(12.3),
            last_battery_update: std::time::Instant::now(),
            telemetry_list: vec![
                "leftY : 0".to_string(),
                "leftX : 0".to_string(),
                "rightX : 0".to_string(),
                "left front : -667".to_string(),
                "left back : 0".to_string(),
                "right front : -737".to_string(),
                "right back : 310".to_string(),
                "rokaLeft : -67".to_string(),
                "rokaRight : 67".to_string(),
                "wanted location before : 0".to_string(),
                "power to set : 0".to_string(),
                "left position : 982".to_string(),
                "lifterLeft : 982".to_string(),
                "lifterRight : 983".to_string(),
                "wantedlocation : 984.428".to_string(),
                "L:  : 0".to_string(),
                "D:  : -0".to_string(),
                "claw wanted position : 1".to_string(),
                "curent position : 1".to_string(),
            ],
            configuration_types: Some(vec![HardwareDeviceType { flavor: DeviceFlavor::AnalogSensor, xml_tag: "AnalogInput".to_string(), name: "Analog Input".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "AdafruitBNO055IMU".to_string(), name: "Adafruit IMU".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "an Adafruit IMU".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "LynxColorSensor".to_string(), name: "REV Color/Range Sensor".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "ModernRoboticsI2cRangeSensor".to_string(), name: "MR Range Sensor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "a MR range sensor".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "Compass".to_string(), name: "[Unknown]".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "PulseWidthDevice".to_string(), name: "Pulse Width Device".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::DigitalIO, xml_tag: "DigitalDevice".to_string(), name: "Digital Device".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "NeveRest40Gearmotor".to_string(), name: "NeveRest 40 Gearmotor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "MaxSonarI2CXL".to_string(), name: "MaxSonar I2CXL".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::AnalogSensor, xml_tag: "OpticalDistanceSensor".to_string(), name: "MR Optical Distance Sensor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "a Modern Robotics optical distance sensor".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Servo, xml_tag: "RevSPARKMini".to_string(), name: "REV SPARKmini Controller".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "a REV SPARKmini Motor Controller".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "NeveRest20Gearmotor".to_string(), name: "NeveRest 20 Gearmotor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "LynxEmbeddedIMU".to_string(), name: "REV internal IMU (BNO055)".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "an embedded IMU".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "NeveRest3.7v1Gearmotor".to_string(), name: "NeveRest 3.7 v1 Gearmotor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "OctoQuadFTC".to_string(), name: "OctoQuadFTC".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "KauaiLabsNavxMicro".to_string(), name: "navX Micro".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "a navX Micro gyro".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Servo, xml_tag: "ContinuousRotationServo".to_string(), name: "Continuous Rotation Servo".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "IrSeekerV3".to_string(), name: "MR IR Seeker v3".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::AnalogSensor, xml_tag: "ModernRoboticsAnalogTouchSensor".to_string(), name: "MR Touch Sensor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "a Modern Robotics touch sensor".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "goBILDA5201SeriesMotor".to_string(), name: "GoBILDA 5201 series".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "goBILDA5202SeriesMotor".to_string(), name: "GoBILDA 5202/3/4 series".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "RevColorSensorV3".to_string(), name: "REV Color Sensor V3".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "a REV Color Sensor V3".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "TetrixMotor".to_string(), name: "Tetrix Motor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "SparkFunOTOS".to_string(), name: "SparkFun OTOS".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "SparkFun Qwiic Optical Tracking Odometry Sensor".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "Matrix12vMotor".to_string(), name: "Matrix 12v Motor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "Accelerometer".to_string(), name: "[Unknown]".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "Webcam".to_string(), name: "Webcam".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "ControlHubImuBHI260AP".to_string(), name: "REV internal IMU (BHI260AP)".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "an embedded IMU".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::DigitalIO, xml_tag: "Led".to_string(), name: "LED".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "an LED".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "Gyro".to_string(), name: "MR Gyro".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "IrSeeker".to_string(), name: "[Unknown]".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "LynxModule".to_string(), name: "Expansion Hub".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "AdafruitColorSensor".to_string(), name: "Adafruit Color Sensor".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Servo, xml_tag: "Servo".to_string(), name: "Servo".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "ModernRoboticsI2cCompassSensor".to_string(), name: "MR Compass Sensor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "a MR compass sensor".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "RevExternalImu".to_string(), name: "REV 9-Axis IMU".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "a REV 9-Axis IMU".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "StudicaMaverick".to_string(), name: "Studica Maverick".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "AndyMarkIMU".to_string(), name: "AndyMark 9-Axis IMU".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "AndyMark External IMU".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "AndyMarkTOF".to_string(), name: "AndyMark TOF Lidar".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "AndyMark 2m TOF Lidar".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "Robot".to_string(), name: "[Unknown]".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Servo, xml_tag: "ServoFullRange".to_string(), name: "Full Range Servo".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "RevRoboticsUltraplanetaryHDHexMotor".to_string(), name: "REV Robotics UltraPlanetary HD Hex Motor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "QWIIC_LED_STICK".to_string(), name: "SparkFun QWIIC LED Stick".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "a SparkFun QWIIC LED Stick".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "<unknown>".to_string(), name: "[Unknown]".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "LightSensor".to_string(), name: "[Unknown]".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "ColorSensor".to_string(), name: "MR Color Sensor".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "RevRobotics40HDHexMotor".to_string(), name: "REV Robotics 40:1 HD Hex Motor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec!["RevRoboticsHDHexMotor".to_string()], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Servo, xml_tag: "RevBlinkinLedDriver".to_string(), name: "REV Blinkin LED Driver".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "a REV Blinkin LED Driver".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "RevRoboticsCoreHexMotor".to_string(), name: "REV Robotics Core Hex Motor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "goBILDAPinpoint".to_string(), name: "goBILDA® Pinpoint Odometry Computer".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "goBILDA® Pinpoint Odometry Computer (IMU Sensor Fusion for 2 Wheel Odometry)".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "REV_VL53L0X_RANGE_SENSOR".to_string(), name: "REV 2M Distance Sensor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "REV 2M Distance Sensor".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "Motor".to_string(), name: "Unspecified Motor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "Nothing".to_string(), name: "Nothing".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::DigitalIO, xml_tag: "RevTouchSensor".to_string(), name: "REV Touch Sensor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "a REV touch sensor".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "LynxUsbDevice".to_string(), name: "Expansion Hub Portal".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "AndyMarkColor".to_string(), name: "AndyMark Proximity & Color Sensor".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "AndyMark Proximity & Color Sensor".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::I2C, xml_tag: "HuskyLens".to_string(), name: "HuskyLens".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "HuskyLens Vision Sensor".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "ServoHub".to_string(), name: "[Unknown]".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "UltrasonicSensor".to_string(), name: "[Unknown]".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::BuiltIn, xml_tag: "EthernetDevice".to_string(), name: "[Unknown]".to_string(), built_in: false, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "NeveRest60Gearmotor".to_string(), name: "NeveRest 60 Gearmotor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }, HardwareDeviceType { flavor: DeviceFlavor::Motor, xml_tag: "RevRobotics20HDHexMotor".to_string(), name: "REV Robotics 20:1 HD Hex Motor".to_string(), built_in: true, is_deprecated: false, is_external_libraries: false, is_on_bot_java: false, description: "".to_string(), xml_tag_aliases: vec![], motor_extras: None, servo_extras: None }])}
    }
}
