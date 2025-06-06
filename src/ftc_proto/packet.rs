use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::traits::{Readable, Writeable};

/// Base structure of all packets
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    pub packet_type: PacketType,
    /// Is there for most packets, expect for Heartbeat
    pub sequence_number: Option<i16>,
    pub data: Vec<u8>,
}

impl Packet {
    /// Creates a packet from the type and data, setting the sequence number to None (as its set
    /// later, when sending)
    pub fn from_packet_type_and_bytes(packet_type: PacketType, data: Vec<u8>) -> Packet {
        Packet {
            sequence_number: None,
            packet_type,
            data,
        }
    }

    /// Creates a packet from the packet type and any writable data
    pub fn from_packet_type_and_writable(packet_type: PacketType, data: &impl Writeable) -> Packet {
        let mut data_buffer = Vec::new();

        data.write_to(&mut data_buffer);

        Self::from_packet_type_and_bytes(packet_type, data_buffer)
    }

    /// Returns self with sequence number set to Some(arg)
    pub fn with_sequence_number(self, sequence_number: i16) -> Self {
        let mut self_a = self;
        self_a.sequence_number = Some(sequence_number);
        self_a
    }
}

impl Writeable for Packet {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        self.packet_type.write_to(buffer);
        (self.data.len() as i16).write_to(buffer);

        if let Some(seq) = self.sequence_number {
            seq.write_to(buffer);
        }

        buffer.append(&mut self.data.clone())
    }
}

impl Readable for Packet {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let packet_type = PacketType::read_from(buffer)?;
        let data_length = i16::read_from(buffer)?;

        let sequence_number = if packet_type != PacketType::Heartbeat {
            Some(i16::read_from(buffer)?)
        } else {
            None
        };

        let data = buffer.drain(..(data_length as usize)).collect();

        Some(Self {
            packet_type,
            sequence_number,
            data,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum PacketType {
    /// Packet periodically pinging the server with one timestamp and the timezone, the server
    /// echoes it back with two more timestamps
    Time = 0x1,

    /// Packet giving the server gamepad data
    Gamepad = 0x2,

    /// Packet used for heartbeating, sent and received every second, contains the sdk versions of
    /// both apps
    Heartbeat = 0x3,

    /// Packet to issue commands from the robot and receive data from it
    Command = 0x4,

    /// Packet giving us opmode telemetry data
    Telemetry = 0x5,
}

impl Writeable for PacketType {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        let as_u8: u8 = (*self).into();

        as_u8.write_to(buffer);
    }
}

impl Readable for PacketType {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self> {
        u8::read_from(buffer)?.try_into().ok()
    }
}
