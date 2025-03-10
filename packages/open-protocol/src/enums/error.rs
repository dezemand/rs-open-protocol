use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode};
use thiserror::Error;

#[derive(Debug, Default, Clone, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, Error)]
pub enum ErrorCode {
    #[default]
    #[error("No Error")]
    #[open_protocol_value(number = 0)]
    NoError,

    #[error("Invalid data")]
    #[open_protocol_value(number = 1)]
    InvalidData,

    #[error("Parameter set ID not present")]
    #[open_protocol_value(number = 2)]
    ParameterSetIdNotPresent,

    #[error("Parameter set cannot be set")]
    #[open_protocol_value(number = 3)]
    ParameterSetCannotBeSet,

    #[error("Parameter set not running")]
    #[open_protocol_value(number = 4)]
    ParameterSetNotRunning,

    #[error("VIN upload subscription already exists")]
    #[open_protocol_value(number = 6)]
    VinUploadSubscriptionExists,

    #[error("VIN upload subscription does not exist")]
    #[open_protocol_value(number = 7)]
    VinUploadSubscriptionDoesNotExist,

    #[error("VIN input source not granted")]
    #[open_protocol_value(number = 8)]
    VinInputSourceNotGranted,

    #[error("Last tightening result subscription already exists")]
    #[open_protocol_value(number = 9)]
    LastTighteningResultSubscriptionExists,

    #[error("Last tightening result subscription does not exist")]
    #[open_protocol_value(number = 10)]
    LastTighteningResultSubscriptionDoesNotExist,

    #[error("Alarm subscription already exists")]
    #[open_protocol_value(number = 11)]
    AlarmSubscriptionExists,

    #[error("Alarm subscription does not exist")]
    #[open_protocol_value(number = 12)]
    AlarmSubscriptionDoesNotExist,

    #[error("Parameter set selection subscription already exists")]
    #[open_protocol_value(number = 13)]
    ParameterSetSelectionSubscriptionExists,

    #[error("Parameter set selection subscription does not exist")]
    #[open_protocol_value(number = 14)]
    ParameterSetSelectionSubscriptionDoesNotExist,

    #[error("Tightening ID requested not found")]
    #[open_protocol_value(number = 15)]
    TighteningIdNotFound,

    #[error("Connection rejected: protocol busy")]
    #[open_protocol_value(number = 16)]
    ConnectionRejectedProtocolBusy,

    #[error("Job ID not present")]
    #[open_protocol_value(number = 17)]
    JobIdNotPresent,

    #[error("Job info subscription already exists")]
    #[open_protocol_value(number = 18)]
    JobInfoSubscriptionExists,

    #[error("Job info subscription does not exist")]
    #[open_protocol_value(number = 19)]
    JobInfoSubscriptionDoesNotExist,

    #[error("Job cannot be set")]
    #[open_protocol_value(number = 20)]
    JobCannotBeSet,

    #[error("Job not running")]
    #[open_protocol_value(number = 21)]
    JobNotRunning,

    #[error("Not possible to execute dynamic Job request")]
    #[open_protocol_value(number = 22)]
    DynamicJobRequestNotPossible,

    #[error("Job batch decrement failed")]
    #[open_protocol_value(number = 23)]
    JobBatchDecrementFailed,

    #[error("Not possible to create Pset")]
    #[open_protocol_value(number = 24)]
    CreatePsetNotPossible,

    #[error("Programming control not granted")]
    #[open_protocol_value(number = 25)]
    ProgrammingControlNotGranted,

    #[error("Wrong tool type for Pset download")]
    #[open_protocol_value(number = 26)]
    WrongToolTypeForPsetDownload,

    #[error("Tool is inaccessible")]
    #[open_protocol_value(number = 27)]
    ToolInaccessible,

    #[error("Job abortion is in progress")]
    #[open_protocol_value(number = 28)]
    JobAbortionInProgress,

    #[error("Tool does not exist")]
    #[open_protocol_value(number = 29)]
    ToolDoesNotExist,

    #[error("Controller is not a sync Master/station controller")]
    #[open_protocol_value(number = 30)]
    NotSyncMasterOrStationController,

    #[error("Multi-spindle status subscription already exists")]
    #[open_protocol_value(number = 31)]
    MultiSpindleStatusSubscriptionExists,

    #[error("Multi-spindle status subscription does not exist")]
    #[open_protocol_value(number = 32)]
    MultiSpindleStatusSubscriptionDoesNotExist,

    #[error("Multi-spindle result subscription already exists")]
    #[open_protocol_value(number = 33)]
    MultiSpindleResultSubscriptionExists,

