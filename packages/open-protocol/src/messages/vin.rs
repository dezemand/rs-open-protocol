use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 50, revision = 1)]
pub struct MID0050rev1 {
    /// The Vehicle Identification Number (VIN) to be sent to the controller
    #[open_protocol_field(length = 25)]
    pub vin_number: String,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 51, revision = 1)]
pub struct MID0051rev1 {
    // Subscription for VIN and other identifiers
    // No fields for this revision
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 52, revision = 1)]
pub struct MID0052rev1 {
    /// The current Vehicle Identification Number (VIN)
    #[open_protocol_field(length = 25)]
    pub vin_number: String,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 52, revision = 2)]
pub struct MID0052rev2 {
    /// The current Vehicle Identification Number (VIN)
    #[open_protocol_field(length = 25)]
    pub vin_number: String,

    /// Additional identifier parts
    #[open_protocol_field(list, amount = 3, length = 25)]
    pub additional_identifiers: Vec<String>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 53, revision = 1)]
pub struct MID0053rev1 {
    // Acknowledge receipt of the VIN number
    // No fields for this revision
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 54, revision = 1)]
pub struct MID0054rev1 {
    // Unsubscribe from the current tightening identifiers
    // No fields for this revision
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 54, revision = 2)]
pub struct MID0054rev2 {
    // Unsubscribe from the current tightening identifiers
    // No fields for this revision
}