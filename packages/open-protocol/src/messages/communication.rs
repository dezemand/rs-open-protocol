use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};
use crate::enums::error::ErrorCode;

#[derive(Debug, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub enum KeepAlive {
    #[open_protocol_value(number = 0)]
    Use,
    #[open_protocol_value(number = 1)]
    Ignore,
}

#[derive(
    Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage,
)]
#[open_protocol_message(MID = 0001, revision = 7)]
pub struct MID0001rev7 {
    #[open_protocol_field(number = 1, length = 1)]
    pub keep_alive: Option<KeepAlive>,
}

#[derive(
    Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage,
)]
#[open_protocol_message(MID = 0002, revision = 1)]
pub struct MID0002rev1 {
    #[open_protocol_field(number = 1, length = 4)]
    pub cell_id: u16,
    #[open_protocol_field(number = 2, length = 2)]
    pub channel_id: u8,
    #[open_protocol_field(number = 3, length = 25)]
    pub controller_name: String,
}

#[derive(
    Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage,
)]
#[open_protocol_message(MID = 0002, revision = 2)]
pub struct MID0002rev2 {
    #[open_protocol_field(number = 1, length = 4)]
    pub cell_id: u16,
    #[open_protocol_field(number = 2, length = 2)]
    pub channel_id: u8,
    #[open_protocol_field(number = 3, length = 25)]
    pub controller_name: String,
    #[open_protocol_field(number = 4, length = 3)]
    pub supplier_code: String,
}

#[derive(
    Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage,
)]
#[open_protocol_message(MID = 0002, revision = 3)]
pub struct MID0002rev3 {
    #[open_protocol_field(number = 1, length = 4)]
    pub cell_id: u16,
    #[open_protocol_field(number = 2, length = 2)]
    pub channel_id: u8,
    #[open_protocol_field(number = 3, length = 25)]
    pub controller_name: String,
    #[open_protocol_field(number = 4, length = 3)]
    pub supplier_code: String,
    #[open_protocol_field(number = 5, length = 19)]
    pub open_protocol_version: String,
    #[open_protocol_field(number = 6, length = 19)]
    pub controller_software_version: String,
    #[open_protocol_field(number = 7, length = 19)]
    pub tool_software_version: String,
}

#[derive(
    Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage,
)]
#[open_protocol_message(MID = 0002, revision = 4)]
pub struct MID0002rev4 {
    #[open_protocol_field(number = 1, length = 4)]
    pub cell_id: u16,
    #[open_protocol_field(number = 2, length = 2)]
    pub channel_id: u8,
    #[open_protocol_field(number = 3, length = 25)]
    pub controller_name: String,
    #[open_protocol_field(number = 4, length = 3)]
    pub supplier_code: String,
    #[open_protocol_field(number = 5, length = 19)]
    pub open_protocol_version: String,
    #[open_protocol_field(number = 6, length = 19)]
    pub controller_software_version: String,
    #[open_protocol_field(number = 7, length = 19)]
    pub tool_software_version: String,
    #[open_protocol_field(number = 8, length = 24)]
    pub rbu_type: String,
    #[open_protocol_field(number = 9, length = 10)]
    pub controller_serial_number: String,
}

#[derive(
    Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage,
)]
#[open_protocol_message(MID = 0002, revision = 5)]
pub struct MID0002rev5 {
    #[open_protocol_field(number = 1, length = 4)]
    pub cell_id: u16,
    #[open_protocol_field(number = 2, length = 2)]
    pub channel_id: u8,
    #[open_protocol_field(number = 3, length = 25)]
    pub controller_name: String,
    #[open_protocol_field(number = 4, length = 3)]
    pub supplier_code: String,
    #[open_protocol_field(number = 5, length = 19)]
    pub open_protocol_version: String,
    #[open_protocol_field(number = 6, length = 19)]
    pub controller_software_version: String,
    #[open_protocol_field(number = 7, length = 19)]
    pub tool_software_version: String,
    #[open_protocol_field(number = 8, length = 24)]
    pub rbu_type: String,
    #[open_protocol_field(number = 9, length = 10)]
    pub controller_serial_number: String,
    #[open_protocol_field(number = 10, length = 3)]
    pub system_type: u16,
    #[open_protocol_field(number = 11, length = 3)]
    pub system_subtype: u16,
}

