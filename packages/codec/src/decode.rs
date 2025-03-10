use thiserror;
use crate::{FieldNumber, FIELD_NUMBER_LEN};
use chrono::{DateTime, Local, MappedLocalTime, TimeZone};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Invalid character '{0}' (not a digit) on position {1}.")]
    InvalidDigit(u8, usize),
    #[error("Cannot parse '{0}' as boolean on position {1}.")]
    InvalidBoolean(char, usize),
    #[error("Invalid character {0} on position {1}.")]
    InvalidCharacter(char, usize),
    #[error("Invalid number {0} for enum on position {1}.")]
    InvalidEnumNumber(u16, usize),
    #[error("Invalid MID {mid} revision {revision}")]
    InvalidMessage { mid: u16, revision: u16 },
    #[error("Cannot parse bytes to UTF-8 string")]
    InvalidArgNumber { wanted: u8, actual: u8 },
    #[error("This type does not allow for unsized decodes.")]
    UnsizedDecodeNotAllowed,
    #[error("Out of bounds, wants {request}, but total size is {size}.")]
    OutOfRightBound { request: usize, size: usize },
    #[error("Out of bounds, wants to go back {request} positions, but cursor position is {cursor}.")]
    OutOfLeftBound { request: usize, cursor: usize },
    #[error("Invalid timestamp")]
    InvalidTimestamp,
    #[error("Expected character '{expected_char}', but got character '{decoded_char}' on position {pos}.")]
    ExpectedCharacter { decoded_char: char, expected_char: char, pos: usize },
    #[error("Type {type_name} cannot be decoded with size {requested_size}.")]
    SizeMismatch { requested_size: usize, type_name: String },
    #[error("Integer {number} does not fit the type {type_name}.")]
    IntegerOverflow { type_name: String, number: u128 },
    #[error("Insufficient bytes to decode message, header indicates {need} bytes but only have {have} bytes.")]
    InsufficientBytes { have: usize, need: usize },

    #[error("Not implemented")]
    NotImplemented,
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Decode: Sized {
    /// This will take the decoder and return the data itself, parsed from the decoder's input
    /// bytes.
    fn decode(decoder: &mut Decoder) -> Result<Self>;

    fn decode_sized(decoder: &mut Decoder, _size: usize) -> Result<Self> {
        Self::decode(decoder)
    }
}

#[derive(Debug)]
pub struct Decoder<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    pub fn read_byte(&mut self) -> Result<u8> {
        if self.cursor >= self.bytes.len() {
            return Err(Error::OutOfRightBound { request: self.cursor + 1, size: self.bytes.len()});
        }

        let byte = self.bytes[self.cursor];

        self.skip(1)?;
        Ok(byte)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        if (self.cursor + len) > self.bytes.len() {
            return Err(Error::OutOfRightBound { request: self.cursor + len, size: self.bytes.len()});
        }

        let bytes = &self.bytes[self.cursor..(self.cursor + len)];

