macro_rules! open_protocol_messages {
    ( $( ($mid:expr, $rev:expr) => $msg:ty ),* $(,)? ) => {
        ::paste::paste!(
            /// Enum representing any possible OpenProtocol message.
            #[derive(Debug, Eq, PartialEq)]
            pub enum OpenProtocolMessage {
                $(
                    [<MID $mid rev $rev>]($msg),
                )*
            }

            impl OpenProtocolMessage {
                fn mid_revision(&self) -> (u16, u16) {
                    match self {
                        $(
                            OpenProtocolMessage::[<MID $mid rev $rev>](_) => ($mid, $rev),
                        )*
                    }
                }

                fn encode_payload(&self, encoder: &mut ::open_protocol_codec::encode::Encoder) -> ::open_protocol_codec::encode::Result<()> {
                    match self {
                        $(
                            OpenProtocolMessage::[<MID $mid rev $rev>](payload) => payload.encode(encoder)?,
                        )*
                    };
                    Ok(())
                }

                fn decode_payload(mid: u16, revision: u16, decoder: &mut ::open_protocol_codec::decode::Decoder) -> ::open_protocol_codec::decode::Result<Self> {
                    Ok(match (mid, revision) {
                        $(
                            ($mid, $rev) => OpenProtocolMessage::[<MID $mid rev $rev>]($msg::decode(decoder)?),
                        )*
                        _ => return Err(::open_protocol_codec::decode::Error::NotImplemented),
                    })
                }

                fn decode_message(decoder: &mut ::open_protocol_codec::decode::Decoder) -> ::open_protocol_codec::decode::Result<(Header, Self)> {
                    let header = Header::decode(decoder)?;
                    let payload = Self::decode_payload(header.mid, header.revision_number(), decoder)?;
                    decoder.expect_char('\0')?;
                    Ok((header, payload))
                }
            }
        );
    };
}

pub(crate) use open_protocol_messages;
