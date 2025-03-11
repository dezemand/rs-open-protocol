pub mod messages;
pub mod types;
pub mod enums;
mod messages_macro;

pub use open_protocol_codec::encode;
pub use open_protocol_codec::decode;
pub use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode};

use open_protocol_codec::decode::{Decode};
use open_protocol_codec::encode::{Encode};
use crate::messages_macro::open_protocol_messages;
use crate::messages::{
    alarm,
    communication,
    job,
    keep_alive,
    link_communication,
    mode,
    multi_spindle_result,
    multi_spindle_status,
    parameter_set,
    powermacs_result,
    result,
    tightening_result,
    time,
    tool,
    user_interface,
    vin
};

#[derive(Debug, Default, Eq, PartialEq, Clone, OpenProtocolEncode, OpenProtocolDecode)]
pub struct Header {
    #[open_protocol_field(length = 4)]
    pub length: u16,
    #[open_protocol_field(length = 4)]
    pub mid: u16,
    #[open_protocol_field(length = 3)]
    pub revision: Option<u16>,
    #[open_protocol_field(length = 1)]
    pub no_ack_flag: Option<bool>,
    #[open_protocol_field(length = 2)]
    pub station_id: Option<u8>,
    #[open_protocol_field(length = 2)]
    pub spindle_id: Option<u8>,
    #[open_protocol_field(length = 2)]
    pub sequence_number: Option<u8>,
    #[open_protocol_field(length = 1)]
    pub message_parts: Option<u8>,
    #[open_protocol_field(length = 1)]
    pub message_part_number: Option<u8>
}

impl Header {
    pub fn revision_number(&self) -> u16 {
        match self.revision {
            None => 1,
            Some(0) => 1,
            Some(1) => 1,
            Some(n) => n
        }
    }
}

