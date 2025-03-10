use open_protocol_codec_proc_macro::{OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage};

#[derive(Debug, Eq, PartialEq, Default, OpenProtocolEncode, OpenProtocolDecode)]
pub enum RotationDirection {
    #[default]
    #[open_protocol_value(number=1)]
    ClockWise,
    #[open_protocol_value(number=2)]
    CounterClockWise,
}

#[derive(Debug, Eq, PartialEq, Default, OpenProtocolEncode, OpenProtocolDecode)]
pub enum RelayStatus {
    #[default]
    #[open_protocol_value(number=0)]
    Inactive,
    #[open_protocol_value(number=1)]
    Active,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0010, revision = 1)]
pub struct MID0010rev1 {
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0011, revision = 1)]
pub struct MID0011rev1 {
    /// Three ASCII digits for the number of parameter sets
    #[open_protocol_field(length = 3)]
    pub number_of_parameter_sets: u16,

    /// Three ASCII digits for each parameter set
    #[open_protocol_field(list, amount = "number_of_parameter_sets", length = 3)]
    pub parameter_set_ids: Vec<u16>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0011, revision = 2)]
pub struct MID0011rev2 {
    /// Three ASCII digits for the number of parameter sets
    #[open_protocol_field(length = 3)]
    pub number_of_parameter_sets: u16,

    /// Three ASCII digits for each parameter set
    #[open_protocol_field(list, amount = "number_of_parameter_sets", length = 3)]
    pub parameter_set_ids: Vec<u16>,

    /// Three ASCII digits for each parameter set
    #[open_protocol_field(list, amount = "number_of_parameter_sets", length = 2)]
    pub cycles: Vec<u8>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0011, revision = 3)]
pub struct MID0011rev3 {
    /// Three ASCII digits for the number of parameter sets
    #[open_protocol_field(length = 3)]
    pub number_of_parameter_sets: u16,

    /// Three ASCII digits for each parameter set
    #[open_protocol_field(list, amount = "number_of_parameter_sets", length = 3)]
    pub parameter_set_ids: Vec<u16>,

    /// Three ASCII digits for each parameter set
    #[open_protocol_field(list, amount = "number_of_parameter_sets", length = 2)]
    pub cycles: Vec<u8>,

    /// our ASCII character for strings: “Mset” or “Pset” telling if Pset or Multistage
    #[open_protocol_field(list, amount = "number_of_parameter_sets", length = 4)]
    pub types: Vec<String>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0011, revision = 4)]
pub struct MID0011rev4 {
    /// Three ASCII digits for the number of parameter sets
    #[open_protocol_field(length = 3)]
    pub number_of_parameter_sets: u16,

    /// Three ASCII digits for each parameter set
    #[open_protocol_field(list, amount = "number_of_parameter_sets", length = 3)]
    pub parameter_set_ids: Vec<u16>,

    /// Three ASCII digits for each parameter set
    #[open_protocol_field(list, amount = "number_of_parameter_sets", length = 2)]
    pub cycles: Vec<u8>,

    /// our ASCII character for strings: “Mset” or “Pset” telling if Pset or Multistage
    #[open_protocol_field(list, amount = "number_of_parameter_sets", length = 4)]
    pub types: Vec<String>,

    /// 19 ASCII characters YYYY-MM-DD:HH:MM:SS
    #[open_protocol_field(list, amount = "number_of_parameter_sets", length = 19)]
    pub date_of_last_change: Vec<String>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0012, revision = 1)]
pub struct MID0012rev1 {

    /// Three ASCII digits for the parameter set ID
    #[open_protocol_field(length = 3)]
    pub parameter_set_id: u16,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0013, revision = 1)]
pub struct MID0013rev1 {
    /// Three ASCII digits for the parameter set ID
    #[open_protocol_field(number = 1, length = 3)]
    pub parameter_set_id: u16,

    /// 25 ASCII characters for the parameter set name, right-padded with spaces if less
    #[open_protocol_field(number = 2, length = 25)]
    pub parameter_set_name: String,

    /// One ASCII digit for rotation direction: 1=CW, 2=CCW
    #[open_protocol_field(number = 3, length = 1)]
    pub rotation_direction: RotationDirection,

    /// Two ASCII digits for batch size
    #[open_protocol_field(number = 4, length = 2)]
    pub batch_size: u8,

