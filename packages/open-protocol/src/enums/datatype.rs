use open_protocol_codec_proc_macro::{OpenProtocolDecode, OpenProtocolEncode};

#[derive(Debug, Default, Eq, PartialEq, Clone, OpenProtocolEncode, OpenProtocolDecode)]
pub enum DataType {
    /// **Unsigned Integer (UI)**
    /// The value is an unsigned integer. The number of digits is defined with the Length parameter.
    #[default]
    #[open_protocol_value(number = 1)]
    UnsignedInteger,

    /// **Signed Integer (I)**
    /// The value is a signed integer. The number of digits is defined with the Length parameter.
    #[open_protocol_value(number = 2)]
    SignedInteger,

    /// **Float (F)**
    /// The value is sent as a float in a flexible format such as "12.12", "10025.1234", or "-57.5".
    /// The sender determines the number of decimals to send.
    #[open_protocol_value(number = 3)]
    Float,

    /// **String (S)**
    /// The value is sent as ASCII characters, and the length of the data fits the actual string length.
    /// Strings may contain spaces (ASCII 0x20).
    #[open_protocol_value(number = 4)]
    String,

    /// **Timestamp (T)**
    /// The value represents a time stamp in the format `YYYY-MM-DD:HH:MM:SS` (19 ASCII characters).
    #[open_protocol_value(number = 5)]
    Timestamp,

    /// **Boolean (B)**
    /// A boolean value represented as one ASCII digit: `0 = FALSE`, `1 = TRUE`.
    #[open_protocol_value(number = 6)]
    Boolean,

    /// **Hexadecimal (H)**
    /// A hexadecimal value sent as ASCII characters, e.g., `"A24CD3"`.
    #[open_protocol_value(number = 7)]
    Hexadecimal,

    /// **Plotting Point (PL1)**
    /// A plotting point consisting of a Float Array (`FA`) of one pair of float values (Y, X).
    #[open_protocol_value(number = 8)]
    PlotPointPL1,

    /// **Double Arrow Line (PL2)**
    /// A plotting point consisting of an `FA` of two pairs of float values (Y, X).
    #[open_protocol_value(number = 9)]
    PlotPointPL2,

    /// **Window Plot (PL4)**
    /// A plotting point consisting of an `FA` of four pairs of float values (Y, X).
    #[open_protocol_value(number = 10)]
    PlotPointPL4,

    /// **Float Array (FA)**
    /// An array of float values, each sent as 8 ASCII characters.
    /// Negative values start with a `'-'` sign.
    /// Precision varies, omitting the decimal point for large values.
    /// Examples: `"-1234567"`, `"001.1205"`, `"-123.789"`.
    #[open_protocol_value(number = 50)]
    FloatArray,

    /// **Unsigned Integer Array (UA)**
    /// An array of unsigned integers, each sent as 8 ASCII characters.
    /// Examples: `"12345678"`, `"00001234"`, `"00200000"`.
    #[open_protocol_value(number = 51)]
    UnsignedIntegerArray,

    /// **Signed Integer Array (IA)**
    /// An array of signed integers, each sent as 8 ASCII characters.
    /// Negative values start with a `'-'` sign.
    /// Examples: `"12345678"`, `"-1234567"`, `"00200000"`, `"10200000"`.
    #[open_protocol_value(number = 52)]
    SignedIntegerArray,
}
