use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

/// 5.1.1 MID 9998 Communication acknowledge error
/// This message is used in conjunction with the use of header sequence number.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 9998, revision = 1)]
pub struct MID9998rev1 {
    /// MID number to which the acknowledgment error belongs to
    #[open_protocol_field(length = 4)]
    pub mid_number: u16,

    /// Error code for the sent message
    #[open_protocol_field(length = 4)]
    pub error_code: u16,
}

/// 5.1.2 MID 9997 Communication acknowledge
/// This message is used in conjunction with the use of header sequence number.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 9997, revision = 1)]
pub struct MID9997rev1 {
    /// Acknowledged MID number
    #[open_protocol_field(length = 4)]
    pub mid_number: u16,
}