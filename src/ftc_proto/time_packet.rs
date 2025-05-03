use num_enum::{FromPrimitive, IntoPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::traits::{Readable, Writeable, read_string_from};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Data structure of a Time packet
///
/// If requesting, timestamp should be a very large number in nanoseconds,
/// robot_state should be 0,
/// unix_millis_sent should be the current unix time in milliseconds,
/// unix_millis_received_1 should be 0,
/// unix_millis_received_2 should be 0,
/// timezone should be the device's current timezone
pub struct TimePacketData {
    /// Apparently in nanoseconds (System.nanoTime())
    pub timestamp: u64,
    pub robot_op_mode_state: RobotOpmodeState,
    /// Unix milliseconds epoch of when we sent the request packet
    pub unix_millis_sent: u64,
    pub unix_millis_received_1: u64,
    pub unix_millis_received_2: u64,
    pub timezone: String,
}

impl Writeable for TimePacketData {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        self.timestamp.write_to(buffer);

		  let op_mode_state: i8 = self.robot_op_mode_state.into();

        op_mode_state.write_to(buffer);
        self.unix_millis_sent.write_to(buffer);
        self.unix_millis_received_1.write_to(buffer);
        self.unix_millis_received_2.write_to(buffer);
        (self.timezone.len() as u8).write_to(buffer);
        self.timezone.write_to(buffer);
    }
}

impl Readable for TimePacketData {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let timestamp = u64::read_from(buffer)?;
        let robot_state = i8::read_from(buffer)?;
        let unix_millis_sent = u64::read_from(buffer)?;
        let unix_millis_received_1 = u64::read_from(buffer)?;
        let unix_millis_received_2 = u64::read_from(buffer)?;
        let timezone_length = u8::read_from(buffer)?;
        let timezone = read_string_from(buffer, timezone_length.into())?;

        Some(TimePacketData {
            timestamp,
            robot_op_mode_state: robot_state.into(),
            unix_millis_sent,
            unix_millis_received_1,
            unix_millis_received_2,
            timezone,
        })
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, FromPrimitive, Clone, Copy, Debug)]
#[repr(i8)]
pub enum RobotOpmodeState {
	#[default]
	Unknown = -1,
	NotStarted = 0,
	Initialized = 1,
	Running = 2,
	Stopped = 3,
	EmergencyStopped = 4,
}