        self.skip(len)?;
        Ok(bytes)
    }

    pub fn skip(&mut self, len: usize) -> Result<()> {
        if (self.cursor + len) > self.bytes.len() {
            return Err(Error::OutOfRightBound { request: self.cursor + len, size: self.bytes.len()});
        }

        self.cursor += len;
        Ok(())
    }

    pub fn back(&mut self, len: usize) -> Result<()> {
        if self.cursor < len {
            return Err(Error::OutOfLeftBound { request: len, cursor: self.cursor });
        }

        self.cursor -= len;
        Ok(())
    }

    pub fn pos(&self) -> usize {
        self.cursor
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn expect_char(&mut self, expected_char: char) -> Result<()> {
        let decoded_char = char::decode(self)?;

        if decoded_char != expected_char {
            return Err(Error::ExpectedCharacter { decoded_char, expected_char, pos: self.pos() - 1});
        }

        Ok(())
    }

    pub fn read_sized_field<T: Decode>(&mut self, size: usize) -> Result<T> {
        Ok(T::decode_sized(self, size)?)
    }

    pub fn read_numbered_sized_field<T: Decode>(
        &mut self,
        number: FieldNumber,
        size: usize,
    ) -> Result<T> {
        let decoded_number = FieldNumber::decode_sized(self, FIELD_NUMBER_LEN)?;

        if decoded_number != number {
            return Err(Error::InvalidArgNumber { wanted: number, actual: decoded_number });
        }

        self.read_sized_field(size)
    }

    pub fn read_numbered_sized_optional_field<T: Decode>(
        &mut self,
        number: FieldNumber,
        size: usize,
    ) -> Result<Option<T>> {
        let decoded_number = FieldNumber::decode_sized(self, FIELD_NUMBER_LEN)?;

        if decoded_number != number {
            self.back(FIELD_NUMBER_LEN)?;
            return Ok(None)
        }

        Ok(Some(self.read_sized_field(size)?))
    }

    pub fn read_numbered_field<T: Decode>(&mut self, number: FieldNumber) -> Result<T> {
        let decoded_number = FieldNumber::decode_sized(self, FIELD_NUMBER_LEN)?;

        if decoded_number != number {
            return Err(Error::InvalidArgNumber { wanted: number, actual: decoded_number })
        }

        T::decode(self)
    }

    pub fn read_sized_list<T: Decode>(
        &mut self,
        list_length: usize,
        item_size: usize,
    ) -> Result<Vec<T>> {
        let mut list = Vec::with_capacity(list_length);
        for _ in 0..list_length {
            list.push(T::decode_sized(self, item_size)?);
        }
        Ok(list)
    }

    pub fn read_list<T: Decode>(
        &mut self,
        list_length: usize
    ) -> Result<Vec<T>> {
        let mut list = Vec::with_capacity(list_length);
        for _ in 0..list_length {
            list.push(T::decode(self)?);
        }
        Ok(list)
    }
}

impl<'a> From<&'a str> for Decoder<'a> {
    fn from(value: &'a str) -> Self {
        Self { bytes: value.as_bytes(), cursor: 0 }
    }
}

impl<'a> From<&'a [u8]> for Decoder<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self { bytes: value, cursor: 0 }
    }
}


pub fn decode<T: Decode>(bytes: &[u8]) -> Result<T> {
    let mut decoder = Decoder::new(bytes);
    T::decode(&mut decoder)
}


/// Values ranging 0..256, length 1-3
impl Decode for u8 {
    fn decode(_: &mut Decoder) -> Result<Self> {
        Err(Error::UnsizedDecodeNotAllowed)
    }

    fn decode_sized(decoder: &mut Decoder, size: usize) -> Result<Self> {
        if !(1..=3).contains(&size) {
            return Err(Error::SizeMismatch { requested_size: size, type_name: "u8".into() });
        }

        let mut result: u8 = 0;

        for _ in 0..size {
            let raw = decoder.read_byte()?;
            if raw < b'0' || raw > b'9' {
                return Err(Error::InvalidDigit(raw, decoder.pos() - 1));
            }
            let digit = raw - b'0';

            if result > 25 || (result == 25 && digit > 5) {
                return Err(Error::IntegerOverflow { number: (result * 10 + digit) as u128, type_name: "u8".into() });
            }
            result = result * 10 + digit;
        }

        Ok(result)
    }
}

/// Values ranging 0..65536, length 1-5
impl Decode for u16 {
    fn decode(_: &mut Decoder) -> Result<Self> {
        Err(Error::UnsizedDecodeNotAllowed)
    }

    fn decode_sized(decoder: &mut Decoder, size: usize) -> Result<Self> {
        if !(1..=5).contains(&size) {
            return Err(Error::SizeMismatch { requested_size: size, type_name: "u16".into() });
        }

        let mut result: u32 = 0;

        for _ in 0..size {
            let raw = decoder.read_byte()?;
            if raw < b'0' || raw > b'9' {
                return Err(Error::InvalidDigit(raw, decoder.pos() - 1));
            }

            let digit = (raw - b'0') as u32;

            if result > 6553 || (result == 6553 && digit > 5) {
                return Err(Error::IntegerOverflow { number: (result * 10 + digit) as u128, type_name: "u16".into() });
            }
            result = result * 10 + digit;
        }

        Ok(result as u16)
    }
}

