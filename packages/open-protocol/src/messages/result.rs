use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 1201, revision = 1)]
pub struct MID1201rev1 {}
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 1201, revision = 2)]
pub struct MID1201rev2 {}
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 1201, revision = 3)]
pub struct MID1201rev3 {}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 1201, revision = 1, type = "request_extra_data")]
pub struct MID1201RequestExtraData {
    /// The index of the requested tightening result. If equal to zero, response will contain
    /// the most recent result.
    #[open_protocol_field(length = 10)]
    pub index: u64,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 1201, revision = 1, type = "subscribe_extra_data")]
pub struct MID1201SubscriptionExtraData {}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 1202, revision = 1)]
pub struct MID1202rev1 {}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 1202, revision = 2)]
pub struct MID1202rev2 {}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 1203, revision = 1)]
pub struct MID1203rev1 {}