    /// Six ASCII digits for torque minimum, multiplied by 100
    #[open_protocol_field(number = 5, length = 6)]
    pub torque_min: u32,

    /// Six ASCII digits for torque maximum, multiplied by 100
    #[open_protocol_field(number = 6, length = 6)]
    pub torque_max: u32,

    /// Six ASCII digits for the final torque target, multiplied by 100
    #[open_protocol_field(number = 7, length = 6)]
    pub final_torque_target: u32,

    /// Five ASCII digits for angle minimum
    #[open_protocol_field(number = 8, length = 5)]
    pub angle_min: u16,

    /// Five ASCII digits for angle maximum
    #[open_protocol_field(number = 9, length = 5)]
    pub angle_max: u16,

    /// Five ASCII digits for the final angle target
    #[open_protocol_field(number = 10, length = 5)]
    pub final_angle_target: u16,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0013, revision = 2)]
pub struct MID0013rev2 {
    /// Three ASCII digits for the parameter set ID
    #[open_protocol_field(number = 1, length = 3)]
    pub parameter_set_id: u16,

    /// 25 ASCII characters for the parameter set name, right-padded with spaces if less
    #[open_protocol_field(number = 2, length = 25)]
    pub parameter_set_name: String,

    /// One ASCII digit for rotation direction: 1=CW, 2=CCW
    #[open_protocol_field(number = 3, length = 1)]
    pub rotation_direction: RotationDirection,

    /// Two ASCII digits for batch size
    #[open_protocol_field(number = 4, length = 2)]
    pub batch_size: u8,

    /// Six ASCII digits for torque minimum, multiplied by 100
    #[open_protocol_field(number = 5, length = 6)]
    pub torque_min: u32,

    /// Six ASCII digits for torque maximum, multiplied by 100
    #[open_protocol_field(number = 6, length = 6)]
    pub torque_max: u32,

    /// Six ASCII digits for the final torque target, multiplied by 100
    #[open_protocol_field(number = 7, length = 6)]
    pub final_torque_target: u32,

    /// Five ASCII digits for angle minimum
    #[open_protocol_field(number = 8, length = 5)]
    pub angle_min: u16,

    /// Five ASCII digits for angle maximum
    #[open_protocol_field(number = 9, length = 5)]
    pub angle_max: u16,

    /// Five ASCII digits for the final angle target
    #[open_protocol_field(number = 10, length = 5)]
    pub final_angle_target: u16,

    /// Five ASCII digits for angle maximum
    #[open_protocol_field(number = 11, length = 6)]
    pub first_target: u32,

    /// Five ASCII digits for the final angle target
    #[open_protocol_field(number = 12, length = 6)]
    pub start_final_angle: u32,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0014, revision = 1)]
pub struct MID0014rev1 {
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0015, revision = 1)]
pub struct MID0015rev1 {
    /// Three ASCII digits for the parameter set ID
    #[open_protocol_field(number = 1, length = 3)]
    pub parameter_set_id: u16,

    /// 19 ASCII characters for the date of the last change in the parameter set settings
    #[open_protocol_field(number = 2, length = 19)]
    pub date_of_last_change: String,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0015, revision = 2)]
pub struct MID0015rev2 {
    /// Three ASCII digits for the parameter set ID (000-999)
    #[open_protocol_field(number = 1, length = 3)]
    pub parameter_set_id: u16,

    /// 25 ASCII characters for the parameter set name, right-padded with spaces if less than 25 characters
    #[open_protocol_field(number = 2, length = 25)]
    pub parameter_set_name: String,

    /// 19 ASCII characters for the date of the last change in the parameter set (YYYY-MM-DD:HH:MM:SS)
    #[open_protocol_field(number = 3, length = 19)]
    pub date_of_last_change: String,

    /// 1 ASCII character for the rotation direction (1 = CW, 2 = CCW)
    #[open_protocol_field(number = 4, length = 1)]
    pub rotation_direction: u8,

    /// Two ASCII digits for the batch size (00-99)
    #[open_protocol_field(number = 5, length = 2)]
    pub batch_size: u8,

    /// Six ASCII digits for the torque minimum, multiplied by 100 (000000-999999)
    #[open_protocol_field(number = 6, length = 6)]
    pub torque_min: u32,