    #[error("Multi-spindle result subscription does not exist")]
    #[open_protocol_value(number = 34)]
    MultiSpindleResultSubscriptionDoesNotExist,

    #[error("Other master client already connected")]
    #[open_protocol_value(number = 35)]
    OtherMasterClientAlreadyConnected,

    #[error("Lock type not supported")]
    #[open_protocol_value(number = 36)]
    LockTypeNotSupported,

    #[error("Job line control info subscription already exists")]
    #[open_protocol_value(number = 40)]
    JobLineControlSubscriptionExists,

    #[error("Job line control info subscription does not exist")]
    #[open_protocol_value(number = 41)]
    JobLineControlSubscriptionDoesNotExist,

    #[error("Identifier input source not granted")]
    #[open_protocol_value(number = 42)]
    IdentifierInputSourceNotGranted,

    #[error("Multiple identifiers work order subscription already exists")]
    #[open_protocol_value(number = 43)]
    MultipleIdentifiersWorkOrderSubscriptionExists,

    #[error("Multiple identifiers work order subscription does not exist")]
    #[open_protocol_value(number = 44)]
    MultipleIdentifiersWorkOrderSubscriptionDoesNotExist,

    #[error("Status external monitored inputs subscription already exists")]
    #[open_protocol_value(number = 50)]
    StatusExternalMonitoredInputsSubscriptionExists,

    #[error("Status external monitored inputs subscription does not exist")]
    #[open_protocol_value(number = 51)]
    StatusExternalMonitoredInputsSubscriptionDoesNotExist,

    #[error("IO device not connected")]
    #[open_protocol_value(number = 52)]
    IoDeviceNotConnected,

    #[error("Faulty IO device ID")]
    #[open_protocol_value(number = 53)]
    FaultyIoDeviceId,

    #[error("Tool Tag ID unknown")]
    #[open_protocol_value(number = 54)]
    ToolTagIdUnknown,

    #[error("Tool Tag ID subscription already exists")]
    #[open_protocol_value(number = 55)]
    ToolTagIdSubscriptionExists,

    #[error("Tool Tag ID subscription does not exist")]
    #[open_protocol_value(number = 56)]
    ToolTagIdSubscriptionDoesNotExist,

    #[error("Tool motor tuning failed")]
    #[open_protocol_value(number = 57)]
    ToolMotorTuningFailed,

    #[error("No alarm present")]
    #[open_protocol_value(number = 58)]
    NoAlarmPresent,

    #[error("Tool currently in use")]
    #[open_protocol_value(number = 59)]
    ToolCurrentlyInUse,

    #[error("No histogram available")]
    #[open_protocol_value(number = 60)]
    NoHistogramAvailable,

    #[error("Pairing failed")]
    #[open_protocol_value(number = 61)]
    PairingFailed,

    #[error("Pairing denied")]
    #[open_protocol_value(number = 62)]
    PairingDenied,

    #[error("Pairing or pairing abortion attempt on wrong tool type")]
    #[open_protocol_value(number = 63)]
    PairingWrongToolType,

    #[error("Pairing abortion denied")]
    #[open_protocol_value(number = 64)]
    PairingAbortionDenied,

    #[error("Pairing abortion failed")]
    #[open_protocol_value(number = 65)]
    PairingAbortionFailed,

    #[error("Pairing disconnection failed")]
    #[open_protocol_value(number = 66)]
    PairingDisconnectionFailed,

    #[error("Pairing in progress or already done")]
    #[open_protocol_value(number = 67)]
    PairingInProgress,

    #[error("Pairing denied, no program control")]
    #[open_protocol_value(number = 68)]
    PairingDeniedNoProgramControl,

    #[error("Unsupported extra data revision")]
    #[open_protocol_value(number = 69)]
    UnsupportedExtraDataRevision,

    #[error("Calibration failed")]
    #[open_protocol_value(number = 70)]
    CalibrationFailed,

    #[error("Subscription already exists")]
    #[open_protocol_value(number = 71)]
    SubscriptionAlreadyExists,

    #[error("Subscription does not exist")]
    #[open_protocol_value(number = 72)]
    SubscriptionDoesNotExist,

    #[error("Subscribed MID unsupported (trying to subscribe on a non-existing MID)")]
    #[open_protocol_value(number = 73)]
    SubscribedMIDUnsupported,

    #[error("Subscribed MID revision unsupported (trying to subscribe to an unsupported MID revision)")]
    #[open_protocol_value(number = 74)]
    SubscribedMIDRevisionUnsupported,

