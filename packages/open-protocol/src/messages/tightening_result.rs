use chrono::{DateTime, Local};
use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0060, revision = 1)]
pub struct MID0060rev1 {
    // Subscribe to tightening result data
    // No fields for this revision
}

// MID0061 revisions
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0061, revision = 1)]
pub struct MID0061rev1 {
    /// The cell ID of the tightening system
    #[open_protocol_field(number = 1, length = 4)]
    pub cell_id: u16,

    /// The channel ID for the tightening
    #[open_protocol_field(number = 2, length = 2)]
    pub channel_id: u8,

    /// The name of the torque controller
    #[open_protocol_field(number = 3, length = 25)]
    pub controller_name: String,

    /// The Vehicle Identification Number (VIN) used in the tightening
    #[open_protocol_field(number = 4, length = 25)]
    pub vin_number: String,

    /// The ID of the job performed
    #[open_protocol_field(number = 5, length = 2)]
    pub job_id: u8,

    /// The parameter set ID used
    #[open_protocol_field(number = 6, length = 3)]
    pub parameter_set_id: u16,

    /// The total number of tightenings required in the batch
    #[open_protocol_field(number = 7, length = 4)]
    pub batch_size: u16,

    /// The current batch counter
    #[open_protocol_field(number = 8, length = 4)]
    pub batch_counter: u16,

    /// The status of the tightening (0=NOK, 1=OK)
    #[open_protocol_field(number = 9, length = 1)]
    pub tightening_status: u8,

    /// The torque status (0=Low, 1=OK, 2=High)
    #[open_protocol_field(number = 10, length = 1)]
    pub torque_status: u8,

    /// The angle status (0=Low, 1=OK, 2=High)
    #[open_protocol_field(number = 11, length = 1)]
    pub angle_status: u8,

    /// The minimum torque limit
    #[open_protocol_field(number = 12, length = 6)]
    pub torque_min_limit: u32,

    /// The maximum torque limit
    #[open_protocol_field(number = 13, length = 6)]
    pub torque_max_limit: u32,

    /// The final torque target
    #[open_protocol_field(number = 14, length = 6)]
    pub torque_final_target: u32,

    /// The achieved torque value
    #[open_protocol_field(number = 15, length = 6)]
    pub torque: u32,

    /// The minimum angle limit
    #[open_protocol_field(number = 16, length = 5)]
    pub angle_min_limit: u16,

    /// The maximum angle limit
    #[open_protocol_field(number = 17, length = 5)]
    pub angle_max_limit: u16,

    /// The final angle target
    #[open_protocol_field(number = 18, length = 5)]
    pub angle_final_target: u16,

    /// The achieved angle value
    #[open_protocol_field(number = 19, length = 5)]
    pub angle: u16,

    /// Timestamp of the tightening
    #[open_protocol_field(number = 20, length = 19)]
    pub timestamp: DateTime<Local>,

    /// Timestamp of the last parameter set change
    #[open_protocol_field(number = 21, length = 19)]
    pub last_parameter_set_change: DateTime<Local>,

    /// The batch status (0=NOK, 1=OK, 2=Not used, 3=Running)
    #[open_protocol_field(number = 22, length = 1)]
    pub batch_status: u8,

    /// The tightening ID, a unique identifier for each result
    #[open_protocol_field(number = 23, length = 10)]
    pub tightening_id: u32,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0061, revision = 2)]
pub struct MID0061rev2 {
    /// The cell ID of the tightening system
    #[open_protocol_field(number = 1, length = 4)]
    pub cell_id: u16,

    /// The channel ID for the tightening
    #[open_protocol_field(number = 2, length = 2)]
    pub channel_id: u8,

    /// The name of the torque controller
    #[open_protocol_field(number = 3, length = 25)]
    pub controller_name: String,

    /// The Vehicle Identification Number (VIN) used in the tightening
    #[open_protocol_field(number = 4, length = 25)]
    pub vin_number: String,

    /// The ID of the job performed
    #[open_protocol_field(number = 5, length = 4)]
    pub job_id: u16,

