use std::collections::VecDeque;
use std::io;
use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use open_protocol::{Header, Message};
use open_protocol::decode::{self, Decoder, Decode};
use crate::client::{ConnectionError, Event};

pub struct Network {
    pub socket: TcpStream,
    pub read_buf: BytesMut,
}

impl Network {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            read_buf: BytesMut::with_capacity(10 * 1024),
        }
    }

    async fn read_bytes(&mut self, required: usize) -> io::Result<usize> {
        let mut total_read = 0;
        loop {
            let read = self.socket.read_buf(&mut self.read_buf).await?;

            if 0 == read {
                return if self.read_buf.is_empty() {
                    Err(io::Error::new(
                        io::ErrorKind::ConnectionAborted,
                        "connection closed by peer",
                    ))
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::ConnectionReset,
                        "connection reset by peer",
                    ))
                };
            }

            total_read += read;
            if total_read >= required {
                return Ok(total_read);
            }
        }
    }

    pub async fn read(&mut self, events: &mut VecDeque<Event>) -> io::Result<()> {
        loop {
            let required = match read_message(&mut self.read_buf) {
                Ok(message) => {
                    events.push_back(Event::Incoming(message));
                    return Ok(());
                },
                Err(decode::Error::InsufficientBytes { have, need }) => need - have,
                Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string())),
            };

            self.read_bytes(required).await?;
        }
    }

    pub async fn flush(&mut self, write_buf: &mut BytesMut) -> Result<(), ConnectionError> {
        if write_buf.is_empty() {
            return Ok(());
        }

        self.socket.write_all(&write_buf[..]).await?;
        write_buf.clear();
        Ok(())
    }
}

fn read_message(stream: &mut BytesMut) -> decode::Result<Message> {
    if stream.len() < 20 {
        return Err(decode::Error::InsufficientBytes { have: stream.len(), need: 20 });
    }

    let mut decoder = Decoder::new(&stream[..]);
    let header = Header::decode(&mut decoder)?;

    if stream.len() < (header.length as usize) {
        return Err(decode::Error::InsufficientBytes { have: stream.len(), need: header.length as usize });
    }

    let message = Message::decode_payload(header.mid, header.revision_number(), &mut decoder)?;
    decoder.expect_char(0x0 as char)?;
    stream.advance((header.length + 1) as usize);
    Ok(message)
}
