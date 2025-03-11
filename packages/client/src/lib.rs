use std::io;
use thiserror;
use open_protocol::{decode, encode};

mod client;
mod network;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO Error")]
    IoError(#[from] io::Error),
    #[error("Error decoding Open Protocol message")]
    DecodeError(#[from] decode::Error),
    #[error("Error encoding Open Protocol message")]
    EncodeError(#[from] encode::Error),
}

pub type Result<T> = std::result::Result<T, Error>;


#[cfg(test)]
mod tests {
}
