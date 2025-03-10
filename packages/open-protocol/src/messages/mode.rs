use chrono::{DateTime, Local};
use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 2600, revision = 1)]
pub struct MID2600rev1 {
}

/// Contains the list of all modes in the controller.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 2601, revision = 1)]
pub struct MID2601rev1 {
    /// Number of modes in the controller
    #[open_protocol_field(length = 3)]
    pub number_of_modes: u16,

    /// List of mode data entries
    #[open_protocol_field(list, amount = "number_of_modes")]
    pub mode_data: Vec<ModeData>,
}

/// Mode details within a mode list.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub struct ModeData {
    /// Mode ID
    #[open_protocol_field(length = 4)]
    pub mode_id: u32,

    /// Length of the mode name
    #[open_protocol_field(length = 2)]
    pub mode_name_size: u16,

    /// The name of the mode (variable length, depending on `mode_name_size`)
    #[open_protocol_field(length = "mode_name_size")]
    pub mode_name: String,
}

/// Requests detailed mode data for a specific mode ID.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 2602, revision = 1)]
pub struct MID2602rev1 {
    /// Mode ID of the mode to request
    #[open_protocol_field(number = 1, length = 4)]
    pub mode_id: u32,
}

/// Provides detailed information about a mode, including its bolts.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 2603, revision = 1)]
pub struct MID2603rev1 {
    /// Mode ID
    #[open_protocol_field(length = 4)]
    pub mode_id: u32,

    /// Length of the mode name
    #[open_protocol_field(length = 2)]
    pub mode_name_size: u16,

    /// The name of the mode
    #[open_protocol_field(length = "mode_name_size")]
    pub mode_name: String,

    /// Number of bolts in the mode
    #[open_protocol_field(length = 3)]
    pub number_of_bolts: u16,

    /// List of bolts assigned to the mode
    #[open_protocol_field(list, amount = "number_of_bolts")]
    pub bolt_data: Vec<BoltData>,
}

/// Contains details about a bolt in a mode.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub struct BoltData {
    /// Parameter set ID
    #[open_protocol_field(length = 3)]
    pub pset_id: u16,

    /// Tool number
    #[open_protocol_field(length = 3)]
    pub tool_number: u16,

    /// Bolt number
    #[open_protocol_field(length = 4)]
    pub bolt_number: u32,

    /// Length of the bolt name
    #[open_protocol_field(length = 2)]
    pub bolt_name_size: u16,

    /// The name of the bolt
    #[open_protocol_field(length = "bolt_name_size")]
    pub bolt_name: String,
}

/// Confirms mode selection and provides metadata.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 2604, revision = 1)]
pub struct MID2604rev1 {
    /// Mode ID of the selected mode
    #[open_protocol_field(number = 1, length = 4)]
    pub mode_id: u32,

    /// Date of the last change in the mode setting (format YYYY-MM-DD:HH:MM:SS)
    #[open_protocol_field(number = 2, length = 19)]
    pub last_change_date: DateTime<Local>,

    /// Number of bolts in the mode
    #[open_protocol_field(number = 3, length = 3)]
    pub number_of_bolts: u16,
}

/// Acknowledgment of mode selection.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 2605, revision = 1)]
pub struct MID2605rev1 {
}

/// Requests mode selection by ID.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 2606, revision = 1)]
pub struct MID2606rev1 {
    /// Mode ID to be selected
    #[open_protocol_field(number = 1, length = 4)]
    pub mode_id: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_protocol_codec::{decode, encode};

    #[test]
    fn encode_2601() {
        let mode_name_1 = "Hello";
        let mode_name_2 = "World";

        let obj = MID2601rev1 {
            number_of_modes: 2,
            mode_data: vec![
                ModeData {
                    mode_id: 1,
                    mode_name_size: mode_name_1.len() as u16,
                    mode_name: mode_name_1.into(),
                },
                ModeData {
                    mode_id: 2,
                    mode_name_size: mode_name_2.len() as u16,
                    mode_name: mode_name_2.into(),
                }
            ]
        };

        let encoded = encode::encode(&obj);

        assert_eq!(encoded, Ok("002000105Hello000205World".into()));
    }

    #[test]
    fn decode_2601() {
        let mode_name_1 = "Hello";
        let mode_name_2 = "World";

        let str = "002000105Hello000205World";

        let decoded: decode::Result<MID2601rev1> = decode::decode(str.as_bytes());


        let obj = MID2601rev1 {
            number_of_modes: 2,
            mode_data: vec![
                ModeData {
                    mode_id: 1,
                    mode_name_size: mode_name_1.len() as u16,
                    mode_name: mode_name_1.into(),
                },
                ModeData {
                    mode_id: 2,
                    mode_name_size: mode_name_2.len() as u16,
                    mode_name: mode_name_2.into(),
                }
            ]
        };

        assert_eq!(decoded, Ok(obj));
    }
}
