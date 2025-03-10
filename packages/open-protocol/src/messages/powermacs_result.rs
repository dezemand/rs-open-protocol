use chrono::{DateTime, Local};
use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

/// Subscribe to the last PowerMACS tightening result data.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 105, revision = 1)]
pub struct MID0105rev1 {
    // No additional fields for this MID.
}

/// The last PowerMACS tightening result station data.
/// This contains summary information about the tightening station.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 106, revision = 1)]
pub struct MID0106rev1 {
    /// The station number where the tightening occurred.
    #[open_protocol_field(length = 4)]
    pub station_number: u16,

    /// The name of the station.
    #[open_protocol_field(length = 25)]
    pub station_name: String,

    /// The batch size for this station.
    #[open_protocol_field(length = 4)]
    pub batch_size: u16,

    /// The current batch counter.
    #[open_protocol_field(length = 4)]
    pub batch_counter: u16,

    /// The timestamp when the batch started.
    #[open_protocol_field(length = 19)]
    pub batch_start_time: DateTime<Local>,

    /// The batch status (0=NOK, 1=OK, 2=Not used, 3=Running).
    #[open_protocol_field(length = 1)]
    pub batch_status: u8,

    /// The tightening ID of the last operation.
    #[open_protocol_field(length = 10)]
    pub tightening_id: u32,
}

/// The last PowerMACS tightening result bolt data.
/// This contains details about the individual bolts involved in the tightening.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 107, revision = 1)]
pub struct MID0107rev1 {
    /// The number of bolts included in the tightening operation.
    #[open_protocol_field(length = 3)]
    pub number_of_bolts: u16,

    /// Data for each bolt in the tightening.
    #[open_protocol_field(list, amount = "number_of_bolts", length = 10)]
    pub bolt_data: Vec<BoltData>,
}

/// Details of an individual bolt in the tightening result.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub struct BoltData {
    /// The bolt number.
    #[open_protocol_field(length = 4)]
    pub bolt_number: u32,

    /// The torque applied to the bolt.
    #[open_protocol_field(length = 6)]
    pub bolt_torque: u32,

    /// The angle applied to the bolt.
    #[open_protocol_field(length = 5)]
    pub bolt_angle: u16,
}

/// Acknowledge receipt of PowerMACS tightening result data.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 108, revision = 1)]
pub struct MID0108rev1 {
    // No additional fields for this MID.
}

/// Unsubscribe from the PowerMACS tightening result data.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 109, revision = 1)]
pub struct MID0109rev1 {
    // No additional fields for this MID.
}