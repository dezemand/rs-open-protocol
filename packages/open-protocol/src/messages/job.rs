use chrono::{DateTime, Local};
use open_protocol_codec::{decode, encode};
use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0030, revision = 1)]
pub struct MID0030rev1 {
    // No data field for this MID
}

#[derive(
    Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage,
)]
#[open_protocol_message(MID = 0031, revision = 1)]
pub struct MID0031rev1 {
    /// Number of jobs available on the controller
    #[open_protocol_field(length = 2)]
    pub number_of_jobs: u8,

    /// List of Job IDs, each ID is 2 ASCII characters long
    #[open_protocol_field(list, amount = "number_of_jobs", length = 2)]
    pub job_ids: Vec<u8>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0031, revision = 2)]
pub struct MID0031rev2 {
    /// Number of jobs available on the controller
    #[open_protocol_field(length = 4)]
    pub number_of_jobs: u16,

    /// List of Job IDs, each ID is 4 ASCII characters long
    #[open_protocol_field(list, amount = "number_of_jobs", length = 4)]
    pub job_ids: Vec<u16>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0032, revision = 1)]
pub struct MID0032rev1 {
    /// The ID of the requested job, 2 ASCII characters long
    #[open_protocol_field(length = 2)]
    pub job_id: u8,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0032, revision = 2)]
pub struct MID0032rev2 {
    /// The ID of the requested job, 4 ASCII characters long
    #[open_protocol_field(length = 4)]
    pub job_id: u16,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0033, revision = 1)]
pub struct MID0033rev1 {
    /// The ID of the job
    #[open_protocol_field(number = 1, length = 2)]
    pub job_id: u8,

    /// The name of the job, maximum 25 characters
    #[open_protocol_field(number = 2, length = 25)]
    pub job_name: String,

    /// Indicates the order type: 0=free, 1=forced, 2=free and forced
    #[open_protocol_field(number = 3, length = 1)]
    pub forced_order: u8,

    /// Maximum time allowed for the first tightening in the job, in seconds
    #[open_protocol_field(number = 4, length = 4)]
    pub max_time_first_tightening: u16,

    /// Maximum time allowed to complete the job, in seconds
    #[open_protocol_field(number = 5, length = 5)]
    pub max_time_to_complete_job: u32,

    /// Job batch mode: 0=Only OK tightenings counted, 1=Both OK and NOK tightenings counted
    #[open_protocol_field(number = 6, length = 1)]
    pub job_batch_mode: u8,

    /// Indicates if the tool should be locked when the job is done: 0=No, 1=Yes
    #[open_protocol_field(number = 7, length = 1)]
    pub lock_at_job_done: u8,

    /// Indicates if line control is used: 0=No, 1=Yes
    #[open_protocol_field(number = 8, length = 1)]
    pub use_line_control: u8,

    /// Indicates if the job is repeatable: 0=No, 1=Yes
    #[open_protocol_field(number = 9, length = 1)]
    pub repeat_job: u8,

    /// Tool loosening mode: 0=Enable, 1=Disable, 2=Enable only on NOK tightenings
    #[open_protocol_field(number = 10, length = 1)]
    pub tool_loosening: u8,

    /// Reserved for future use
    #[open_protocol_field(number = 11, length = 1)]
    pub reserved: u8,

    /// Number of parameter sets included in the job
    #[open_protocol_field(number = 12, length = 2)]
    pub number_of_parameter_sets: u8,

    /// List of parameter sets in the job
    #[open_protocol_field(list, amount = "number_of_parameter_sets", length = 12)]
    pub job_list: Vec<JobParameterRev1>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0034, revision = 1)]
pub struct MID0034rev1 {
    // No data field for this MID
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0035, revision = 1)]
pub struct MID0035rev1 {
    /// ID of the job being executed
    #[open_protocol_field(number = 1, length = 2)]
    pub job_id: u8,

    /// Status of the job: 0=Not completed, 1=OK, 2=NOK
    #[open_protocol_field(number = 2, length = 1)]
    pub job_status: u8,

    /// Batch mode of the job
    #[open_protocol_field(number = 3, length = 1)]
    pub job_batch_mode: u8,

    /// Total number of tightenings required in the job
    #[open_protocol_field(number = 4, length = 4)]
    pub job_batch_size: u16,

    /// Current count of completed tightenings in the job
    #[open_protocol_field(number = 5, length = 4)]
    pub job_batch_counter: u16,

