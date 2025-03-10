use std::net::TcpStream;
use std::io::{self, Write, Read};
use std::time::Duration;
use open_protocol_codec::decode;
use crate::Result;

// pub struct Client {
//     stream: TcpStream,
// }
//
// impl Client {
//     /// Create a new TCP client and connect to the specified address
//     pub fn connect(addr: &str) -> Result<Self> {
//         let stream = TcpStream::connect(addr)?;
//         stream.set_read_timeout(Some(Duration::from_secs(5)))?;
//         stream.set_write_timeout(Some(Duration::from_secs(5)))?;
//         Ok(Self { stream })
//     }
//
//     /// Send a message to the server
//     pub fn send_message(&mut self, message: OutgoingMessage<>) -> Result<()> {
//         let bytes = message.serialize()?;
//         self.stream.write_all(&bytes)?;
//         Ok(())
//     }
//
//     /// Receive a message from the server
//     pub fn receive_message(&mut self) -> Result<Message> {
//         let mut buffer = [0; 1024];
//         let size = self.stream.read(&mut buffer)?;
//         let message = decode::decode(buffer[0..size].to_vec())?;
//         Ok(message)
//     }
// }