/// Values ranging 0..4294967296, length 1-10
impl Decode for u32 {
    fn decode(_: &mut Decoder) -> Result<Self> {
        // If the protocol always requires a size for u32, we keep this an error.
        Err(Error::UnsizedDecodeNotAllowed)
    }

    fn decode_sized(decoder: &mut Decoder, size: usize) -> Result<Self> {
        if !(1..=10).contains(&size) {
            return Err(Error::SizeMismatch { requested_size: size, type_name: "u32".into() });
        }

        let mut result: u64 = 0;

        for _ in 0..size {
            let raw = decoder.read_byte()?;
            if raw < b'0' || raw > b'9' {
                return Err(Error::InvalidDigit(raw, decoder.pos() - 1));
            }
            let digit = (raw - b'0') as u64;

            if result > 429496729 || (result == 429496729 && digit > 5) {
                return Err(Error::IntegerOverflow { number: (result * 10 + digit) as u128, type_name: "u32".into() });
            }
            result = result * 10 + digit;
        }

        Ok(result as u32)
    }
}


/// Values ranging 0..18446744073709551616, length 1-20
impl Decode for u64 {
    fn decode(_: &mut Decoder) -> Result<Self> {
        // If the protocol always requires a size for u64, we keep this an error.
        Err(Error::UnsizedDecodeNotAllowed)
    }

    fn decode_sized(decoder: &mut Decoder, size: usize) -> Result<Self> {
        if !(1..=20).contains(&size) {
            return Err(Error::SizeMismatch { requested_size: size, type_name: "u64".into() });
        }

        let mut result: u128 = 0;

        for _ in 0..size {
            let raw = decoder.read_byte()?;
            if raw < b'0' || raw > b'9' {
                return Err(Error::InvalidDigit(raw, decoder.pos() - 1));
            }
            let digit = (raw - b'0') as u128;

            if result > 1844674407370955161 || (result == 1844674407370955161 && digit > 5) {
                return Err(Error::IntegerOverflow { number: result * 10 + digit, type_name: "u64".into() });
            }
            result = result * 10 + digit;
        }

        Ok(result as u64)
    }
}

/// Raw ASCII character, length 1
impl Decode for char {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let byte = decoder.read_byte()?;
        Ok(byte.into())
    }

    fn decode_sized(decoder: &mut Decoder, size: usize) -> Result<Self> {
        if size != 1 {
            return Err(Error::SizeMismatch { requested_size: size, type_name: "char".into() });
        }
        Self::decode(decoder)
    }
}

/// String based on the ASCII characters in the decoder, can be length 0-infinite.
impl Decode for String {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let mut chars = Vec::new();

        while decoder.pos() < decoder.len() {
            let next_char = char::decode(decoder)?;

            if next_char == ' ' || next_char == '\0' {
                break;
            }

            chars.push(char::decode(decoder)?);
        }

        Ok(String::from_iter(chars))
    }

    fn decode_sized(decoder: &mut Decoder, size: usize) -> Result<Self> {
        let chars = decoder.read_bytes(size)?;

        let mut end = size;
        while chars[end - 1] == b' ' {
            end -= 1;
        }

        let string = String::from_utf8_lossy(&chars[0..end]).to_string();
        Ok(string)
    }
}

/// Values 0 and 1 only. Length is always 1.
impl Decode for bool {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let decoded_char = char::decode(decoder)?;

        match decoded_char {
            '1' => Ok(true),
            '0' => Ok(false),
            _ => Err(Error::InvalidBoolean(decoded_char, decoder.pos() - 1)),
        }
    }

    fn decode_sized(decoder: &mut Decoder, size: usize) -> Result<Self> {
        if size != 1 {
            return Err(Error::SizeMismatch { requested_size: size, type_name: "bool".into() });
        }
        Self::decode(decoder)
    }
}

