use super::traits::{Readable, Writeable};
use bitflags::bitflags;

#[derive(Debug, Clone, PartialEq, Copy, Default)]
/// Data structure of a Gamepad packet
///
/// This packet is only sent from the client to the server, about 25x per second
pub struct GamepadPacketData {
    /// eg. 2002
    pub gamepad_id: i32,

    /// millis?
    /// eg. 3790115
    pub timestamp: u64,

    pub left_stick_x: f32,
    pub left_stick_y: f32,

    pub right_stick_x: f32,
    pub right_stick_y: f32,

    pub left_trigger: f32,
    pub right_trigger: f32,

    /// See [ButtonFlags]
    pub button_flags: u32,

    /// 1 / 2
    pub user: u8,

    pub legacy_type: u8,
    pub gamepad_type: u8,

    pub touchpad_finger_1_x: f32,
    pub touchpad_finger_1_y: f32,
    pub touchpad_finger_2_x: f32,
    pub touchpad_finger_2_y: f32,
}

impl GamepadPacketData {
    pub fn default_for_user(user: u8) -> GamepadPacketData {
        GamepadPacketData {
            // Note: not sure if this is what we can do, but eh smeh bleh
            gamepad_id: 2002,
            timestamp: 0,
            left_stick_x: 0.0,
            left_stick_y: 0.0,
            right_stick_x: 0.0,
            right_stick_y: 0.0,
            left_trigger: 0.0,
            right_trigger: 0.0,
            button_flags: ButtonFlags::empty().bits(),
            user,
            legacy_type: GAMEPAD_TYPE_XBOX_360,
            gamepad_type: GAMEPAD_TYPE_XBOX_360,
            touchpad_finger_1_x: 0.0,
            touchpad_finger_1_y: 0.0,
            touchpad_finger_2_x: 0.0,
            touchpad_finger_2_y: 0.0,
        }
    }
}

impl Writeable for GamepadPacketData {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        5_u8.write_to(buffer);
        self.gamepad_id.write_to(buffer);
        self.timestamp.write_to(buffer);

        self.left_stick_x.write_to(buffer);
        self.left_stick_y.write_to(buffer);

        self.right_stick_x.write_to(buffer);
        self.right_stick_y.write_to(buffer);

        self.left_trigger.write_to(buffer);
        self.right_trigger.write_to(buffer);

        self.button_flags.write_to(buffer);

        self.user.write_to(buffer);
        self.legacy_type.write_to(buffer);
        self.gamepad_type.write_to(buffer);

        self.touchpad_finger_1_x.write_to(buffer);
        self.touchpad_finger_1_y.write_to(buffer);
        self.touchpad_finger_2_x.write_to(buffer);
        self.touchpad_finger_2_y.write_to(buffer);
    }
}

impl Readable for GamepadPacketData {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        let _five = u8::read_from(buffer)?;
        let gamepad_id = i32::read_from(buffer)?;
        let timestamp = u64::read_from(buffer)?;

        let left_stick_x = f32::read_from(buffer)?;
        let left_stick_y = f32::read_from(buffer)?;

        let right_stick_x = f32::read_from(buffer)?;
        let right_stick_y = f32::read_from(buffer)?;

        let left_trigger = f32::read_from(buffer)?;
        let right_trigger = f32::read_from(buffer)?;

        let button_flags = u32::read_from(buffer)?;

        let user = u8::read_from(buffer)?;
        let legacy_type = u8::read_from(buffer)?;
        let gamepad_type = u8::read_from(buffer)?;

        let touchpad_finger_1_x = f32::read_from(buffer)?;
        let touchpad_finger_1_y = f32::read_from(buffer)?;
        let touchpad_finger_2_x = f32::read_from(buffer)?;
        let touchpad_finger_2_y = f32::read_from(buffer)?;

        Some(GamepadPacketData {
            gamepad_id,
            timestamp,
            left_stick_x,
            left_stick_y,
            right_stick_x,
            right_stick_y,
            left_trigger,
            right_trigger,
            button_flags,
            user,
            legacy_type,
            gamepad_type,
            touchpad_finger_1_x,
            touchpad_finger_1_y,
            touchpad_finger_2_x,
            touchpad_finger_2_y,
        })
    }
}

pub const GAMEPAD_TYPE_UNKNOWN: u8 = 0;
pub const GAMEPAD_TYPE_LOGITECH_F310: u8 = 1;
pub const GAMEPAD_TYPE_XBOX_360: u8 = 2;
pub const GAMEPAD_TYPE_SONY_PS4: u8 = 3;
pub const GAMEPAD_TYPE_SONY_PS4_SUPPORTED_BY_KERNEL: u8 = 4;

bitflags! {
     /// Bitflags representing which buttons are pressed
     #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct ButtonFlags: u32 {
          const RIGHT_BUMPER = 1;
          const LEFT_BUMPER = 1 << 1;

          const BACK = 1 << 2;
          const START = 1 << 3;
          const GUIDE = 1 << 4;

          const Y = 1 << 5;
          const X = 1 << 6;
          const B = 1 << 7;
          const A = 1 << 8;

          const DPAD_RIGHT = 1 << 9;
          const DPAD_LEFT = 1 << 10;
          const DPAD_DOWN = 1 << 11;
          const DPAD_UP = 1 << 12;

          const RIGHT_STICK_BUTTON = 1 << 13;
          const LEFT_STICK_BUTTON = 1 << 14;

          const TOUCHPAD = 1 << 15;
          const TOUCHPAD_FINGER_2 = 1 << 16;
          const TOUCHPAD_FINGER_1 = 1 << 17;
    }
}