#[derive(
    Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage,
)]
#[open_protocol_message(MID = 0002, revision = 6)]
pub struct MID0002rev6 {
    #[open_protocol_field(number = 1, length = 4)]
    pub cell_id: u16,
    #[open_protocol_field(number = 2, length = 2)]
    pub channel_id: u8,
    #[open_protocol_field(number = 3, length = 25)]
    pub controller_name: String,
    #[open_protocol_field(number = 4, length = 3)]
    pub supplier_code: String,
    #[open_protocol_field(number = 5, length = 19)]
    pub open_protocol_version: String,
    #[open_protocol_field(number = 6, length = 19)]
    pub controller_software_version: String,
    #[open_protocol_field(number = 7, length = 19)]
    pub tool_software_version: String,
    #[open_protocol_field(number = 8, length = 24)]
    pub rbu_type: String,
    #[open_protocol_field(number = 9, length = 10)]
    pub controller_serial_number: String,
    #[open_protocol_field(number = 10, length = 3)]
    pub system_type: u16,
    #[open_protocol_field(number = 11, length = 3)]
    pub system_subtype: u16,
    #[open_protocol_field(number = 12, length = 1)]
    pub sequence_number_supported: bool,
    #[open_protocol_field(number = 13, length = 1)]
    pub linking_handling_supported: bool,
    #[open_protocol_field(number = 14, length = 10)]
    pub station_or_cell_id: u32,
    #[open_protocol_field(number = 15, length = 25)]
    pub station_or_cell_name: String,
    #[open_protocol_field(number = 16, length = 1)]
    pub client_id: u8,
}

#[derive(
    Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage,
)]
#[open_protocol_message(MID = 0002, revision = 7)]
pub struct MID0002rev7 {
    #[open_protocol_field(number = 1, length = 4)]
    pub cell_id: u16,
    #[open_protocol_field(number = 2, length = 2)]
    pub channel_id: u8,
    #[open_protocol_field(number = 3, length = 25)]
    pub controller_name: String,
    #[open_protocol_field(number = 4, length = 3)]
    pub supplier_code: String,
    #[open_protocol_field(number = 5, length = 19)]
    pub open_protocol_version: String,
    #[open_protocol_field(number = 6, length = 19)]
    pub controller_software_version: String,
    #[open_protocol_field(number = 7, length = 19)]
    pub tool_software_version: String,
    #[open_protocol_field(number = 8, length = 24)]
    pub rbu_type: String,
    #[open_protocol_field(number = 9, length = 10)]
    pub controller_serial_number: String,
    #[open_protocol_field(number = 10, length = 3)]
    pub system_type: u16,
    #[open_protocol_field(number = 11, length = 3)]
    pub system_subtype: u16,
    #[open_protocol_field(number = 12, length = 1)]
    pub sequence_number_supported: bool,
    #[open_protocol_field(number = 13, length = 1)]
    pub linking_handling_supported: bool,
    #[open_protocol_field(number = 14, length = 10)]
    pub station_or_cell_id: u32,
    #[open_protocol_field(number = 15, length = 25)]
    pub station_or_cell_name: String,
    #[open_protocol_field(number = 16, length = 1)]
    pub client_id: u8,
    #[open_protocol_field(number = 17, length = 1)]
    pub keep_alive: Option<KeepAlive>,
}

/// # 5.2.3 MID 0003 Application Communication stop
/// This message disables the communication. The controller will stop to respond to any commands
/// except for MID 0001 Communication start after receiving this command.
#[derive(
    Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage,
)]
#[open_protocol_message(MID = 0003, revision = 1)]
pub struct MID0003rev1 {}

/// # 5.2.4 MID 0004 Application Communication negative acknowledge
/// This message is used by the controller when a request, command or subscription for any reason
/// has not been performed. The data field contains the message ID of the message request that
/// failed as well as an error code.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0004, revision = 1)]
pub struct MID0004rev1 {
    #[open_protocol_field(length = 4)]
    pub mid: u16,
    #[open_protocol_field(length = 2)]
    pub error_code: ErrorCode,
}