impl<T> Decode for Option<T> where T: Decode {
    fn decode(_: &mut Decoder) -> Result<Self> {
        Err(Error::UnsizedDecodeNotAllowed)
    }

    fn decode_sized(decoder: &mut Decoder, size: usize) -> Result<Self> {
        let bytes = decoder.read_bytes(size)?;

        if bytes.iter().all(|&byte| byte == b' ') {
            Ok(None)
        } else {
            decoder.back(size)?;
            Ok(Some(T::decode_sized(decoder, size)?))
        }
    }
}

impl Decode for DateTime<Local> {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let year = u32::decode_sized(decoder, 4)?;
        decoder.expect_char('-')?;
        let month = u32::decode_sized(decoder, 2)?;
        decoder.expect_char('-')?;
        let day = u32::decode_sized(decoder, 2)?;
        decoder.expect_char(':')?;
        let hour = u32::decode_sized(decoder, 2)?;
        decoder.expect_char(':')?;
        let min = u32::decode_sized(decoder, 2)?;
        decoder.expect_char(':')?;
        let sec = u32::decode_sized(decoder, 2)?;

        match Local.with_ymd_and_hms(year as i32, month, day, hour, min, sec) {
            MappedLocalTime::Single(timestamp) => Ok(timestamp),
            _ => Err(Error::InvalidTimestamp)
        }
    }

    fn decode_sized(decoder: &mut Decoder, size: usize) -> Result<Self> {
        if size != 19 {
            return Err(Error::SizeMismatch { requested_size: size, type_name: "DateTime".into() });
        }
        Self::decode(decoder)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Local, TimeZone};
    use crate::decode::Error;
    use crate::decode::{Decode, Decoder, Result};

    #[test]
    fn test_read_byte() {
        let bytes = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8'];

        let mut decoder = Decoder::new(bytes.as_slice());

        assert_eq!(decoder.read_byte(), Ok(b'1'));
        assert_eq!(decoder.read_byte(), Ok(b'2'));
        assert_eq!(decoder.read_byte(), Ok(b'3'));
        assert_eq!(decoder.read_byte(), Ok(b'4'));

        assert_eq!(decoder.pos(), 4usize);
    }

    #[test]
    fn test_read_bytes() {
        let bytes = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8'];

        let mut decoder = Decoder::new(&bytes[..]);

        assert_eq!(
            decoder.read_bytes(4),
            Ok([b'1', b'2', b'3', b'4'].as_slice())
        );
        assert_eq!(decoder.read_bytes(2), Ok([b'5', b'6'].as_slice()));

        assert_eq!(decoder.pos(), 6usize);
    }

    #[test]
    fn test_read_bool() {
        let bytes = [b'1', b'0', b'1', b'0'];
        let mut decoder = Decoder::new(&bytes[..]);

        assert_eq!(bool::decode(&mut decoder), Ok(true));
        assert_eq!(bool::decode(&mut decoder), Ok(false));
        assert_eq!(bool::decode(&mut decoder), Ok(true));
        assert_eq!(bool::decode(&mut decoder), Ok(false));
        assert_eq!(decoder.pos(), 4);
    }

    #[test]
    fn test_read_u8() {
        let bytes = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8'];
        let mut decoder = Decoder::new(&bytes[..]);

        assert_eq!(u8::decode_sized(&mut decoder, 3), Ok(123));
        assert_eq!(u8::decode_sized(&mut decoder, 2), Ok(45));
        assert_eq!(decoder.pos(), 5);
    }

    #[test]
    fn test_read_u8_unsized() {
        let bytes = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8'];
        let mut decoder = Decoder::new(&bytes[..]);

        assert_eq!(u8::decode(&mut decoder), Err(Error::UnsizedDecodeNotAllowed));
        assert_eq!(decoder.pos(), 0);
    }

    #[test]
    fn test_read_u8_too_large() {
        let bytes = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8'];
        let mut decoder = Decoder::new(&bytes[..]);

        assert_eq!(u8::decode_sized(&mut decoder, 5), Err(Error::IntegerOverflow { number: 12345, type_name: "u8".into()}));
        assert_eq!(decoder.pos(), 0);
    }

    #[test]
    fn test_read_u16() {
        let bytes = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8'];
        let mut decoder = Decoder::new(&bytes[..]);

        assert_eq!(u16::decode_sized(&mut decoder, 5), Ok(12345));
        assert_eq!(decoder.pos(), 5);
    }

    #[test]
    fn test_read_u32() {
        let bytes = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8'];
        let mut decoder = Decoder::new(&bytes[..]);

        assert_eq!(u32::decode_sized(&mut decoder, 8), Ok(12345678));
        assert_eq!(decoder.pos(), 8);
    }

    #[test]
    fn test_read_string() {
        let bytes = [b'H', b'e', b'l', b'l', b'o', b'6', b'7', b'8'];
        let mut decoder = Decoder::new(&bytes[..]);

        assert_eq!(String::decode_sized(&mut decoder, 5), Ok("Hello".to_string()));
        assert_eq!(decoder.pos(), 5);
    }

    #[test]
    fn test_read_option_string() {
        let bytes = [b'H', b'e', b'l', b'l', b'o', b'6', b'7', b'8'];
        let mut decoder = Decoder::new(&bytes[..]);

        let val: Result<Option<String>> = Option::decode_sized(&mut decoder, 5);

        assert_eq!(val, Ok(Some("Hello".to_string())));
        assert_eq!(decoder.pos(), 5);
    }

    #[test]
    fn test_read_option_none() {
        let bytes = [b' ', b' ', b' ', b' ', b' ', b'6', b'7', b'8'];
        let mut decoder = Decoder::new(&bytes[..]);

        let val: Result<Option<String>> = Option::decode_sized(&mut decoder, 5);

        assert_eq!(val, Ok(None));
        assert_eq!(decoder.pos(), 5);
    }

    #[test]
    fn test_read_sized_field() {
        let bytes = [b'H', b'e', b'l', b'l', b'o', b'6', b'7', b'8'];
        let mut decoder = Decoder::new(&bytes[..]);

        assert_eq!(decoder.read_sized_field(5), Ok("Hello".to_string()));
        assert_eq!(decoder.pos(), 5);
    }

    #[test]
    fn test_read_numbered_sized_field() {
        let bytes = [b'0', b'1', b'H', b'e', b'l', b'l', b'o', b'0', b'2', b'1'];
        let mut decoder = Decoder::new(&bytes[..]);

        assert_eq!(decoder.read_numbered_sized_field(1, 5), Ok("Hello".to_string()));
        assert_eq!(decoder.read_numbered_sized_field(2, 1), Ok(true));
        assert_eq!(decoder.pos(), 10);
    }

    #[test]
    fn test_read_numbered_sized_field_invalid_number() {
        let bytes = [b'0', b'1', b'H', b'e', b'l', b'l', b'o', b'0', b'2', b'1'];
        let mut decoder = Decoder::new(&bytes[..]);

        assert_eq!(decoder.read_numbered_sized_field::<String>(4, 5), Err(Error::InvalidArgNumber { wanted: 4, actual: 1 }));
        assert_eq!(decoder.pos(), 2);
    }

    #[test]
    fn test_read_timestamp() {
        let str = "2001-12-01:20:12:45000000";
        let mut decoder = Decoder::new(str.as_bytes());
        let actual_timestamp = Local.with_ymd_and_hms(2001, 12, 1, 20, 12, 45).unwrap();

        let timestamp_res: Result<DateTime<Local>> = DateTime::decode(&mut decoder);

        assert_eq!(timestamp_res, Ok(actual_timestamp));
    }

    #[test]
    fn test_read_invalid_timestamp() {
        let str = "2001:12:01:20:12:45000000";
        let mut decoder = Decoder::new(str.as_bytes());

        let timestamp_res: Result<DateTime<Local>> = DateTime::decode(&mut decoder);

        assert_eq!(timestamp_res, Err(Error::ExpectedCharacter { decoded_char: ':', expected_char: '-', pos: 4 }));
    }
}
