use crate::decode::Decode;
use crate::encode::Encode;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageType {
    Data,
    RequestExtraData,
    SubscribeExtraData,
    UnsubscribeExtraData,
}

pub trait Message: Encode + Decode {
    fn mid() -> u16;
    fn to_mid(&self) -> u16;

    fn revision() -> u16 { 1 }
    fn to_revision(&self) -> u16 { 1 }

    fn message_type() -> MessageType { MessageType::Data }
    fn to_message_type(&self) -> MessageType { MessageType::Data }
}
