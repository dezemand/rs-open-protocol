use chrono::{DateTime, Local};
use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};
use crate::types::datafield::DataField;
use crate::types::trace::TraceSample;

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub enum TraceType {
    #[default]
    #[open_protocol_value(number = 1)]
    Angle,
    #[open_protocol_value(number = 2)]
    Torque,
    #[open_protocol_value(number = 3)]
    Current,
    #[open_protocol_value(number = 4)]
    Gradient,
    #[open_protocol_value(number = 5)]
    Stroke,
    #[open_protocol_value(number = 6)]
    Force
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub enum ObjectType {
    #[default]
    #[open_protocol_value(number = 1)]
    DualReading,
    #[open_protocol_value(number = 2)]
    TighteningProduction,
    #[open_protocol_value(number = 3)]
    TighteningSimulation,
    #[open_protocol_value(number = 4)]
    JointCheck,
    #[open_protocol_value(number = 5)]
    Dimensional,
}

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

/// 5.8.9 MID 0900 Trace curve data message
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0900, revision = 1)]
pub struct MID0900rev1 {
    /// The unique identifier for the result data.
    #[open_protocol_field(length = 10)]
    pub result_data_id: u32,

    /// Timestamp of when the trace was recorded.
    #[open_protocol_field(length = 19)]
    pub timestamp: DateTime<Local>,

    /// The number of PIDs (parameter data fields).
    #[open_protocol_field(length = 3)]
    pub number_of_pids: u16,

    /// Data fields (list of parameter values).
    #[open_protocol_field(list, amount = "number_of_pids")]
    pub data_fields: Vec<DataField>,

    /// The trace type identifier.
    #[open_protocol_field(length = 2)]
    pub trace_type: TraceType,

    /// The transducer type used in the trace.
    #[open_protocol_field(length = 2)]
    pub transducer_type: u8,

    /// The unit of measurement for the trace.
    #[open_protocol_field(length = 3)]
    pub unit: u16,

    /// The number of parameter fields.
    #[open_protocol_field(length = 3)]
    pub number_of_parameter_fields: u16,

    /// Parameter fields for time intervals.
    #[open_protocol_field(list, amount = "number_of_parameter_fields")]
    pub parameter_fields: Vec<DataField>,

    /// The number of resolution fields.
    #[open_protocol_field(length = 3)]
    pub number_of_resolution_fields: u16,

    /// Resolution fields for time intervals.
    #[open_protocol_field(list, amount = "number_of_resolution_fields")]
    pub resolution_fields: Vec<DataField>,

    /// The number of trace samples included.
    #[open_protocol_field(length = 5)]
    pub number_of_trace_samples: u32,

    /// NUL character (0x00) separating text from binary data.
    #[open_protocol_field(length = 1)]
    pub nul_character: u8,

    /// The trace samples (binary values).
    #[open_protocol_field(list, amount = "number_of_trace_samples", length = 2)]
    pub trace_samples: Vec<TraceSample>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0900, revision = 2)]
pub struct MID0900rev2 {
    /// The unique identifier for the result data.
    #[open_protocol_field(length = 10)]
    pub result_data_id: u32,

    /// Timestamp of when the trace was recorded.
    #[open_protocol_field(length = 19)]
    pub timestamp: DateTime<Local>,

    /// The number of PIDs (parameter data fields).
    #[open_protocol_field(length = 3)]
    pub number_of_pids: u16,

    /// Data fields (list of parameter values).
    #[open_protocol_field(list, amount = "number_of_pids")]
    pub data_fields: Vec<DataField>,

    /// The trace type identifier.
    #[open_protocol_field(length = 2)]
    pub trace_type: TraceType,

    /// The transducer type used in the trace.
    #[open_protocol_field(length = 2)]
    pub transducer_type: u8,

    /// The unit of measurement for the trace.
    #[open_protocol_field(length = 3)]
    pub unit: u16,

    /// The MID of the request that this message is a response to.
    /// Typically 0008 (subscribe) or 0006 (data upload).
    #[open_protocol_field(length = 4)]
    pub request_mid: u16,

    /// The number of parameter fields.
    #[open_protocol_field(length = 3)]
    pub number_of_parameter_fields: u16,

    /// Parameter fields for time intervals.
    #[open_protocol_field(list, amount = "number_of_parameter_fields")]
    pub parameter_fields: Vec<DataField>,

    /// The number of resolution fields.
    #[open_protocol_field(length = 3)]
    pub number_of_resolution_fields: u16,

    /// Resolution fields for time intervals.
    #[open_protocol_field(list, amount = "number_of_resolution_fields")]
    pub resolution_fields: Vec<DataField>,

    /// The number of trace samples included.
    #[open_protocol_field(length = 5)]
    pub number_of_trace_samples: u32,

