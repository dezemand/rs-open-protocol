macro_rules! open_protocol_messages {
    ( $( MID $mid:literal revision $rev:literal : $msg:ty ),* $(,)? ) => {
        ::paste::paste!(
            /// Enum representing any possible OpenProtocol message.
            #[derive(Debug, Eq, PartialEq)]
            pub enum Message {
                $(
                    [<MID $mid rev $rev>]($msg),
                )*
            }

            impl Message {
                pub fn mid_revision(&self) -> (u16, u16) {
                    match self {
                        $(
                            Message::[<MID $mid rev $rev>](_) => ($mid, $rev),
                        )*
                    }
                }

                pub fn encode_payload(&self, encoder: &mut ::open_protocol_codec::encode::Encoder) -> ::open_protocol_codec::encode::Result<()> {
                    match self {
                        $(
                            Message::[<MID $mid rev $rev>](payload) => payload.encode(encoder)?,
                        )*
                    };
                    Ok(())
                }

                pub fn decode_payload(mid: u16, revision: u16, decoder: &mut ::open_protocol_codec::decode::Decoder) -> ::open_protocol_codec::decode::Result<Self> {
                    Ok(match (mid, revision) {
                        $(
                            ($mid, $rev) => Message::[<MID $mid rev $rev>]($msg::decode(decoder)?),
                        )*
                        _ => return Err(::open_protocol_codec::decode::Error::NotImplemented),
                    })
                }

                pub fn decode_message(decoder: &mut ::open_protocol_codec::decode::Decoder) -> ::open_protocol_codec::decode::Result<(Header, Self)> {
                    let header = Header::decode(decoder)?;

                    if (header.length as usize) > decoder.len() {
                        return Err(::open_protocol_codec::decode::Error::InsufficientBytes { have: decoder.len(), need: header.length as usize })
                    }

                    let payload = Self::decode_payload(header.mid, header.revision_number(), decoder)?;
                    decoder.expect_char('\0')?;
                    Ok((header, payload))
                }
            }
        );
    };
}

pub(crate) use open_protocol_messages;