    /// Six ASCII digits for the torque maximum, multiplied by 100 (000000-999999)
    #[open_protocol_field(number = 7, length = 6)]
    pub torque_max: u32,

    /// Six ASCII digits for the final torque target, multiplied by 100 (000000-999999)
    #[open_protocol_field(number = 8, length = 6)]
    pub final_torque_target: u32,

    /// Five ASCII digits for the angle minimum (00000-99999)
    #[open_protocol_field(number = 9, length = 5)]
    pub angle_min: u16,

    /// Five ASCII digits for the angle maximum (00000-99999)
    #[open_protocol_field(number = 10, length = 5)]
    pub angle_max: u16,

    /// Five ASCII digits for the final angle target (00000-99999)
    #[open_protocol_field(number = 11, length = 5)]
    pub final_angle_target: u16,

    /// Six ASCII digits for the first torque target, multiplied by 100 (000000-999999)
    #[open_protocol_field(number = 12, length = 6)]
    pub first_torque_target: u32,

    /// Six ASCII digits for the start final angle, multiplied by 100 (000000-999999)
    #[open_protocol_field(number = 13, length = 6)]
    pub start_final_angle: u32,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0016, revision = 1)]
pub struct MID0016rev1 {
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0017, revision = 1)]
pub struct MID0017rev1 {
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0018, revision = 1)]
pub struct MID0018rev1 {
    /// Three ASCII digits for the parameter set ID
    #[open_protocol_field(number = 1, length = 3)]
    pub parameter_set_id: u16,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0019, revision = 1)]
pub struct MID0019rev1 {
    /// Three ASCII digits for the parameter set ID (000-999)
    #[open_protocol_field(number = 1, length = 3)]
    pub parameter_set_id: u16,

    /// Two ASCII digits for the batch size (00-99)
    #[open_protocol_field(number = 2, length = 2)]
    pub batch_size: u8,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0019, revision = 2)]
pub struct MID0019rev2 {
    /// Three ASCII digits for the parameter set ID (000-999)
    #[open_protocol_field(number = 1, length = 3)]
    pub parameter_set_id: u16,

    /// Four ASCII digits for the batch size (0000-9999)
    #[open_protocol_field(number = 2, length = 4)]
    pub batch_size: u16,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0020, revision = 1)]
pub struct MID0020rev1 {

    /// Three ASCII digits for the parameter set ID (000-999)
    #[open_protocol_field(number = 1, length = 3)]
    pub parameter_set_id: u16,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0021, revision = 1)]
pub struct MID0021rev1 {
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0022, revision = 1)]
pub struct MID0022rev1 {

    /// One ASCII digit representing the relay status (0 = inactive, 1 = active)
    #[open_protocol_field(number = 1, length = 1)]
    pub relay_status: RelayStatus,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 2506, revision = 1)]
pub struct MID2506rev1 {

    /// Four ASCII digits for the program ID. A zero value is illegal.
    #[open_protocol_field(number = 1, length = 4)]
    pub program_id: u32,

    /// Three ASCII digits for the node type (e.g., 201 for a multistep tightening program)
    #[open_protocol_field(number = 2, length = 3)]
    pub node_type: u16,
}

#[cfg(test)]
mod tests {
    use open_protocol_codec::{decode, encode};
    use super::*;

    #[test]
    fn encode_0010() {
        let message = MID0010rev1 {};

        let result = encode::encode(&message);

        assert_eq!(result, Ok("".to_string()));
    }

    #[test]
    fn encode_0012() {
        let message = MID0012rev1 { parameter_set_id: 12 };

        let result = encode::encode(&message);

        assert_eq!(result, Ok("01012".to_string()));
    }

    #[test]
    fn encode_0011rev1() {
        let message = MID0011rev1 {
            number_of_parameter_sets: 5,
            parameter_set_ids: vec![1, 2, 3, 10, 20],
        };

        let result = encode::encode(&message);

        assert_eq!(result, Ok("005001002003010020".to_string()));
    }

    #[test]
    fn decode_0011rev1() {
        let message = "005001002003010020";

        let result = decode::decode(message.as_bytes());

        assert_eq!(result, Ok(MID0011rev1 {
            number_of_parameter_sets: 5,
            parameter_set_ids: vec![1, 2, 3, 10, 20],
        }));
    }
}