    /// NUL character (0x00) separating text from binary data.
    #[open_protocol_field(length = 1)]
    pub nul_character: u8,

    /// The trace samples (binary values).
    #[open_protocol_field(list, amount = "number_of_trace_samples", length = 2)]
    pub trace_samples: Vec<TraceSample>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0900, revision = 3)]
pub struct MID0900rev3 {
    /// The unique identifier for the result data.
    #[open_protocol_field(length = 10)]
    pub result_data_id: u32,

    /// Timestamp of when the trace was recorded.
    #[open_protocol_field(length = 19)]
    pub timestamp: DateTime<Local>,

    /// The user defined object ID
    #[open_protocol_field(length = 4)]
    pub object_id: u16,

    /// The type of the object
    #[open_protocol_field(length = 1)]
    pub object_type: ObjectType,

    /// Link to related Object ID
    #[open_protocol_field(length = 4)]
    pub reference_object_id: u16,

    /// The number of PIDs (parameter data fields).
    #[open_protocol_field(length = 3)]
    pub number_of_pids: u16,

    /// Data fields (list of parameter values).
    #[open_protocol_field(list, amount = "number_of_pids")]
    pub data_fields: Vec<DataField>,

    /// The trace type identifier.
    #[open_protocol_field(length = 2)]
    pub trace_type: TraceType,

    /// The transducer type used in the trace.
    #[open_protocol_field(length = 2)]
    pub transducer_type: u8,

    /// The unit of measurement for the trace.
    #[open_protocol_field(length = 3)]
    pub unit: u16,

    /// The MID of the request that this message is a response to.
    /// Typically 0008 (subscribe) or 0006 (data upload).
    #[open_protocol_field(length = 4)]
    pub request_mid: u16,

    /// The number of parameter fields.
    #[open_protocol_field(length = 3)]
    pub number_of_parameter_fields: u16,

    /// Parameter fields for time intervals.
    #[open_protocol_field(list, amount = "number_of_parameter_fields")]
    pub parameter_fields: Vec<DataField>,

    /// The number of resolution fields.
    #[open_protocol_field(length = 3)]
    pub number_of_resolution_fields: u16,

    /// Resolution fields for time intervals.
    #[open_protocol_field(list, amount = "number_of_resolution_fields")]
    pub resolution_fields: Vec<DataField>,

    /// The number of trace samples included.
    #[open_protocol_field(length = 5)]
    pub number_of_trace_samples: u32,

    /// NUL character (0x00) separating text from binary data.
    #[open_protocol_field(length = 1)]
    pub nul_character: u8,

    /// The trace samples (binary values).
    #[open_protocol_field(list, amount = "number_of_trace_samples", length = 2)]
    pub trace_samples: Vec<TraceSample>,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub struct MID0900RequestExtraData {
    /// The index of the requested tightening result. If equal to zero, response
    /// will contain the most recent result.
    #[open_protocol_field(length = 10)]
    pub index: u64,

    /// Type of the trace requested.
    #[open_protocol_field(length = 3)]
    pub trace_type: TraceType,

    /// The number of the tool.
    #[open_protocol_field(length = 4)]
    pub tool_number: u16,
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub struct MID0900SubscribeExtraData {
    // TODO
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub struct MID0900UnsubscribeExtraData {
    // TODO
}

/// Subscribe, MID 0901 Trace plotting parameter
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0901, revision = 1)]
pub struct MID0901rev1 {
    // TODO
}

/// Subscribe, MID 0901 Trace plotting parameter
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0901, revision = 2)]
pub struct MID0901rev2 {
    // TODO
}

/// Subscribe, MID 0901 Trace plotting parameter
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0901, revision = 3)]
pub struct MID0901rev3 {
    // TODO
}

/// 5.8.11 MID 0902 Tightening Result DB Info Upload
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 0902, revision = 1)]
pub struct MID0902rev1 {
    /// The total number of tightening results that can be stored.
    #[open_protocol_field(length = 10)]
    pub capacity: u64,

    /// The sequence number of the oldest tightening result currently in the database.
    #[open_protocol_field(length = 10)]
    pub oldest_sequence_number: u64,

    /// The start time of the oldest tightening result currently in the database.
    #[open_protocol_field(length = 19)]
    pub oldest_time: DateTime<Local>,

    /// The sequence number of the newest tightening result currently in the database.
    #[open_protocol_field(length = 10)]
    pub newest_sequence_number: u64,

    /// The start time of the newest tightening result currently in the database.
    #[open_protocol_field(length = 19)]
    pub newest_time: DateTime<Local>,

    /// The number of PIDs (parameter data fields).
    #[open_protocol_field(length = 3)]
    pub number_of_pids: u16,

    /// Data fields (list of parameter values).
    #[open_protocol_field(list, amount = "number_of_pids")]
    pub data_fields: Vec<DataField>,
}
