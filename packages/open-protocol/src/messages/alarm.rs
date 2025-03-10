use chrono::{DateTime, Local};
use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub enum ToolReadyStatus {
    #[default]
    #[open_protocol_value(number = 0)]
    NOK,
    #[open_protocol_value(number = 1)]
    OK
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub enum ControllerReadyStatus {
    #[default]
    #[open_protocol_value(number = 0)]
    NOK,
    #[open_protocol_value(number = 1)]
    OK
}

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub enum AlarmStatus {
    #[default]
    #[open_protocol_value(number = 0)]
    NoAlarm,
    #[open_protocol_value(number = 1)]
    AlarmActive
}

/// A subscription request for alarms in the controller.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 70, revision = 1)]
pub struct MID0070rev1 {
    // No additional fields for this MID.
}

/// This message is sent when an alarm appears in the controller.
/// It includes the alarm code, controller/tool status, and timestamp.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 71, revision = 1)]
pub struct MID0071rev1 {
    /// The alarm error code.
    #[open_protocol_field(length = 4)]
    pub error_code: String,

    /// Controller ready status (1=OK, 0=NOK).
    #[open_protocol_field(length = 1)]
    pub controller_ready_status: ControllerReadyStatus,

    /// Tool ready status (1=OK, 0=NOK).
    #[open_protocol_field(length = 1)]
    pub tool_ready_status: ToolReadyStatus,

    /// Timestamp for the alarm (YYYY-MM-DD:HH:MM:SS).
    #[open_protocol_field(length = 19)]
    pub timestamp: DateTime<Local>,
}

/// Acknowledgment for MID 0071 Alarm.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 72, revision = 1)]
pub struct MID0072rev1 {
    // No additional fields for this MID.
}

/// Cancels a previously subscribed alarm notification.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 73, revision = 1)]
pub struct MID0073rev1 {
    // No additional fields for this MID.
}

/// The controller informs that the alarm has been acknowledged.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 74, revision = 1)]
pub struct MID0074rev1 {
    /// The alarm error code that was acknowledged.
    #[open_protocol_field(length = 4)]
    pub error_code: String,
}

/// Acknowledges receipt of MID 0074 Alarm Acknowledged on Controller.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 75, revision = 1)]
pub struct MID0075rev1 {
    // No additional fields for this MID.
}

/// Provides the status of an alarm after subscription.
/// This message is used to inform the integrator of active alarms.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 76, revision = 1)]
pub struct MID0076rev1 {
    /// Alarm status (0=no alarm, 1=alarm active).
    #[open_protocol_field(length = 1)]
    pub alarm_status: AlarmStatus,

    /// The alarm error code.
    #[open_protocol_field(length = 4)]
    pub error_code: String,

    /// Controller ready status (1=OK, 0=NOK).
    #[open_protocol_field(length = 1)]
    pub controller_ready_status: ControllerReadyStatus,

    /// Tool ready status (1=OK, 0=NOK).
    #[open_protocol_field(length = 1)]
    pub tool_ready_status: ToolReadyStatus,

    /// Timestamp of the alarm (YYYY-MM-DD:HH:MM:SS).
    #[open_protocol_field(length = 19)]
    pub timestamp: DateTime<Local>,
}

/// Acknowledges receipt of MID 0076 Alarm Status.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 77, revision = 1)]
pub struct MID0077rev1 {
    // No additional fields for this MID.
}

/// The integrator remotely acknowledges the current alarm on the controller.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 78, revision = 1)]
pub struct MID0078rev1 {
    /// The alarm error code to acknowledge remotely.
    #[open_protocol_field(length = 4)]
    pub error_code: String,
}

/// An alarm has appeared in the controller.
/// This message replaces the old MID 0071.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 1000, revision = 1)]
pub struct MID1000rev1 {
    /// The alarm error code.
    #[open_protocol_field(length = 5)]
    pub error_code: String,

    /// Timestamp for the alarm (YYYY-MM-DD:HH:MM:SS).
    #[open_protocol_field(length = 19)]
    pub timestamp: DateTime<Local>,

    /// The number of variable data fields included in this message.
    #[open_protocol_field(length = 3)]
    pub number_of_data_fields: u16,

    /// The variable data fields associated with the alarm.
    #[open_protocol_field(list, amount = "number_of_data_fields")]
    pub data_fields: Vec<u8>,
}

/// Acknowledges receipt of MID 1000 Alarm.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 1001, revision = 1)]
pub struct MID1001rev1 {
    // No additional fields for this MID.
}
