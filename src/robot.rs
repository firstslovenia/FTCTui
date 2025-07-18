use crate::ftc_proto::{
    command_packet::{
        DEFAULT_OPMODE_GROUP, OPMODE_STOP, OpModeData, OpModeFlavor, RobotConfigurationFile,
    },
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
        }
    }
}
