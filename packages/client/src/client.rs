use crate::network::Network;
use bytes::{BufMut, BytesMut};
use flume::{bounded, Receiver, Sender};
use open_protocol::messages::communication::MID0001rev7;
use open_protocol::messages::keep_alive::MID9999rev1;
use open_protocol::{Header, OpenProtocolMessage};
use open_protocol_codec::encode::{Encode, Encoder};
use open_protocol_codec::{decode, encode};
use std::collections::VecDeque;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::str::FromStr;
use std::time::Duration;
use thiserror;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::{Instant, Sleep};
use tokio::{select, time};

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Requests are done")]
    RequestsDone,
    #[error("Decode error: {0}")]
    DecodeError(#[from] decode::Error),
    #[error("Encode error: {0}")]
    EncodeError(#[from] encode::Error),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

#[derive(Debug)]
pub enum Event {
    Incoming(OpenProtocolMessage),
    Outgoing(OpenProtocolMessage),
}

pub struct EventLoop {
    network: Option<Network>,
    requests_rx: Receiver<OpenProtocolMessage>,
    pub requests_tx: Sender<OpenProtocolMessage>,
    pending: VecDeque<OpenProtocolMessage>, // should not be added to yet...
    events: VecDeque<Event>,
    write_buf: BytesMut,
    keepalive_timeout: Option<Pin<Box<Sleep>>>,
}

impl EventLoop {
    pub fn new(socket: TcpStream) -> Self {
        let (requests_tx, requests_rx) = bounded(1000);
        let pending = VecDeque::with_capacity(1000);
        let events = VecDeque::with_capacity(1000);

        Self {
            requests_tx,
            requests_rx,
            pending,
            events,
            write_buf: BytesMut::with_capacity(10 * 1024),
            network: Some(Network::new(socket)),
            keepalive_timeout: None,
        }
    }

    async fn select(&mut self) -> Result<Event, ConnectionError> {
        // let network = self.network.as_mut().unwrap();
        // let await_acks = self.state.await_acks;

        // let network = self.network.as_mut().unwrap();

        let inflight_full = false; //self.state.inflight >= self.state.max_outgoing_inflight;
        let collision = false; //self.state.collision.is_some();

        if let Some(event) = self.events.pop_front() {
            return Ok(event);
        }

        select! {
            o = next_request(
                &mut self.pending,
                &self.requests_rx,
                Duration::ZERO
            ), if !self.pending.is_empty() || (!inflight_full && !collision) => match o {
                Ok(request) => {
                    self.handle_outgoing_packet(request)?;
                    self.network.as_mut().unwrap().flush(&mut self.write_buf).await?;
                    Ok(self.events.pop_front().unwrap())
                }
                Err(_) => Err(ConnectionError::RequestsDone),
            },

            o = self.network.as_mut().unwrap().read(&mut self.events) => {
                o?;
                // flush all the acks and return first incoming packet
                self.network.as_mut().unwrap().flush(&mut self.write_buf).await?;
                Ok(self.events.pop_front().unwrap())
            },

            _ = self.keepalive_timeout.as_mut().unwrap() => {
                let timeout = self.keepalive_timeout.as_mut().unwrap();
                timeout.as_mut().reset(Instant::now() + Duration::from_secs(5));

                self.handle_outgoing_packet(OpenProtocolMessage::MID9999rev1(MID9999rev1 {}))?;
                self.network.as_mut().unwrap().flush(&mut self.write_buf).await?;
                Ok(self.events.pop_front().unwrap())
            }
        }
    }

    fn handle_outgoing_packet(&mut self, request: OpenProtocolMessage) -> Result<(), ConnectionError> {
        let mut payload_encoder = Encoder::new();
        request.encode_payload(&mut payload_encoder)?;

        let (mid, revision) = request.mid_revision();
        let header = Header {
            mid,
            revision: Some(revision),
            length: (payload_encoder.len() as u16) + 20,
            ..Default::default()
        };
        let mut header_encoder = Encoder::new();
        header.encode(&mut header_encoder)?;

        self.write_buf.extend(header_encoder.as_slice());
        self.write_buf.extend(payload_encoder.as_slice());
        self.write_buf.put_u8(0x0);

        self.events.push_back(Event::Outgoing(request));
        Ok(())
    }

    pub async fn poll(&mut self) -> Result<Event, ConnectionError> {
        if self.keepalive_timeout.is_none() {
            self.keepalive_timeout = Some(Box::pin(time::sleep(Duration::from_secs(5))));
        }

        match self.select().await {
            Ok(v) => Ok(v),
            Err(e) => {
                // self.clean();
                Err(e)
            }
        }
    }
}


async fn next_request(
    pending: &mut VecDeque<OpenProtocolMessage>,
    rx: &Receiver<OpenProtocolMessage>,
    pending_throttle: Duration,
) -> Result<OpenProtocolMessage, ConnectionError> {
    if !pending.is_empty() {
        time::sleep(pending_throttle).await;
        Ok(pending.pop_front().unwrap())
    } else {
        match rx.recv_async().await {
            Ok(r) => Ok(r),
            Err(_) => Err(ConnectionError::RequestsDone),
        }
    }
}

pub async fn connect() -> (Sender<OpenProtocolMessage>, EventLoop) {
    let socket = TcpStream::connect(SocketAddr::from_str("127.0.0.1:4545").unwrap())
        .await
        .unwrap();

    let event_loop = EventLoop::new(socket);
    let sender = event_loop.requests_tx.clone();

    sender.send_async(OpenProtocolMessage::MID0001rev1(MID0001rev7 { keep_alive: None })).await.unwrap();

    (sender, event_loop)
}