    /// Timestamp of the job information
    #[open_protocol_field(number = 6, length = 19)]
    pub timestamp: DateTime<Local>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0036, revision = 1)]
pub struct MID0036rev1 {
    // No data field for this MID
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0037, revision = 1)]
pub struct MID0037rev1 {
    // No data field for this MID
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0038, revision = 1)]
pub struct MID0038rev1 {
    /// ID of the job to be selected, 2 ASCII characters
    #[open_protocol_field(number = 1, length = 2)]
    pub job_id: u8,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0038, revision = 2)]
pub struct MID0038rev2 {
    /// ID of the job to be selected, 4 ASCII characters
    #[open_protocol_field(number = 1, length = 4)]
    pub job_id: u16,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0039, revision = 1)]
pub struct MID0039rev1 {
    /// ID of the job to be restarted, 2 ASCII characters
    #[open_protocol_field(number = 1, length = 2)]
    pub job_id: u8,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0039, revision = 2)]
pub struct MID0039rev2 {
    /// ID of the job to be restarted, 4 ASCII characters
    #[open_protocol_field(number = 1, length = 4)]
    pub job_id: u16,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct JobParameterRev1 {
    /// ID of the channel associated with this parameter set
    pub channel_id: u8,

    /// ID of the program or parameter set
    pub program_id: u16,

    /// Indicates if auto-select is enabled: 0=No, 1=Yes
    pub auto_select: bool,

    /// Batch size for this parameter set
    pub batch_size: u8,
}

impl decode::Decode for JobParameterRev1 {
    fn decode(decoder: &mut decode::Decoder) -> decode::Result<Self> {
        let channel_id: u8 = decoder.read_sized_field(2)?;
        decoder.expect_char(':')?;
        let program_id: u16 = decoder.read_sized_field(3)?;
        decoder.expect_char(':')?;
        let auto_select: bool = decoder.read_sized_field(1)?;
        decoder.expect_char(':')?;
        let batch_size: u8 = decoder.read_sized_field(2)?;
        decoder.expect_char(';')?;

        Ok(Self {
            channel_id,
            program_id,
            auto_select,
            batch_size,
        })
    }
}

impl encode::Encode for JobParameterRev1 {
    fn encode(&self, encoder: &mut encode::Encoder) -> encode::Result<()> {
        self.channel_id.encode_sized(encoder, 2)?;
        ':'.encode(encoder)?;
        self.program_id.encode_sized(encoder, 3)?;
        ':'.encode(encoder)?;
        self.auto_select.encode_sized(encoder, 1)?;
        ':'.encode(encoder)?;
        self.batch_size.encode_sized(encoder, 2)?;
        ';'.encode(encoder)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use open_protocol_codec::{decode, encode, message::Message};

    #[test]
    fn test_mid0030rev1() {
        assert_eq!(MID0030rev1::mid(), 30);
        assert_eq!(MID0030rev1::revision(), 1);
        assert_eq!(MID0030rev1 {}.to_mid(), 30);
        assert_eq!(MID0030rev1 {}.to_revision(), 1);
    }

    #[test]
    fn test_mid0031rev1() {
        let message = MID0031rev1 {
            number_of_jobs: 2,
            job_ids: vec![3, 4],
        };

        assert_eq!(message.to_mid(), MID0031rev1::mid());
        assert_eq!(message.to_mid(), 31);
        assert_eq!(message.to_revision(), MID0031rev1::revision());
        assert_eq!(message.to_revision(), 1);
    }

    #[test]
    fn test_mid0035rev1_encode() {
        let message = MID0035rev1 {
            job_id: 1,
            job_status: 0,
            job_batch_mode: 0,
            job_batch_size: 8,
            job_batch_counter: 3,
            timestamp: Local.with_ymd_and_hms(2001, 12, 1, 20, 12, 45).unwrap(),
        };

        let encoded = encode::encode(&message);

        assert_eq!(
            encoded,
            Ok("0101020030040008050003062001-12-01:20:12:45".to_string())
        );
    }

    #[test]
    fn test_mid0035rev1_decode() {
        let str = "0101020030040008050003062001-12-01:20:12:45";
        let decoded = decode::decode::<MID0035rev1>(str.as_bytes());
        let obj = MID0035rev1 {
            job_id: 1,
            job_status: 0,
            job_batch_mode: 0,
            job_batch_size: 8,
            job_batch_counter: 3,
            timestamp: Local.with_ymd_and_hms(2001, 12, 1, 20, 12, 45).unwrap(),
        };

        assert_eq!(decoded, Ok(obj));
    }
}