    #[error("Requested MID unsupported (trying to request a non-existing MID)")]
    #[open_protocol_value(number = 75)]
    RequestedMIDUnsupported,

    #[error("Requested MID revision unsupported (trying to request an unsupported MID revision)")]
    #[open_protocol_value(number = 76)]
    RequestedMIDRevisionUnsupported,

    #[error("Requested specific data not supported (trying to request data that is not supported)")]
    #[open_protocol_value(number = 77)]
    RequestedSpecificDataNotSupported,

    #[error("Subscription on specific data not supported (trying to subscribe for unsupported data)")]
    #[open_protocol_value(number = 78)]
    SubscriptionSpecificDataNotSupported,

    #[error("Command failed")]
    #[open_protocol_value(number = 79)]
    CommandFailed,

    #[error("Audi emergency status subscription exists")]
    #[open_protocol_value(number = 80)]
    AudiEmergencyStatusSubscriptionExists,

    #[error("Audi emergency status subscription does not exist")]
    #[open_protocol_value(number = 81)]
    AudiEmergencyStatusSubscriptionDoesNotExist,

    #[error("Automatic/Manual mode subscription already exists")]
    #[open_protocol_value(number = 82)]
    AutomaticManualModeSubscriptionExists,

    #[error("Automatic/Manual mode subscription does not exist")]
    #[open_protocol_value(number = 83)]
    AutomaticManualModeSubscriptionDoesNotExist,

    #[error("The relay function subscription already exists")]
    #[open_protocol_value(number = 84)]
    RelayFunctionSubscriptionExists,

    #[error("The relay function subscription does not exist")]
    #[open_protocol_value(number = 85)]
    RelayFunctionSubscriptionDoesNotExist,

    #[error("The selector socket info subscription already exists")]
    #[open_protocol_value(number = 86)]
    SelectorSocketInfoSubscriptionExists,

    #[error("The selector socket info subscription does not exist")]
    #[open_protocol_value(number = 87)]
    SelectorSocketInfoSubscriptionDoesNotExist,

    #[error("The digital input info subscription already exists")]
    #[open_protocol_value(number = 88)]
    DigitalInputSubscriptionExists,

    #[error("The digital input info subscription does not exist")]
    #[open_protocol_value(number = 89)]
    DigitalInputSubscriptionDoesNotExist,

    #[error("Lock at batch done subscription already exists")]
    #[open_protocol_value(number = 90)]
    LockAtBatchDoneSubscriptionExists,

    #[error("Lock at batch done subscription does not exist")]
    #[open_protocol_value(number = 91)]
    LockAtBatchDoneSubscriptionDoesNotExist,

    #[error("Open protocol commands disabled")]
    #[open_protocol_value(number = 92)]
    OpenProtocolCommandsDisabled,

    #[error("Open protocol commands disabled subscription already exists")]
    #[open_protocol_value(number = 93)]
    OpenProtocolCommandsDisabledSubscriptionExists,

    #[error("Open protocol commands disabled subscription does not exist")]
    #[open_protocol_value(number = 94)]
    OpenProtocolCommandsDisabledSubscriptionDoesNotExist,

    #[error("Reject request, PowerMACS is in manual mode")]
    #[open_protocol_value(number = 95)]
    RejectRequestPowerMACSManualMode,

    #[error("Reject connection, client already connected")]
    #[open_protocol_value(number = 96)]
    RejectConnectionClientAlreadyConnected,

    #[error("MID revision unsupported")]
    #[open_protocol_value(number = 97)]
    MIDRevisionUnsupported,

    #[error("Controller internal request timeout")]
    #[open_protocol_value(number = 98)]
    ControllerInternalRequestTimeout,

    #[error("Unknown MID")]
    #[open_protocol_value(number = 99)]
    UnknownMID,

    #[error("Illegal PID")]
    #[open_protocol_value(number = 100)]
    IllegalPID,

    #[error("Tightening in progress")]
    #[open_protocol_value(number = 101)]
    TighteningInProgress,

    #[error("Delete of object not possible")]
    #[open_protocol_value(number = 102)]
    DeleteOfObjectNotPossible,

    #[error("Illegal program ID")]
    #[open_protocol_value(number = 103)]
    IllegalProgramID,

    #[error("Illegal node type")]
    #[open_protocol_value(number = 104)]
    IllegalNodeType,

    #[error("Application specific error code {0}")]
    #[open_protocol_value(number_range = "900-999")]
    ApplicationSpecificErrorCode(u16),

    #[error("Unknown error with code {0}")]
    #[open_protocol_value(other)]
    UnknownError(u16)
}
