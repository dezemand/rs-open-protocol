use std::ops::Deref;
use open_protocol_codec::decode::{self, Decode, Decoder};
use open_protocol_codec::encode::{self, Encode, Encoder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceSample(u16);

impl TraceSample {
    pub fn number(&self) -> u16 {
        self.0
    }
}

impl Encode for TraceSample {
    fn encode(&self, _encoder: &mut Encoder) -> encode::Result<()> {
        Err(encode::Error::UnsizedEncodeNotAllowed)
    }

    fn encode_sized(&self, encoder: &mut Encoder, size: usize) -> encode::Result<()> {
        if size != 2 {
            return Err(encode::Error::InvalidSize(size));
        }

        let bytes = self.0.to_be_bytes();
        encoder.write_byte(&bytes[0])?;
        encoder.write_byte(&bytes[1])?;
        Ok(())
    }
}

impl Decode for TraceSample {
    fn decode(_decoder: &mut Decoder) -> decode::Result<Self> {
        Err(decode::Error::UnsizedDecodeNotAllowed)
    }

    fn decode_sized(decoder: &mut Decoder, size: usize) -> decode::Result<Self> {
        if size != 2 {
            return Err(decode::Error::SizeMismatch { requested_size: size, type_name: "TraceSample".into() })
        }

        let b0 = decoder.read_byte()?;
        let b1 = decoder.read_byte()?;
        Ok(TraceSample(u16::from_be_bytes([b0, b1])))
    }
}

impl From<u16> for TraceSample {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Into<u16> for TraceSample {
    fn into(self) -> u16 {
        self.0
    }
}

impl Deref for TraceSample {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
