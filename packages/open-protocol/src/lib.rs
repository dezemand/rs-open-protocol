pub mod messages;
pub mod types;
pub mod enums;
mod messages_macro;

use open_protocol_codec::decode::{Decode};
use open_protocol_codec::encode::{Encode};
use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode};
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
    (9998, 1) => link_communication::MID9998rev1,
    (9997, 1) => link_communication::MID9997rev1,

    (0001, 1) => communication::MID0001rev7,
    (0001, 7) => communication::MID0001rev7,
    (0002, 1) => communication::MID0002rev1,
    (0002, 2) => communication::MID0002rev2,
    (0002, 3) => communication::MID0002rev3,
    (0002, 4) => communication::MID0002rev4,
    (0002, 5) => communication::MID0002rev5,
    (0002, 6) => communication::MID0002rev6,
    (0002, 7) => communication::MID0002rev7,
    (0003, 1) => communication::MID0003rev1,
    (0004, 1) => communication::MID0004rev1,
    (0004, 2) => communication::MID0004rev2,
    (0005, 1) => communication::MID0005rev1,
    (0006, 1) => communication::MID0006rev1,
    (0008, 1) => communication::MID0008rev1,
    (0009, 1) => communication::MID0009rev1,

    (0010, 1) => parameter_set::MID0010rev1,
    (0011, 1) => parameter_set::MID0011rev1,
    (0011, 2) => parameter_set::MID0011rev2,
    (0011, 3) => parameter_set::MID0011rev3,
    (0011, 4) => parameter_set::MID0011rev4,
    (0012, 1) => parameter_set::MID0012rev1,
    (0013, 1) => parameter_set::MID0013rev1,
    (0013, 2) => parameter_set::MID0013rev2,
    (0014, 1) => parameter_set::MID0014rev1,
    (0015, 1) => parameter_set::MID0015rev1,
    (0015, 2) => parameter_set::MID0015rev2,
    (0016, 1) => parameter_set::MID0016rev1,
    (0017, 1) => parameter_set::MID0017rev1,
    (0018, 1) => parameter_set::MID0018rev1,
    (0019, 1) => parameter_set::MID0019rev1,
    (0019, 2) => parameter_set::MID0019rev2,
    (0020, 1) => parameter_set::MID0020rev1,
    (0021, 1) => parameter_set::MID0021rev1,
    (0022, 1) => parameter_set::MID0022rev1,

    (2600, 1) => mode::MID2600rev1,
    (2601, 1) => mode::MID2601rev1,
    (2602, 1) => mode::MID2602rev1,
    (2603, 1) => mode::MID2603rev1,
    (2604, 1) => mode::MID2604rev1,
    (2605, 1) => mode::MID2605rev1,
    (2606, 1) => mode::MID2606rev1,

    (0030, 1) => job::MID0030rev1,
    (0031, 1) => job::MID0031rev1,
    (0031, 2) => job::MID0031rev2,
    (0032, 1) => job::MID0032rev1,
    (0032, 2) => job::MID0032rev2,
    (0033, 1) => job::MID0033rev1,
    (0034, 1) => job::MID0034rev1,
    (0035, 1) => job::MID0035rev1,
    (0036, 1) => job::MID0036rev1,
    (0037, 1) => job::MID0037rev1,
    (0038, 1) => job::MID0038rev1,
    (0038, 2) => job::MID0038rev2,
    (0039, 1) => job::MID0039rev1,
    (0039, 2) => job::MID0039rev2,

    (0040, 6) => tool::MID0040rev6,
    (0041, 1) => tool::MID0041rev1,
    (0041, 2) => tool::MID0041rev2,
    (0042, 2) => tool::MID0042rev2,
    (0043, 2) => tool::MID0043rev2,
    (0044, 1) => tool::MID0044rev1,
    (0045, 1) => tool::MID0045rev1,
    (0045, 2) => tool::MID0045rev2,
    (0046, 1) => tool::MID0046rev1,
    (0047, 1) => tool::MID0047rev1,
    (0048, 1) => tool::MID0048rev1,

    (0050, 1) => vin::MID0050rev1,
    (0051, 1) => vin::MID0051rev1,
    (0052, 1) => vin::MID0052rev1,
    (0052, 2) => vin::MID0052rev2,
    (0053, 1) => vin::MID0053rev1,
    (0054, 1) => vin::MID0054rev1,
    (0054, 2) => vin::MID0054rev2,

    (0060, 1) => tightening_result::MID0060rev1,
    (0061, 1) => tightening_result::MID0061rev1,
    (0061, 2) => tightening_result::MID0061rev2,
    (0061, 3) => tightening_result::MID0061rev3,
    (0062, 1) => tightening_result::MID0062rev1,
    (0063, 1) => tightening_result::MID0063rev1,
    (0064, 1) => tightening_result::MID0064rev1,
    (0065, 1) => tightening_result::MID0065rev1,
    (0066, 1) => tightening_result::MID0066rev1,
    (0066, 2) => tightening_result::MID0066rev2,
    (0067, 1) => tightening_result::MID0067rev1,
    (0900, 1) => tightening_result::MID0900rev1,
    (0900, 2) => tightening_result::MID0900rev2,
    (0900, 3) => tightening_result::MID0900rev3,
    (0901, 1) => tightening_result::MID0901rev1,
    (0901, 2) => tightening_result::MID0901rev2,
    (0901, 3) => tightening_result::MID0901rev3,
    (0902, 1) => tightening_result::MID0902rev1,

    (1201, 1) => result::MID1201rev1,
    (1201, 2) => result::MID1201rev2,
    (1201, 3) => result::MID1201rev3,
    (1202, 1) => result::MID1202rev1,
    (1202, 2) => result::MID1202rev2,
    (1203, 1) => result::MID1203rev1,

    (0070, 1) => alarm::MID0070rev1,
    (0071, 1) => alarm::MID0071rev1,
    (0072, 1) => alarm::MID0072rev1,
    (0073, 1) => alarm::MID0073rev1,
    (0074, 1) => alarm::MID0074rev1,
    (0075, 1) => alarm::MID0075rev1,
    (0076, 1) => alarm::MID0076rev1,
    (0077, 1) => alarm::MID0077rev1,
    (0078, 1) => alarm::MID0078rev1,
    (1000, 1) => alarm::MID1000rev1,
    (1001, 1) => alarm::MID1001rev1,

    (0080, 1) => time::MID0080rev1,
    (0081, 1) => time::MID0081rev1,
    (0082, 1) => time::MID0082rev1,

    (0090, 1) => multi_spindle_status::MID0090rev1,
    (0091, 1) => multi_spindle_status::MID0091rev1,
    (0092, 1) => multi_spindle_status::MID0092rev1,
    (0093, 1) => multi_spindle_status::MID0093rev1,

    (0100, 1) => multi_spindle_result::MID0100rev1,
    (0101, 1) => multi_spindle_result::MID0101rev1,
    (0102, 1) => multi_spindle_result::MID0102rev1,
    (0103, 1) => multi_spindle_result::MID0103rev1,
    (0104, 1) => multi_spindle_result::MID0104rev1,

    (0105, 1) => powermacs_result::MID0105rev1,
    (0106, 1) => powermacs_result::MID0106rev1,
    (0107, 1) => powermacs_result::MID0107rev1,
    (0108, 1) => powermacs_result::MID0108rev1,
    (0109, 1) => powermacs_result::MID0109rev1,

    (0110, 1) => user_interface::MID0110rev1,
    (0111, 1) => user_interface::MID0111rev1,
    (0113, 1) => user_interface::MID0113rev1,

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

    (9999, 1) => keep_alive::MID9999rev1,
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
        let packet = "00380011001         005001002003010020";
        let (header, payload) = OpenProtocolMessage::decode_message(&mut packet.into()).unwrap();

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

        assert_eq!(payload, OpenProtocolMessage::MID0011rev1(parameter_set::MID0011rev1 {
            number_of_parameter_sets: 5,
            parameter_set_ids: vec![1, 2, 3, 10, 20]
        }));
    }

    #[test]
    fn encode_payload() {
        let payload = OpenProtocolMessage::MID0011rev1(parameter_set::MID0011rev1 {
            number_of_parameter_sets: 5,
            parameter_set_ids: vec![1, 2, 3, 10, 20]
        });

        let mut encoder = Encoder::new();
        payload.encode_payload(&mut encoder).unwrap();
        let packet = encoder.to_string();

        assert_eq!(packet, "005001002003010020");
    }
}
