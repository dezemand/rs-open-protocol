use chrono::{DateTime, Local};
use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 40, revision = 6)]
pub struct MID0040rev6 {
    /// The tool number for which the data is being requested
    #[open_protocol_field(length = 4)]
    pub tool_number: u16,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 41, revision = 1)]
pub struct MID0041rev1 {
    /// The serial number of the tool
    #[open_protocol_field(length = 14)]
    pub tool_serial_number: String,

    /// The number of tightenings performed with this tool
    #[open_protocol_field(length = 10)]
    pub number_of_tightenings: u32,

    /// The date of the last calibration of the tool
    #[open_protocol_field(length = 19)]
    pub last_calibration_date: DateTime<Local>,

    /// The serial number of the controller or RBU type
    #[open_protocol_field(length = 10)]
    pub controller_serial_number: String,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 41, revision = 2)]
pub struct MID0041rev2 {
    /// The serial number of the tool
    #[open_protocol_field(length = 14)]
    pub tool_serial_number: String,

    /// The number of tightenings performed with this tool
    #[open_protocol_field(length = 10)]
    pub number_of_tightenings: u32,

    /// The date of the last calibration of the tool
    #[open_protocol_field(length = 19)]
    pub last_calibration_date: DateTime<Local>,

    /// The serial number of the controller or RBU type
    #[open_protocol_field(length = 10)]
    pub controller_serial_number: String,

    /// Calibration value of the tool
    #[open_protocol_field(length = 6)]
    pub calibration_value: u32,

    /// Date of the last service for the tool
    #[open_protocol_field(length = 19)]
    pub last_service_date: DateTime<Local>,

    /// Number of tightenings since the last service
    #[open_protocol_field(length = 10)]
    pub tightenings_since_service: u32,

    /// Type of tool
    #[open_protocol_field(length = 2)]
    pub tool_type: u8,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 42, revision = 2)]
pub struct MID0042rev2 {
    /// The tool number to disable
    #[open_protocol_field(length = 4)]
    pub tool_number: u16,

    /// The type of disable action
    #[open_protocol_field(length = 2)]
    pub disable_type: u8,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 43, revision = 2)]
pub struct MID0043rev2 {
    /// The tool number to enable
    #[open_protocol_field(length = 4)]
    pub tool_number: u16,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 44, revision = 1)]
pub struct MID0044rev1 {
    // Request to allow the tool to be disconnected
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 45, revision = 1)]
pub struct MID0045rev1 {
    /// The unit of the calibration value
    #[open_protocol_field(length = 1)]
    pub calibration_value_unit: u8,

    /// The calibration value to set
    #[open_protocol_field(length = 6)]
    pub calibration_value: u32,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 45, revision = 2)]
pub struct MID0045rev2 {
    /// The unit of the calibration value
    #[open_protocol_field(length = 1)]
    pub calibration_value_unit: u8,

    /// The calibration value to set
    #[open_protocol_field(length = 6)]
    pub calibration_value: u32,

    /// The channel number for calibration
    #[open_protocol_field(length = 2)]
    pub channel_number: u8,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 46, revision = 1)]
pub struct MID0046rev1 {
    /// The tool to be set as the primary tool
    #[open_protocol_field(length = 2)]
    pub primary_tool: u8,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 47, revision = 1)]
pub struct MID0047rev1 {
    /// Type of pairing handling action
    #[open_protocol_field(length = 2)]
    pub pairing_handling_type: u8,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 48, revision = 1)]
pub struct MID0048rev1 {
    /// Current status of the tool pairing
    #[open_protocol_field(length = 2)]
    pub pairing_status: u8,

    /// Timestamp of the status change
    #[open_protocol_field(length = 19)]
    pub timestamp: DateTime<Local>,
}
