use super::traits::{Readable, Writeable};

pub const HEARTBEAT_SEQUENCE_NUMBER_REQUEST: i16 = 10003;
pub const HEARTBEAT_SEQUENCE_NUMBER_RESPONSE: i16 = 7;

#[derive(Debug, Clone, PartialEq, Copy)]
/// Data structure of a Heartbeat packet
///
/// This packet is only sent from the client to the server, about 25x per second
pub struct HeartbeatPacketData {
    /// Usually 1 ([PEER_TYPE_PEER])
    pub peer_type: i8,

    /// Always 10003 for requests, always 7 for responses
    pub sequence_number: i16,

    pub sdk_build_month: i8,
    pub sdk_build_year: i16,

    pub sdk_major_version: i8,
    pub sdk_minor_version: i8,
}

impl Writeable for HeartbeatPacketData {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        124_u8.write_to(buffer);

        self.peer_type.write_to(buffer);
        self.sequence_number.write_to(buffer);

        self.sdk_build_month.write_to(buffer);
        self.sdk_build_year.write_to(buffer);

        self.sdk_major_version.write_to(buffer);
        self.sdk_minor_version.write_to(buffer);

        0_u8.write_to(buffer);
    }
}

impl Readable for HeartbeatPacketData {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let _124 = u8::read_from(buffer)?;

        let peer_type = i8::read_from(buffer)?;
        let sequence_number = i16::read_from(buffer)?;

        let sdk_build_month = i8::read_from(buffer)?;
        let sdk_build_year = i16::read_from(buffer)?;

        let sdk_major_version = i8::read_from(buffer)?;
        let sdk_minor_version = i8::read_from(buffer)?;

        let _0 = u8::read_from(buffer)?;

        Some(HeartbeatPacketData {
            peer_type,
            sequence_number,
            sdk_build_month,
            sdk_build_year,
            sdk_major_version,
            sdk_minor_version,
        })
    }
}

pub const PEER_TYPE_NOT_SET: i8 = 0;
pub const PEER_TYPE_PEER: i8 = 1;
pub const PEER_TYPE_GROUP_OWNER: i8 = 2;
pub const PEER_TYPE_NOT_CONNECTED_DUE_TO_PREEXISTING_CONNECTION: i8 = 3;
