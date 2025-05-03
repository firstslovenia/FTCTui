/// A trait for objects that can be written to a byte buffer
pub trait Writeable {
    /// Appends self in bytes to the buffer
    fn write_to(&self, buffer: &mut Vec<u8>);
}

/// A trait for objects that have a fixed size byte representation
pub trait Sizeable {
    /// Returns the written size of the Writeable struct, used for prefixing the size
    /// before the type's data
    fn get_size_bytes() -> u16;
}

/// A trait for objects that can be read from a byte buffer
pub trait Readable {
    /// Attempts to read self from the start of the buffer, removing the bytes.
    ///
    /// Returns [None] if the buffer is not long enough or the read otherwise failed.
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized;
}

impl Writeable for u8 {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.push(*self);
    }
}

impl Sizeable for u8 {
    fn get_size_bytes() -> u16 {
        1
    }
}

impl Readable for u8 {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::get_size_bytes().into() {
            return None;
        }

        Some(buffer.remove(0))
    }
}

impl Writeable for u16 {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut self.to_be_bytes().to_vec());
    }
}

impl Sizeable for u16 {
    fn get_size_bytes() -> u16 {
        2
    }
}

impl Readable for u16 {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::get_size_bytes().into() {
            return None;
        }

        Some(Self::from_be_bytes(
            buffer
                .drain(..(Self::get_size_bytes() as usize))
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        ))
    }
}

impl Writeable for u32 {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut self.to_be_bytes().to_vec());
    }
}

impl Sizeable for u32 {
    fn get_size_bytes() -> u16 {
        4
    }
}

impl Readable for u32 {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::get_size_bytes().into() {
            return None;
        }

        Some(Self::from_be_bytes(
            buffer
                .drain(..(Self::get_size_bytes() as usize))
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        ))
    }
}

impl Writeable for u64 {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut self.to_be_bytes().to_vec());
    }
}

impl Sizeable for u64 {
    fn get_size_bytes() -> u16 {
        8
    }
}

impl Readable for u64 {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::get_size_bytes().into() {
            return None;
        }

        Some(Self::from_be_bytes(
            buffer
                .drain(..(Self::get_size_bytes() as usize))
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        ))
    }
}

impl Writeable for i8 {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut self.to_be_bytes().to_vec());
    }
}

impl Sizeable for i8 {
    fn get_size_bytes() -> u16 {
        1
    }
}

impl Readable for i8 {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::get_size_bytes().into() {
            return None;
        }

        Some(Self::from_be_bytes(
            buffer
                .drain(..(Self::get_size_bytes() as usize))
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        ))
    }
}

impl Writeable for i16 {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut self.to_be_bytes().to_vec());
    }
}

impl Sizeable for i16 {
    fn get_size_bytes() -> u16 {
        2
    }
}

impl Readable for i16 {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::get_size_bytes().into() {
            return None;
        }

        Some(Self::from_be_bytes(
            buffer
                .drain(..(Self::get_size_bytes() as usize))
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        ))
    }
}

impl Writeable for i32 {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut self.to_be_bytes().to_vec());
    }
}

impl Sizeable for i32 {
    fn get_size_bytes() -> u16 {
        4
    }
}

impl Readable for i32 {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::get_size_bytes().into() {
            return None;
        }

        Some(Self::from_be_bytes(
            buffer
                .drain(..(Self::get_size_bytes() as usize))
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        ))
    }
}

impl Writeable for i64 {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut self.to_be_bytes().to_vec());
    }
}

impl Sizeable for i64 {
    fn get_size_bytes() -> u16 {
        8
    }
}

impl Readable for i64 {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::get_size_bytes().into() {
            return None;
        }

        Some(Self::from_be_bytes(
            buffer
                .drain(..(Self::get_size_bytes() as usize))
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        ))
    }
}

impl Writeable for f32 {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut self.to_be_bytes().to_vec());
    }
}

impl Sizeable for f32 {
    fn get_size_bytes() -> u16 {
        4
    }
}

impl Readable for f32 {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::get_size_bytes().into() {
            return None;
        }

        Some(Self::from_be_bytes(
            buffer
                .drain(..(Self::get_size_bytes() as usize))
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        ))
    }
}

impl Writeable for bool {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.push(*self as u8);
    }
}

impl Sizeable for bool {
    fn get_size_bytes() -> u16 {
        1
    }
}

impl Readable for bool {
    fn read_from(buffer: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized,
    {
        if buffer.len() < Self::get_size_bytes().into() {
            return None;
        }

        let u8 = u8::read_from(buffer).unwrap();

        match u8 {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
}

impl Writeable for String {
    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.append(&mut self.bytes().collect());
    }
}

pub fn read_string_from(buffer: &mut Vec<u8>, size: usize) -> Option<String> {
    if buffer.len() < size {
        return None;
    }

    String::from_utf8(
        buffer
            .drain(..size)
            .collect::<Vec<u8>>(),
    )
    .ok()
}
