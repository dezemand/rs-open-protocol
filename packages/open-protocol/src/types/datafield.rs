use chrono::{DateTime, Local};
use open_protocol_codec::decode::{self, Decode, Decoder};
use open_protocol_codec::encode::{self, Encode, Encoder};
use crate::enums::datatype::DataType;

pub trait ParseDataField<T> {
    fn parse(&self) -> decode::Result<T>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataField {
    /// Parameter id (PID). 5 bytes, UI. The available PIDs may vary depending on the system type.
    pub parameter_id: u32,
    /// Length. 3 bytes, UI. Length of data value.
    pub length: u16,
    /// Data Type. 2 bytes, UI. Data type of the data value.
    pub data_type: DataType,
    /// Unit. 3 bytes, UI. Unit of the data.
    pub unit: u16,
    /// Step no. 4 bytes, UI. The step number for the result variable. Sent as 0000 if not relevant.
    pub step_no: Option<u16>,
    /// The data value.
    pub data_value: String,
}

impl Encode for DataField {
    fn encode(&self, encoder: &mut Encoder) -> encode::Result<()> {
        self.parameter_id.encode_sized(encoder, 5)?;
        self.length.encode_sized(encoder, 3)?;
        self.data_type.encode_sized(encoder, 2)?;
        self.unit.encode_sized(encoder, 3)?;
        self.step_no.unwrap_or(0).encode_sized(encoder, 4)?;
        self.data_value
            .encode_sized(encoder, self.length as usize)?;
        Ok(())
    }
}

impl Decode for DataField {
    fn decode(decoder: &mut Decoder) -> decode::Result<Self> {
        let parameter_id = u32::decode_sized(decoder, 5)?;
        let length = u16::decode_sized(decoder, 3)?;
        let data_type = DataType::decode_sized(decoder, 2)?;
        let unit = u16::decode_sized(decoder, 3)?;
        let step_no = u16::decode_sized(decoder, 4)?;
        let data_value = String::decode_sized(decoder, length as usize)?;

        Ok(Self {
            parameter_id,
            length,
            data_type,
            unit,
            step_no: if step_no == 0 { None } else { Some(step_no) },
            data_value,
        })
    }
}

impl ParseDataField<u16> for DataField {
    fn parse(&self) -> decode::Result<u16> {
        if self.data_type != DataType::UnsignedInteger {
            return Err(decode::Error::NotImplemented);
        }

        let mut decoder = Decoder::new(self.data_value.as_bytes());
        u16::decode_sized(&mut decoder, self.length as usize)
    }
}

impl ParseDataField<u32> for DataField {
    fn parse(&self) -> decode::Result<u32> {
        if self.data_type != DataType::UnsignedInteger {
            return Err(decode::Error::NotImplemented);
        }

        let mut decoder = Decoder::new(self.data_value.as_bytes());
        u32::decode_sized(&mut decoder, self.length as usize)
    }
}

impl ParseDataField<u64> for DataField {
    fn parse(&self) -> decode::Result<u64> {
        if self.data_type != DataType::UnsignedInteger {
            return Err(decode::Error::NotImplemented);
        }

        let mut decoder = Decoder::new(self.data_value.as_bytes());
        u64::decode_sized(&mut decoder, self.length as usize)
    }
}

impl ParseDataField<bool> for DataField {
    fn parse(&self) -> decode::Result<bool> {
        if self.data_type != DataType::Boolean {
            return Err(decode::Error::NotImplemented);
        }

        let mut decoder = Decoder::new(self.data_value.as_bytes());
        bool::decode_sized(&mut decoder, self.length as usize)
    }
}

impl ParseDataField<String> for DataField {
    fn parse(&self) -> decode::Result<String> {
        if self.data_type != DataType::String {
            return Err(decode::Error::NotImplemented);
        }

        Ok(self.data_value.clone())
    }
}

impl ParseDataField<DateTime<Local>> for DataField {
    fn parse(&self) -> decode::Result<DateTime<Local>> {
        if self.data_type != DataType::Timestamp {
            return Err(decode::Error::NotImplemented);
        }

        let mut decoder = Decoder::new(self.data_value.as_bytes());
        DateTime::<Local>::decode_sized(&mut decoder, self.length as usize)
    }
}

impl ParseDataField<Vec<u8>> for DataField {
    fn parse(&self) -> decode::Result<Vec<u8>> {
        match self.data_type {
            DataType::Hexadecimal => {
                let mut decoder = Decoder::new(self.data_value.as_bytes());
                let num_bytes = (self.length as usize) / 2;
                let mut bytes = Vec::with_capacity(num_bytes);

                for _ in 0..num_bytes {
                    let c1 = char::decode_sized(&mut decoder, 1)?;
                    let c2 = char::decode_sized(&mut decoder, 1)?;
                    let hi = c1.to_digit(16).ok_or(decode::Error::NotImplemented)?;
                    let lo = c2.to_digit(16).ok_or(decode::Error::NotImplemented)?;
                    bytes.push((hi * 16 + lo) as u8);
                }

                Ok(bytes)
            },

            _ => Err(decode::Error::NotImplemented),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let packet = "0000100201001000012";

        let df: DataField = decode::decode(packet.as_bytes()).unwrap();

        assert_eq!(df, DataField {
            parameter_id: 1,
            length: 2,
            data_type: DataType::UnsignedInteger,
            unit: 1,
            step_no: None,
            data_value: "12".into()
        });
    }

    #[test]
    fn test_encode() {
        let df = DataField {
            parameter_id: 1,
            length: 2,
            data_type: DataType::UnsignedInteger,
            unit: 1,
            step_no: None,
            data_value: "12".into()
        };

        let encoded = encode::encode(&df).unwrap();

        assert_eq!(encoded, "0000100201001000012");
    }

    #[test]
    fn test_parse_bytes() {
        let df = DataField {
            parameter_id: 1,
            length: 8,
            data_type: DataType::Hexadecimal,
            unit: 1,
            step_no: None,
            data_value: "DEADBEEF".into()
        };

        let parsed: decode::Result<Vec<u8>> = df.parse();

        assert_eq!(parsed, Ok(vec![0xDE, 0xAD, 0xBE, 0xEF]));
    }
}
