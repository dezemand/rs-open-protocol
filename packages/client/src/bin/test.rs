/* Note: this file is only used for testing. */

use std::time::Duration;
use flume::Sender;
use tokio::signal::ctrl_c;
use tokio::time;
use open_protocol::messages::communication::MID0003rev1;
use open_protocol::messages::parameter_set::{MID0010rev1, MID0012rev1};
use open_protocol::Message;
use open_protocol_client::client::{connect, Event, EventLoop};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (sender, event_loop) = connect().await;

    let _task = tokio::spawn(run_event_loop(event_loop, sender.clone()));

    ctrl_c().await.unwrap();

    sender.send_async(Message::MID0003rev1(MID0003rev1 {})).await.unwrap();

    time::sleep(Duration::from_secs(1)).await;
}

async fn run_event_loop(mut event_loop: EventLoop, sender: Sender<Message>) {
    loop {
        let event = event_loop.poll().await;
        println!("Event: {:?}", event);

        match event {
            Ok(Event::Incoming(Message::MID0002rev1(message))) => {
                println!("Connected to {}", message.controller_name);

                sender.send_async(Message::MID0010rev1(MID0010rev1 {})).await.unwrap();
            }

            Ok(Event::Incoming(Message::MID0011rev1(message))) => {
                println!("Received {} parameter sets", message.number_of_parameter_sets);
                for id in message.parameter_set_ids {
                    sender.send_async(Message::MID0012rev1(MID0012rev1 { parameter_set_id: id })).await.unwrap();
                }
            }

            Ok(Event::Incoming(Message::MID0013rev1(message))) => {
                println!("Parameter set: {:?}", message);
            }

            // Ok(event) => println!("{:?}", event),

            Err(conn_err) => {
                println!("Errored: {:?}", conn_err);
                return;
            }

            _ => {},
        }
    }
}
