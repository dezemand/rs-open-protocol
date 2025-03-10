use chrono::{DateTime, Local};
use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

/// A subscription for the multi-spindle status.
/// For Power Focus, the subscription must be addressed to the sync Master.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 90, revision = 1)]
pub struct MID0090rev1 {
    // No additional fields for this MID.
}

/// The multi-spindle status is sent after each sync tightening.
/// It contains both the common status of the multiple and the individual status of each spindle.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 91, revision = 1)]
pub struct MID0091rev1 {
    /// The number of spindles in the tightening.
    #[open_protocol_field(length = 2)]
    pub number_of_spindles: u8,

    /// The number of spindles currently running in the multiple.
    #[open_protocol_field(length = 2)]
    pub spindles_running: u8,

    /// A unique ID for each sync tightening result.
    #[open_protocol_field(length = 5)]
    pub sync_tightening_id: u32,

    /// Timestamp for the multi-spindle status (YYYY-MM-DD:HH:MM:SS).
    #[open_protocol_field(length = 19)]
    pub timestamp: DateTime<Local>,

    /// The overall status of the sync tightening (1=OK, 0=NOK).
    #[open_protocol_field(length = 1)]
    pub sync_overall_status: u8,

    /// The status of each spindle in the multiple.
    #[open_protocol_field(list, amount = "number_of_spindles", length = 5)]
    pub spindle_statuses: Vec<SpindleStatus>,
}

/// Acknowledges receipt of MID 0091 Multi-Spindle Status.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 92, revision = 1)]
pub struct MID0092rev1 {
    // No additional fields for this MID.
}

/// Cancels a previously subscribed multi-spindle status notification.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 93, revision = 1)]
pub struct MID0093rev1 {
    // No additional fields for this MID.
}

/// Status of an individual spindle in a multi-spindle tightening.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub struct SpindleStatus {
    /// The spindle number within the multiple.
    #[open_protocol_field(length = 2)]
    pub spindle_number: u8,

    /// The channel ID of the spindle.
    #[open_protocol_field(length = 2)]
    pub channel_id: u8,

    /// The overall status of the tightening for this spindle (0=NOK, 1=OK).
    #[open_protocol_field(length = 1)]
    pub overall_status: u8,
}
