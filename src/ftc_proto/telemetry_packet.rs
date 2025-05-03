use super::{
    time_packet::RobotOpmodeState,
    traits::{Readable, Writeable, read_string_from},
};

pub const ROBOT_BATTERY_LEVEL_KEY: &str = "$Robot$Battery$Level$";
pub const ROBOT_CONTROLLER_BATTERY_STATUS_KEY: &str = "$RobotController$Battery$Status$";

pub const SYSTEM_WARNING_KEY: &str = "$System$Warning$";
pub const SYSTEM_ERROR_KEY: &str = "$System$Error$";
pub const SYSTEM_NONE_KEY: &str = "$System$None$";

pub struct TelemetryPacket {
    /// Unix epoch in milliseconds of when packet was sent
    pub unix_timestamp_millis: i64,

    /// Set to true if telemetry map entries are sorted
    pub is_sorted: bool,

    /// Current state of the robot / active opmode
    pub robot_state: RobotOpmodeState,

    /// Optional tag, can be empty string
    pub tag: String,

    pub string_entries: Vec<TelemetryEntry>,
    pub float_entries: Vec<FloatEntry>,
}

impl Readable for TelemetryPacket {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let unix_timestamp_millis = i64::read_from(buffer)?;
        let is_sorted = bool::read_from(buffer)?;
        let robot_state: RobotOpmodeState = i8::read_from(buffer)?.into();

        let tag_length = u8::read_from(buffer)?;
        let tag = read_string_from(buffer, tag_length as usize)?;

        let number_of_string_entries = u8::read_from(buffer)?;
        let mut string_entries = Vec::new();

        for _i in 0..number_of_string_entries {
            string_entries.push(TelemetryEntry::read_from(buffer)?);
        }

        let number_of_float_entries = u8::read_from(buffer)?;
        let mut float_entries = Vec::new();

        for _i in 0..number_of_float_entries {
            float_entries.push(FloatEntry::read_from(buffer)?);
        }

        Some(TelemetryPacket {
            unix_timestamp_millis,
            is_sorted,
            robot_state,
            tag,
            string_entries,
            float_entries,
        })
    }
}

impl Writeable for TelemetryPacket {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        self.unix_timestamp_millis.write_to(buffer);
        self.is_sorted.write_to(buffer);

        let robot_state: i8 = self.robot_state.into();
        robot_state.write_to(buffer);

        (self.tag.len() as u8).write_to(buffer);
        self.tag.write_to(buffer);

        (self.string_entries.len() as u8).write_to(buffer);

        for entry in &self.string_entries {
            entry.write_to(buffer);
        }

        (self.float_entries.len() as u8).write_to(buffer);

        for entry in &self.float_entries {
            entry.write_to(buffer);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TelemetryEntry {
    pub key: String,

    /// Note: can be empty string
    pub value: String,
}

impl Readable for TelemetryEntry {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let key_length = u16::read_from(buffer)?;

        let key = read_string_from(buffer, key_length as usize)?;

        let value_length = u16::read_from(buffer)?;

        let value = read_string_from(buffer, value_length as usize)?;

        Some(TelemetryEntry { key, value })
    }
}

impl Writeable for TelemetryEntry {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        (self.key.len() as u16).write_to(buffer);
        self.key.write_to(buffer);

        (self.value.len() as u16).write_to(buffer);
        self.value.write_to(buffer);
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FloatEntry {
    pub key: String,

    pub value: f32,
}

impl Readable for FloatEntry {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let key_length = u16::read_from(buffer)?;

        let key = read_string_from(buffer, key_length as usize)?;

        let value = f32::read_from(buffer)?;

        Some(FloatEntry { key, value })
    }
}

impl Writeable for FloatEntry {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        (self.key.len() as u16).write_to(buffer);
        self.key.write_to(buffer);
        self.value.write_to(buffer);
    }
}
