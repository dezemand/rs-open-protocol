use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode, OpenProtocolMessage};

#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode)]
pub enum RemovalCondition {
    #[default]
    #[open_protocol_value(number = 0)]
    AcknowledgeOrWait,
    #[open_protocol_value(number = 1)]
    Acknowledge
}

/// Display user text on compact display.
/// The text must be maximum 4 bytes long.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 110, revision = 1)]
pub struct MID0110rev1 {
    /// The user text to be displayed (max 4 characters).
    /// If less than 4 characters, right pad with spaces (0x20).
    #[open_protocol_field(length = 4)]
    pub user_text: String,
}

/// Display user text on a graphical display.
/// Allows setting display duration and acknowledgment settings.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 111, revision = 1)]
pub struct MID0111rev1 {
    /// Time duration for the text display in seconds.
    /// Four ASCII digits, range: 0000-9999.
    #[open_protocol_field(length = 4)]
    pub display_duration: u16,

    /// Removal condition: 0 = acknowledge or wait expiration time, 1 = acknowledge required.
    #[open_protocol_field(length = 1)]
    pub removal_condition: RemovalCondition,

    /// The first line (header) of the message (25 ASCII characters, right-padded with spaces).
    #[open_protocol_field(list, length = 25, amount = 4)]
    pub lines: Vec<String>,
}

/// Flash green light on tool.
/// The tool’s green light will flash until the operator pushes the trigger.
#[derive(Debug, Default, Eq, PartialEq, OpenProtocolEncode, OpenProtocolDecode, OpenProtocolMessage)]
#[open_protocol_message(MID = 113, revision = 1)]
pub struct MID0113rev1 {
    // No additional fields for this MID.
}