/// # 5.2.4 MID 0004 Application Communication negative acknowledge
/// This message is used by the controller when a request, command or subscription for any reason
/// has not been performed. The data field contains the message ID of the message request that
/// failed as well as an error code.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0004, revision = 2)]
pub struct MID0004rev2 {
    #[open_protocol_field(length = 4)]
    pub mid: u16,
    #[open_protocol_field(length = 3)]
    pub error_code: ErrorCode,
}

/// # 5.2.5 MID 0005 Application Communication positive acknowledge
/// This message is used by the controller to confirm that the latest command, request or
/// subscription sent by the integrator was accepted. The data field contains the MID of the
/// request accepted if the special MIDs for request or subscription are used.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0005, revision = 1)]
pub struct MID0005rev1 {
    #[open_protocol_field(length = 4)]
    pub mid: u16,
}

/// # 5.2.6 MID 0006 Application data message request
/// Do a request for data. This message is used for ALL request handling.
/// When used it substitutes the use of all MID special request messages.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0006, revision = 1)]
pub struct MID0006rev1 {
    #[open_protocol_field(length = 4)]
    requested_mid: u16,
    #[open_protocol_field(length = 3)]
    wanted_revision: u16,
    #[open_protocol_field(length = 2)]
    extra_data_length: u8,
    #[open_protocol_field(list, amount = "extra_data_length", length = 1)]
    extra_data: Vec<u8>,
}

/// # 5.2.7 MID 0008 Application data message subscription
/// Start a subscription of data.
/// This message is used for ALL subscription handling.
/// When used it substitutes the use of all MID special subscription messages.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 8, revision = 1)]
pub struct MID0008rev1 {
    /// The MID ID to be subscribed for. Can be used for ALL subscription handling.
    #[open_protocol_field(length = 4)]
    pub subscription_mid: u16,

    /// The revision of the MID to subscribe for.
    #[open_protocol_field(length = 3)]
    pub wanted_revision: u16,

    /// The length of the extra data field.
    #[open_protocol_field(length = 2)]
    pub extra_data_length: u16,

    /// The extra data field (variable length).
    #[open_protocol_field(list, amount = "extra_data_length", length = 1)]
    pub extra_data: Vec<u8>,
}

/// # 5.2.8 MID 0009 Application Data Message unsubscribe.
/// Unsubscribe from a previously subscribed MID.
/// This message is used for ALL unsubscriptions.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 9, revision = 1)]
pub struct MID0009rev1 {
    /// The MID ID to be unsubscribed for.
    #[open_protocol_field(length = 4)]
    pub unsubscription_mid: u16,

    /// The revision of the MIDs extra data that is subscribed for.
    #[open_protocol_field(length = 3)]
    pub extra_data_revision: u16,

    /// The length of the extra data field.
    #[open_protocol_field(length = 2)]
    pub extra_data_length: u16,

    /// The extra data field (variable length).
    #[open_protocol_field(list, amount = "extra_data_length", length = 1)]
    pub extra_data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_protocol_codec::{decode, encode};

    #[test]
    fn encode_mid0001rev7_with_keep_alive() {
        let message = MID0001rev7 {
            keep_alive: Some(KeepAlive::Use),
        };

        let output = encode::encode(&message).unwrap();

        assert_eq!(output, "010".to_string());
    }

    #[test]
    fn encode_mid0001rev7_empty() {
        let message = MID0001rev7 { keep_alive: None };

        let output = encode::encode(&message).unwrap();

        assert_eq!(output, "".to_string());
    }

    #[test]
    fn parse_mid0002_rev1() {
        let message = "010001020103Airbag1                  ";

        let parsed = decode::decode::<MID0002rev1>(message.as_bytes());

        assert_eq!(
            parsed,
            Ok(MID0002rev1 {
                cell_id: 1,
                channel_id: 1,
                controller_name: "Airbag1".into()
            })
        )
    }

    #[test]
    fn write_mid0002_rev1() {
        let message = MID0002rev1 {
            cell_id: 1,
            channel_id: 1,
            controller_name: "Airbag1".into(),
        };

        let encoded = encode::encode(&message);

        assert_eq!(
            encoded,
            Ok("010001020103Airbag1                  ".to_string())
        );
    }
}