    /// The parameter set ID used
    #[open_protocol_field(number = 6, length = 3)]
    pub parameter_set_id: u16,

    /// Strategy used in the tightening
    #[open_protocol_field(number = 7, length = 2)]
    pub strategy: u8,

    /// Strategy used in the tightening
    #[open_protocol_field(number = 8, length = 5)]
    pub strategy_options: u32,

    /// Batch size for the tightening operation
    #[open_protocol_field(number = 9, length = 4)]
    pub batch_size: u16,

    /// The current count of tightenings in the batch
    #[open_protocol_field(number = 10, length = 4)]
    pub batch_counter: u16,

    /// The status of the tightening (0=NOK, 1=OK)
    #[open_protocol_field(number = 11, length = 1)]
    pub tightening_status: u8,

    /// The batch status (0=NOK, 1=OK, 2=Not used, 3=Running)
    #[open_protocol_field(number = 12, length = 1)]
    pub batch_status: u8,

    /// The torque status (0=Low, 1=OK, 2=High)
    #[open_protocol_field(number = 13, length = 1)]
    pub torque_status: u8,

    /// The angle status (0=Low, 1=OK, 2=High)
    #[open_protocol_field(number = 14, length = 1)]
    pub angle_status: u8,

    /// The rundown angle status (0=Low, 1=OK, 2=High)
    #[open_protocol_field(number = 15, length = 1)]
    pub rundown_angle_status: u8,

    /// Current monitoring status (0=Low, 1=OK, 2=High)
    #[open_protocol_field(number = 16, length = 1)]
    pub current_monitoring_status: u8,

    /// Self-tap status (0=Low, 1=OK, 2=High)
    #[open_protocol_field(number = 17, length = 1)]
    pub self_tap_status: u8,

    /// Prevail torque monitoring status (0=Low, 1=OK, 2=High)
    #[open_protocol_field(number = 18, length = 1)]
    pub prevail_torque_monitoring_status: u8,

    /// Prevail torque compensate status (0=Low, 1=OK, 2=High)
    #[open_protocol_field(number = 19, length = 1)]
    pub prevail_torque_compensate_status: u8,

    /// Tightening error status
    #[open_protocol_field(number = 20, length = 10)]
    pub tightening_error_status: String,

    /// Minimum torque limit
    #[open_protocol_field(number = 21, length = 6)]
    pub torque_min_limit: u32,

    /// Maximum torque limit
    #[open_protocol_field(number = 22, length = 6)]
    pub torque_max_limit: u32,

    /// Final torque target
    #[open_protocol_field(number = 23, length = 6)]
    pub torque_final_target: u32,

    /// Achieved torque value
    #[open_protocol_field(number = 24, length = 6)]
    pub torque: u32,

    /// Minimum angle limit
    #[open_protocol_field(number = 25, length = 5)]
    pub angle_min_limit: u16,

    /// Maximum angle limit
    #[open_protocol_field(number = 26, length = 5)]
    pub angle_max_limit: u16,

    /// Final angle target
    #[open_protocol_field(number = 27, length = 5)]
    pub angle_final_target: u16,

    /// Achieved angle value
    #[open_protocol_field(number = 28, length = 5)]
    pub angle: u16,

    /// Minimum rundown angle
    #[open_protocol_field(number = 29, length = 5)]
    pub rundown_angle_min: u16,

    /// Maximum rundown angle
    #[open_protocol_field(number = 30, length = 5)]
    pub rundown_angle_max: u16,

    /// Achieved rundown angle
    #[open_protocol_field(number = 31, length = 5)]
    pub rundown_angle: u16,

    /// Minimum current monitoring value
    #[open_protocol_field(number = 32, length = 3)]
    pub current_monitoring_min: u16,

    /// Maximum current monitoring value
    #[open_protocol_field(number = 33, length = 3)]
    pub current_monitoring_max: u16,

    /// Achieved current monitoring value
    #[open_protocol_field(number = 34, length = 3)]
    pub current_monitoring_value: u16,