open_protocol_messages!(
    MID 9998 revision 1: link_communication::MID9998rev1,
    MID 9997 revision 1: link_communication::MID9997rev1,

    MID 0001 revision 1: communication::MID0001rev7,
    MID 0001 revision 7: communication::MID0001rev7,
    MID 0002 revision 1: communication::MID0002rev1,
    MID 0002 revision 2: communication::MID0002rev2,
    MID 0002 revision 3: communication::MID0002rev3,
    MID 0002 revision 4: communication::MID0002rev4,
    MID 0002 revision 5: communication::MID0002rev5,
    MID 0002 revision 6: communication::MID0002rev6,
    MID 0002 revision 7: communication::MID0002rev7,
    MID 0003 revision 1: communication::MID0003rev1,
    MID 0004 revision 1: communication::MID0004rev1,
    MID 0004 revision 2: communication::MID0004rev2,
    MID 0005 revision 1: communication::MID0005rev1,
    MID 0006 revision 1: communication::MID0006rev1,
    MID 0008 revision 1: communication::MID0008rev1,
    MID 0009 revision 1: communication::MID0009rev1,

    MID 0010 revision 1: parameter_set::MID0010rev1,
    MID 0011 revision 1: parameter_set::MID0011rev1,
    MID 0011 revision 2: parameter_set::MID0011rev2,
    MID 0011 revision 3: parameter_set::MID0011rev3,
    MID 0011 revision 4: parameter_set::MID0011rev4,
    MID 0012 revision 1: parameter_set::MID0012rev1,
    MID 0013 revision 1: parameter_set::MID0013rev1,
    MID 0013 revision 2: parameter_set::MID0013rev2,
    MID 0014 revision 1: parameter_set::MID0014rev1,
    MID 0015 revision 1: parameter_set::MID0015rev1,
    MID 0015 revision 2: parameter_set::MID0015rev2,
    MID 0016 revision 1: parameter_set::MID0016rev1,
    MID 0017 revision 1: parameter_set::MID0017rev1,
    MID 0018 revision 1: parameter_set::MID0018rev1,
    MID 0019 revision 1: parameter_set::MID0019rev1,
    MID 0019 revision 2: parameter_set::MID0019rev2,
    MID 0020 revision 1: parameter_set::MID0020rev1,
    MID 0021 revision 1: parameter_set::MID0021rev1,
    MID 0022 revision 1: parameter_set::MID0022rev1,

    MID 2600 revision 1: mode::MID2600rev1,
    MID 2601 revision 1: mode::MID2601rev1,
    MID 2602 revision 1: mode::MID2602rev1,
    MID 2603 revision 1: mode::MID2603rev1,
    MID 2604 revision 1: mode::MID2604rev1,
    MID 2605 revision 1: mode::MID2605rev1,
    MID 2606 revision 1: mode::MID2606rev1,

    MID 0030 revision 1: job::MID0030rev1,
    MID 0031 revision 1: job::MID0031rev1,
    MID 0031 revision 2: job::MID0031rev2,
    MID 0032 revision 1: job::MID0032rev1,
    MID 0032 revision 2: job::MID0032rev2,
    MID 0033 revision 1: job::MID0033rev1,
    MID 0034 revision 1: job::MID0034rev1,
    MID 0035 revision 1: job::MID0035rev1,
    MID 0036 revision 1: job::MID0036rev1,
    MID 0037 revision 1: job::MID0037rev1,
    MID 0038 revision 1: job::MID0038rev1,
    MID 0038 revision 2: job::MID0038rev2,
    MID 0039 revision 1: job::MID0039rev1,
    MID 0039 revision 2: job::MID0039rev2,

    MID 0040 revision 6: tool::MID0040rev6,
    MID 0041 revision 1: tool::MID0041rev1,
    MID 0041 revision 2: tool::MID0041rev2,
    MID 0042 revision 2: tool::MID0042rev2,
    MID 0043 revision 2: tool::MID0043rev2,
    MID 0044 revision 1: tool::MID0044rev1,
    MID 0045 revision 1: tool::MID0045rev1,
    MID 0045 revision 2: tool::MID0045rev2,
    MID 0046 revision 1: tool::MID0046rev1,
    MID 0047 revision 1: tool::MID0047rev1,
    MID 0048 revision 1: tool::MID0048rev1,

    MID 0050 revision 1: vin::MID0050rev1,
    MID 0051 revision 1: vin::MID0051rev1,
    MID 0052 revision 1: vin::MID0052rev1,
    MID 0052 revision 2: vin::MID0052rev2,
    MID 0053 revision 1: vin::MID0053rev1,
    MID 0054 revision 1: vin::MID0054rev1,
    MID 0054 revision 2: vin::MID0054rev2,

    MID 0060 revision 1: tightening_result::MID0060rev1,
    MID 0061 revision 1: tightening_result::MID0061rev1,
    MID 0061 revision 2: tightening_result::MID0061rev2,
    MID 0061 revision 3: tightening_result::MID0061rev3,
    MID 0062 revision 1: tightening_result::MID0062rev1,
    MID 0063 revision 1: tightening_result::MID0063rev1,
    MID 0064 revision 1: tightening_result::MID0064rev1,
    MID 0065 revision 1: tightening_result::MID0065rev1,
    MID 0066 revision 1: tightening_result::MID0066rev1,
    MID 0066 revision 2: tightening_result::MID0066rev2,
    MID 0067 revision 1: tightening_result::MID0067rev1,
    MID 0900 revision 1: tightening_result::MID0900rev1,
    MID 0900 revision 2: tightening_result::MID0900rev2,
    MID 0900 revision 3: tightening_result::MID0900rev3,
    MID 0901 revision 1: tightening_result::MID0901rev1,
    MID 0901 revision 2: tightening_result::MID0901rev2,
    MID 0901 revision 3: tightening_result::MID0901rev3,
    MID 0902 revision 1: tightening_result::MID0902rev1,

    MID 1201 revision 1: result::MID1201rev1,
    MID 1201 revision 2: result::MID1201rev2,
    MID 1201 revision 3: result::MID1201rev3,
    MID 1202 revision 1: result::MID1202rev1,
    MID 1202 revision 2: result::MID1202rev2,
    MID 1203 revision 1: result::MID1203rev1,

    MID 0070 revision 1: alarm::MID0070rev1,
    MID 0071 revision 1: alarm::MID0071rev1,
    MID 0072 revision 1: alarm::MID0072rev1,
    MID 0073 revision 1: alarm::MID0073rev1,
    MID 0074 revision 1: alarm::MID0074rev1,
    MID 0075 revision 1: alarm::MID0075rev1,
    MID 0076 revision 1: alarm::MID0076rev1,
    MID 0077 revision 1: alarm::MID0077rev1,
    MID 0078 revision 1: alarm::MID0078rev1,
    MID 1000 revision 1: alarm::MID1000rev1,
    MID 1001 revision 1: alarm::MID1001rev1,

    MID 0080 revision 1: time::MID0080rev1,
    MID 0081 revision 1: time::MID0081rev1,
    MID 0082 revision 1: time::MID0082rev1,

    MID 0090 revision 1: multi_spindle_status::MID0090rev1,
    MID 0091 revision 1: multi_spindle_status::MID0091rev1,
    MID 0092 revision 1: multi_spindle_status::MID0092rev1,
    MID 0093 revision 1: multi_spindle_status::MID0093rev1,

    MID 0100 revision 1: multi_spindle_result::MID0100rev1,
    MID 0101 revision 1: multi_spindle_result::MID0101rev1,
    MID 0102 revision 1: multi_spindle_result::MID0102rev1,
    MID 0103 revision 1: multi_spindle_result::MID0103rev1,
    MID 0104 revision 1: multi_spindle_result::MID0104rev1,

    MID 0105 revision 1: powermacs_result::MID0105rev1,
    MID 0106 revision 1: powermacs_result::MID0106rev1,
    MID 0107 revision 1: powermacs_result::MID0107rev1,
    MID 0108 revision 1: powermacs_result::MID0108rev1,
    MID 0109 revision 1: powermacs_result::MID0109rev1,

    MID 0110 revision 1: user_interface::MID0110rev1,
    MID 0111 revision 1: user_interface::MID0111rev1,
    MID 0113 revision 1: user_interface::MID0113rev1,

    // job_advanced;
    // multiple_identifiers;
    // io_interface;
    // plc_user_data;
    // selector;
    // tool_location_system;
    // controller;
    // statistic;
    // automatic_manual_mode;
    // open_protocol_commands_disabled;
    // audi;
    // motor_turning;

    MID 9999 revision 1: keep_alive::MID9999rev1,
);

