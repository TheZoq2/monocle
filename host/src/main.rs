extern crate serial;
extern crate ssmarshal;
extern crate websocket;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;


use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

mod types;
mod serial_reader;
mod websockets;
mod data;

use types::RealReading;

use data::{ClientHostMessage};

fn processing_thread(
    message_receiver: Receiver<ClientHostMessage>,
    reading_sender: Sender<RealReading>
) {
    let mut frequency = None;
    loop {
        let received = message_receiver.recv()
            .expect("sender disconnected");

        match received {
            ClientHostMessage::FrequencyHertz(val) => {
                frequency = Some(val);
            },
            ClientHostMessage::Reading(val) => {
                if let Some(frequency) = frequency {
                    reading_sender.send(RealReading::from_reading(frequency, val)).unwrap();
                }
            },
            ClientHostMessage::Reset(_) => {
                println!("Reset operation is not currently handled");
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
