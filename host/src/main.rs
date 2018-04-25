extern crate serial;
extern crate websocket;
extern crate api;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;


use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

mod types;
mod serial_reader;
mod websockets;

use types::{RealReading, WebMessage, time_to_microseconds};

use api::data::{ClientHostMessage};

fn processing_thread(
    hw_message_receiver: Receiver<ClientHostMessage>,
    web_message_sender: Sender<WebMessage>
) {
    let mut frequency = None;
    loop {
        let received = hw_message_receiver.recv()
            .expect("sender disconnected");

        match received {
            ClientHostMessage::FrequencyHertz(val) => {
                println!("Got frequency value: {}", val);
                frequency = Some(val);
            },
            ClientHostMessage::Reading(val) => {
                if let Some(frequency) = frequency {
                    let message =
                        WebMessage::Reading(RealReading::from_reading(frequency, val));

                    web_message_sender.send(message).unwrap();
                }
            },
            ClientHostMessage::Reset(_) => {
                println!("Reset operation is not currently handled");
            },
            ClientHostMessage::CurrentTime(time_u32) => {
                if let Some(frequency) = frequency {
                    let message = WebMessage::CurrentTime(time_to_microseconds(
                        frequency,
                        time_u32
                    ));
                    web_message_sender.send(message).unwrap();
                }
            }
        }
    }
}

fn main() {
    let (message_tx, message_rx) = channel();
    let (reading_tx, reading_rx) = channel();


    thread::spawn(|| processing_thread(message_rx, reading_tx));
    thread::spawn(|| websockets::server("0.0.0.0:8765", reading_rx));

    serial_reader::serial_reader_thread(message_tx);
}