    /// Minimum self-tap torque
    #[open_protocol_field(number = 35, length = 6)]
    pub self_tap_torque_min: u32,

    /// Maximum self-tap torque
    #[open_protocol_field(number = 36, length = 6)]
    pub self_tap_torque_max: u32,

    /// Achieved self-tap torque
    #[open_protocol_field(number = 37, length = 6)]
    pub self_tap_torque: u32,

    /// Minimum prevail torque value
    #[open_protocol_field(number = 38, length = 6)]
    pub prevail_torque_min: u32,

    /// Maximum prevail torque value
    #[open_protocol_field(number = 39, length = 6)]
    pub prevail_torque_max: u32,

    /// Achieved prevail torque value
    #[open_protocol_field(number = 40, length = 6)]
    pub prevail_torque: u32,

    /// The tightening ID, a unique identifier for this result
    #[open_protocol_field(number = 41, length = 10)]
    pub tightening_id: u32,

    /// The job sequence number, a unique number for each job
    #[open_protocol_field(number = 42, length = 5)]
    pub job_sequence_number: u16,

    /// The sync tightening ID
    #[open_protocol_field(number = 43, length = 5)]
    pub sync_tightening_id: u16,

    /// The serial number of the tool
    #[open_protocol_field(number = 44, length = 14)]
    pub tool_serial_number: String,

    /// Timestamp of the tightening
    #[open_protocol_field(number = 45, length = 19)]
    pub timestamp: DateTime<Local>,

    /// Timestamp of the last parameter set change
    #[open_protocol_field(number = 46, length = 19)]
    pub last_parameter_set_change: DateTime<Local>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0061, revision = 3)]
pub struct MID0061rev3 {
    /// Name of the parameter set used
    #[open_protocol_field(number = 47, length = 25)]
    pub parameter_set_name: String,

    /// The unit of torque values (1=Nm, 2=Lbf.ft, 3=Lbf.In, etc.)
    #[open_protocol_field(number = 48, length = 1)]
    pub torque_unit: u8,

    /// The result type (1=Tightening, 2=Loosening, etc.)
    #[open_protocol_field(number = 49, length = 2)]
    pub result_type: u8,
}

// MID0062 (Acknowledgment for MID0061 tightening results)
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0062, revision = 1)]
pub struct MID0062rev1 {
    // Acknowledgment for tightening result data
    // No fields for this revision
}

// MID0063 (Unsubscribe from tightening results)
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0063, revision = 1)]
pub struct MID0063rev1 {
    // Unsubscribe from tightening result data
    // No fields for this revision
}

// MID0064 (Old tightening result upload request)
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0064, revision = 1)]
pub struct MID0064rev1 {
    /// Request a specific tightening result by its ID
    #[open_protocol_field(number = 1, length = 10)]
    pub tightening_id: u32,
}

// MID0065 (Old tightening result upload reply)
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0065, revision = 1)]
pub struct MID0065rev1 {
    // Reply with the details of an old tightening result
    // Contains the same fields as MID0061 revision 1, plus optional extensions
}

// MID0066 (Number of offline results)
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0066, revision = 1)]
pub struct MID0066rev1 {
    /// Number of offline results stored in the system
    #[open_protocol_field(number = 1, length = 2)]
    pub number_of_offline_results: u8,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0066, revision = 2)]
pub struct MID0066rev2 {
    /// Number of offline results stored in the system
    #[open_protocol_field(number = 1, length = 3)]
    pub number_of_offline_results: u16,

    /// Number of offline curves stored in the system
    #[open_protocol_field(number = 2, length = 3)]
    pub number_of_offline_curves: u16,
}

// MID0067 (Tightening result list upload)
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0067, revision = 1)]
pub struct MID0067rev1 {
    /// Start index of the requested tightening results
    #[open_protocol_field(number = 1, length = 10)]
    pub start_index: u32,

    /// Number of tightening results requested
    #[open_protocol_field(number = 2, length = 3)]
    pub count: u16,
}
