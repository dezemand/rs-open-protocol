use chrono::{DateTime, Local};
use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

/// A subscription for the multi-spindle result.
/// For Power Focus, the subscription must be addressed to the sync Master.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 100, revision = 1)]
pub struct MID0100rev1 {
    // No additional fields for this MID.
}

/// This message is sent after each sync tightening.
/// It contains the final result of the tightening for all spindles involved.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 101, revision = 1)]
pub struct MID0101rev1 {
    /// The number of spindles or presses involved in the tightening.
    #[open_protocol_field(length = 2)]
    pub number_of_spindles: u8,

    /// The number of spindles currently running in the multiple.
    #[open_protocol_field(length = 2)]
    pub spindles_running: u8,

    /// A unique ID for each sync tightening result.
    #[open_protocol_field(length = 5)]
    pub sync_tightening_id: u32,

    /// The overall status of the sync tightening (1=OK, 0=NOK).
    #[open_protocol_field(length = 1)]
    pub sync_overall_status: u8,

    /// The VIN (Vehicle Identification Number) associated with the tightening.
    #[open_protocol_field(length = 25)]
    pub vin_number: String,

    /// The job ID associated with the tightening.
    #[open_protocol_field(length = 2)]
    pub job_id: u8,

    /// The parameter set ID used in the tightening.
    #[open_protocol_field(length = 3)]
    pub parameter_set_id: u16,

    /// The total number of tightenings required in the batch.
    #[open_protocol_field(length = 4)]
    pub batch_size: u16,

    /// The current batch counter.
    #[open_protocol_field(length = 4)]
    pub batch_counter: u16,

    /// The batch status (0=NOK, 1=OK, 2=Not used, 3=Running).
    #[open_protocol_field(length = 1)]
    pub batch_status: u8,

    /// Timestamp for the multi-spindle result (YYYY-MM-DD:HH:MM:SS).
    #[open_protocol_field(length = 19)]
    pub timestamp: DateTime<Local>,

    /// Status of each spindle in the multi-spindle tightening.
    #[open_protocol_field(list, amount = "number_of_spindles", length = 5)]
    pub spindle_statuses: Vec<SpindleResult>,
}

/// Acknowledges receipt of MID 0101 Multi-Spindle Result.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 102, revision = 1)]
pub struct MID0102rev1 {
    // No additional fields for this MID.
}

/// Cancels a previously subscribed multi-spindle result notification.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 103, revision = 1)]
pub struct MID0103rev1 {
    // No additional fields for this MID.
}

/// Requests an old multi-spindle tightening result by its unique ID.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 104, revision = 1)]
pub struct MID0104rev1 {
    /// The ID of the requested old multi-spindle result.
    #[open_protocol_field(length = 10)]
    pub old_sync_tightening_id: u32,
}

/// Status and data for an individual spindle in a multi-spindle tightening result.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub struct SpindleResult {
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
