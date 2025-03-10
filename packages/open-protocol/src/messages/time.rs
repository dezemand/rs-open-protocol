use chrono::{DateTime, Local};
use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

/// This message requests the current time from the controller.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 80, revision = 1)]
pub struct MID0080rev1 {
    // No additional fields for this MID.
}

/// This message contains the current system time from the controller.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 81, revision = 1)]
pub struct MID0081rev1 {
    /// The current system time in the format YYYY-MM-DD:HH:MM:SS.
    #[open_protocol_field(length = 19)]
    pub time: DateTime<Local>,
}

/// This message sets the system time on the controller.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 82, revision = 1)]
pub struct MID0082rev1 {
    /// The new system time to be set in the format YYYY-MM-DD:HH:MM:SS.
    #[open_protocol_field(length = 19)]
    pub time: DateTime<Local>,
}