#[cfg(test)]
mod tests {
    use open_protocol_codec::{encode, decode};
    use open_protocol_codec::encode::Encoder;
    use super::*;

    #[test]
    fn parse_header() {
        let header_packet = "00530071            ";

        let result = Header::decode(&mut header_packet.into());

        assert_eq!(result, Ok(Header { length: 53, mid: 71, ..Default::default() }));
    }

    #[test]
    fn parse_header_small() {
        let header_packet = "00530071";

        let result = Header::decode(&mut header_packet.into());

        assert_eq!(result, Err(decode::Error::OutOfRightBound { request: 11, size: 8 }));
    }

    #[test]
    fn write_header() {
        let header = Header { length: 53, mid: 71, revision: Some(7), ..Default::default() };

        let result = encode::encode(&header);

        assert_eq!(result, Ok("00530071007         ".to_string()));
    }

    #[test]
    fn read_payload() {
        let packet = "00380011001         005001002003010020\0";
        let (header, payload) = Message::decode_message(&mut packet.into()).unwrap();

        assert_eq!(header, Header {
            length: 38,
            mid: 11,
            revision: Some(1),
            no_ack_flag: None,
            station_id: None,
            spindle_id: None,
            sequence_number: None,
            message_parts: None,
            message_part_number: None
        });

        assert_eq!(payload.mid_revision(), (11, 1));

        assert_eq!(payload, Message::MID0011rev1(parameter_set::MID0011rev1 {
            number_of_parameter_sets: 5,
            parameter_set_ids: vec![1, 2, 3, 10, 20]
        }));
    }

    #[test]
    fn encode_payload() {
        let payload = Message::MID0011rev1(parameter_set::MID0011rev1 {
            number_of_parameter_sets: 5,
            parameter_set_ids: vec![1, 2, 3, 10, 20]
        });

        let mut encoder = Encoder::new();
        payload.encode_payload(&mut encoder).unwrap();
        let packet = encoder.to_string();

        assert_eq!(packet, "005001002003010020");
    }
}
