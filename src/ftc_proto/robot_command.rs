use serde::{Deserialize, Serialize};

use super::traits::{Readable, Writeable, read_string_from};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Data structure of a Robot Command Packet
pub struct CommandPacketData {
    /// Apparently in nanoseconds (System.nanoTime())
    ///
    /// eg. 9336809990439
    pub timestamp: u64,

    /// If set to true, we are just acknowledging a command sent by the other party, not sending
    /// one ourselves. In such a case, we don't send any command data.
    pub acknowledged: bool,

    pub command: String,
    pub data: String,
}

impl Writeable for CommandPacketData {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        self.timestamp.write_to(buffer);
        self.acknowledged.write_to(buffer);
        (self.command.len() as u16).write_to(buffer);
        self.command.write_to(buffer);

        if !self.acknowledged {
            (self.data.len() as u16).write_to(buffer);

            if !self.data.is_empty() {
                self.data.write_to(buffer);
            }
        }
    }
}

impl Readable for CommandPacketData {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let timestamp = u64::read_from(buffer)?;
        let acknowledged = bool::read_from(buffer)?;
        let command_len = u16::read_from(buffer)?;
        let command = read_string_from(buffer, command_len as usize)?;

        let mut data = String::new();

        if !acknowledged {
            let data_len = u16::read_from(buffer)?;
            data = read_string_from(buffer, data_len as usize)?;
        }

        Some(CommandPacketData {
            timestamp,
            acknowledged,
            command,
            data,
        })
    }
}

/// Client to server
pub const RESTART_ROBOT: &str = "CMD_RESTART_ROBOT";

/// Data is stringified i32 match number
///
/// Client to server
pub const SET_MATCH_NUMBER: &str = "CMD_SET_MATCH_NUMBER";

/// Data is stringified integer representation of RobotOpModeState
///
/// Server to client
pub const NOTIFY_OP_MODE_STATE: &str = "CMD_NOTIFY_ROBOT_STATE";

/// Client to server
pub const REQUEST_OP_MODES: &str = "CMD_REQUEST_OP_MODE_LIST";

/// Data is a json list of OpModeData
///
/// Server to client
pub const NOTIFY_OP_MODES: &str = "CMD_NOTIFY_OP_MODE_LIST";

/// Data is a string name of a configuration
///
/// Client to server
pub const ACTIVATE_CONFIGURATION: &str = "CMD_ACTIVATE_CONFIGURATION";

/// Client to server
pub const REQUEST_ACTIVE_CONFIGURATION: &str = "CMD_REQUEST_ACTIVE_CONFIG";

/// Data is json RobotConfiguration
///
/// Server to client
pub const NOTIFY_ACTIVE_CONFIGURATION: &str = "CMD_NOTIFY_ACTIVE_CONFIGURATION";

/// Data is a string name of an opmode
///
/// To stop the running opmode, send this with $Stop$Robot$
///
/// Client to server
pub const INIT_OPMODE: &str = "CMD_INIT_OP_MODE";

/// Data is a string name of an opmode
///
/// Client to server
pub const RUN_OPMODE: &str = "CMD_RUN_OP_MODE";

/// Data is a string name of an opmode
///
/// Server to client
pub const NOTIFY_INIT_OPMODE: &str = "CMD_NOTIFY_INIT_OP_MODE";

/// Data is a string name of an opmode
///
/// Server to client
pub const NOTIFY_RUN_OPMODE: &str = "CMD_NOTIFY_RUN_OP_MODE";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
/// Data about a particular opmode, sent in notify opmode list
pub struct OpModeData {
    /// Type of opmode. System opmodes cannot be user defined, and an example of one is
    /// "$Stop$Robot$", which means there is no running opmode.
    pub flavor: OpModeFlavor,

    /// Name of the group the opmode belongs to. "$$$$$$$" is the default group
    pub group: String,

    /// Name of the opmode
    pub name: String,

    /// External source of the opmode, if any
    pub source: Option<OpModeSource>,

    #[serde(rename = "systemOpModeBaseDisplayName")]
    /// User friendly display name for system opmodes
    pub system_opmode_display_name: Option<String>,
}

pub const DEFAULT_OPMODE_GROUP: &str = "$$$$$$$";
pub const OPMODE_STOP: &str = "$Stop$Robot$";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// What type of opmode it is
pub enum OpModeFlavor {
    Autonomous,
    System,
    Teleop,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Optional external source of the opmode
pub enum OpModeSource {
    AndroidStudio,
    Blockly,
    ExternalLibrary,

    #[serde(rename = "ONBOTJAVA")]
    OnbotJava,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
/// Data about a particular configuration (file)
pub struct RobotConfiguration {
    /// Unsure what this means
    pub is_dirty: bool,

    /// Where the configuration is stored
    pub location: ConfigurationLocation,

    /// Name of the configuration
    pub name: String,

    /// 0 by default, likely set to some value if location is Resource
    pub resource_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Where the configuration is stored
pub enum ConfigurationLocation {
    None,
    LocalStorage,
    Resource,
}
