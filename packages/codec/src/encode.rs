use chrono::{DateTime, Datelike, Local, Timelike};
use thiserror;
use crate::{FieldNumber, FIELD_NUMBER_LEN};

pub trait Encode {
    fn encode(&self, encoder: &mut Encoder) -> Result<()>;

    fn encode_sized(&self , encoder: &mut Encoder, _size: usize) -> Result<()> {
        self.encode(encoder)
    }
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("This feature is not implemented yet")]
    NotImplemented,
    #[error("Cannot write this as {0} characters")]
    InvalidSize(usize),
    #[error("Unsized encode is not allowed for this type.")]
    UnsizedEncodeNotAllowed,
    #[error("Character is a non-ASCII character, which is required.")]
    NonAsciiCharacter(char)
}

pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug, Clone)]
pub struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Self {
        Self { bytes: vec![] }
    }

    pub fn write_byte(&mut self, byte: &u8) -> Result<()> {
        self.bytes.push(*byte);
        Ok(())
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        for byte_char in bytes {
            self.write_byte(byte_char)?;
        }
        Ok(())
    }

    pub fn write_sized_field<T: Encode>(&mut self, item: &T, size: usize) -> Result<()> {
        item.encode_sized(self, size)?;
        Ok(())
    }

    pub fn write_numbered_sized_field<T: Encode>(&mut self, item: &T, number: FieldNumber, size: usize) -> Result<()> {
        number.encode_sized(self, FIELD_NUMBER_LEN)?;
        item.encode_sized(self, size)?;
        Ok(())
    }

    pub fn write_numbered_sized_optional_field<T: Encode>(&mut self, item: &Option<T>, number: FieldNumber, size: usize) -> Result<()> {
        match item {
            Some(content) => {
                number.encode_sized(self, FIELD_NUMBER_LEN)?;
                content.encode_sized(self, size)?;
            },
            None => {},
        };
        Ok(())
    }

    pub fn write_numbered_field<T: Encode>(&mut self, item: &T, number: FieldNumber) -> Result<()> {
        number.encode_sized(self, FIELD_NUMBER_LEN)?;
        item.encode(self)?;
        Ok(())
    }

    pub fn write_numbered_optional_field<T: Encode>(&mut self, item: &Option<T>, number: FieldNumber) -> Result<()> {
        match item {
            Some(content) => {
                number.encode_sized(self, FIELD_NUMBER_LEN)?;
                content.encode(self)?;
            },
            None => {},
        }
        Ok(())
    }

    pub fn write_sized_list<T: Encode>(&mut self, list: &Vec<T>, item_size: usize, amount: usize) -> Result<()> {
        for index in 0..amount {
            let item = &list[index];
            self.write_sized_field(item, item_size)?;
        }

        Ok(())
    }

    pub fn write_list<T: Encode>(&mut self, list: &Vec<T>, amount: usize) -> Result<()> {
        for index in 0..amount {
            let item = &list[index];
            item.encode(self)?
        }

        Ok(())
    }

    pub fn to_string(&self) -> String {
        let bytes = self.bytes.clone();
        String::from_utf8(bytes).unwrap()
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn append(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }
}

pub fn encode<T: Encode>(item: &T) -> Result<String> {
    let mut encoder = Encoder::new();
    item.encode(&mut encoder)?;
    Ok(encoder.to_string())
}

impl Encode for u8 {
    fn encode(&self, _: &mut Encoder) -> Result<()> {
        Err(Error::UnsizedEncodeNotAllowed)
    }

    fn encode_sized(&self, encoder: &mut Encoder, size: usize) -> Result<()> {
        for i in (0..size).rev() {
            let digit_byte = (self / 10u8.pow(i as u32)) % 10 + b'0';
            encoder.write_byte(&digit_byte)?;
        }
        Ok(())
    }
}

impl Encode for u16 {
    fn encode(&self, _: &mut Encoder) -> Result<()> {
        Err(Error::UnsizedEncodeNotAllowed)
    }

    fn encode_sized(&self, encoder: &mut Encoder, size: usize) -> Result<()> {
        for i in (0..size).rev() {
            let digit_byte = ((self / 10u16.pow(i as u32)) % 10) as u8 + b'0';
            encoder.write_byte(&digit_byte)?;
        }

        Ok(())
    }
}

impl Encode for u32 {
    fn encode(&self, _: &mut Encoder) -> Result<()> {
        Err(Error::UnsizedEncodeNotAllowed)
    }

    fn encode_sized(&self, encoder: &mut Encoder, size: usize) -> Result<()> {
        for i in (0..size).rev() {
            let digit_byte = ((self / 10u32.pow(i as u32)) % 10) as u8 + b'0';
            encoder.write_byte(&digit_byte)?;
        }

        Ok(())
    }
}

impl Encode for u64 {
    fn encode(&self, _: &mut Encoder) -> Result<()> {
        Err(Error::UnsizedEncodeNotAllowed)
    }

    fn encode_sized(&self, encoder: &mut Encoder, size: usize) -> Result<()> {
        for i in (0..size).rev() {
            let digit_byte = ((self / 10u64.pow(i as u32)) % 10) as u8 + b'0';
            encoder.write_byte(&digit_byte)?;
        }

        Ok(())
    }
}

impl Encode for char {
    fn encode(&self, encoder: &mut Encoder) -> Result<()> {
        if !self.is_ascii() {
            return Err(Error::NonAsciiCharacter(*self));
        }

        encoder.write_byte(&u8::try_from(u32::from(*self)).unwrap())?;
        Ok(())
    }
}

impl Encode for String {
    fn encode(&self, encoder: &mut Encoder) -> Result<()> {
        for char in self.chars() {
            char.encode(encoder)?;
        }

        Ok(())
    }

    fn encode_sized(&self, encoder: &mut Encoder, size: usize) -> Result<()> {
        if self.len() > size {
            return Err(Error::InvalidSize(size));
        }

        for char in self.chars() {
            char.encode(encoder)?;
        }

        for _ in 0..(size - self.len()) {
            ' '.encode(encoder)?;
        }

        Ok(())
    }
}

impl Encode for bool {
    fn encode(&self, encoder: &mut Encoder) -> Result<()> {
        match *self {
            true => '1'.encode(encoder),
            false => '0'.encode(encoder),
        }
    }

    fn encode_sized(&self, encoder: &mut Encoder, size: usize) -> Result<()> {
        if size != 1 {
            return Err(Error::InvalidSize(size));
        }

        self.encode(encoder)
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, _: &mut Encoder) -> Result<()> {
        Err(Error::UnsizedEncodeNotAllowed)
    }

    fn encode_sized(&self, encoder: &mut Encoder, size: usize) -> Result<()> {
        match self {
            None => {
                for char in std::iter::repeat(' ').take(size) {
                    char.encode(encoder)?;
                }
            },
            Some(item) => {
                item.encode_sized(encoder, size)?
            },
        };

        Ok(())
    }
}

impl Encode for DateTime<Local> {
    fn encode(&self, encoder: &mut Encoder) -> Result<()> {
        (self.year() as u32).encode_sized(encoder, 4)?;
        '-'.encode(encoder)?;
        self.month().encode_sized(encoder, 2)?;
        '-'.encode(encoder)?;
        self.day().encode_sized(encoder, 2)?;
        ':'.encode(encoder)?;
        self.hour().encode_sized(encoder, 2)?;
        ':'.encode(encoder)?;
        self.minute().encode_sized(encoder, 2)?;
        ':'.encode(encoder)?;
        self.second().encode_sized(encoder, 2)?;

        Ok(())
    }
}
