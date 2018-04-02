extern crate serial;
extern crate api;
extern crate ssmarshal;

use api::data::{self, ClientHostMessage};
use ssmarshal::deserialize;

use std::{env, io};
use std::io::Write;
use std::net::TcpStream;

use serial::prelude::*;

use std::sync::mpsc::{channel, Sender, Receiver};

const DATALOGGER_ADDRESS: &str = "localhost:2000";


#[derive(Debug)]
struct RealReading {
    channel1: bool,
    channel2: bool,
    time: f64
}

impl RealReading {
    fn from_reading(frequency_hertz: u32, reading: data::Reading) -> Self {
        Self {
            channel1: reading.state.channel1(),
            channel2: reading.state.channel2(),
            time: (reading.time as f64) / (frequency_hertz / 1_000_000) as f64,
        }
    }
}

fn send_reading(reading_name: &str, value: u8, timestamp: f64) {
    let mut stream = TcpStream::connect(DATALOGGER_ADDRESS)
        .expect("Failed to connect to datalogger");

    let message = format!("{}:{}:{}", reading_name, value, timestamp);
    stream.write(&message.as_bytes()).expect("Failed to write message");
}

fn handle_real_reading(reading: RealReading) {
    // Convert the reading into separate channels for sending to the server
    send_reading("channel1", reading.channel1 as u8, reading.time);
    send_reading("channel2", reading.channel2 as u8, reading.time);

    println!("{:?}", reading);
}

fn processing_thread(
    reading_receiver: Receiver<ClientHostMessage>
) {
    let mut frequency = None;
    loop {
        let received = reading_receiver.recv()
            .expect("sender disconnected");

        match received {
            ClientHostMessage::FrequencyHertz(val) => {
                frequency = Some(val);
            },
            ClientHostMessage::Reading(val) => {
                if let Some(frequency) = frequency {
                    handle_real_reading(RealReading::from_reading(frequency, val));
                }
            }
        }
    }
}

fn main() {
    let (tx, rx) = channel();


    ::std::thread::spawn(|| processing_thread(rx));
    serial_reader_thread(tx);
}

fn serial_reader_thread(reading_sender: Sender<data::ClientHostMessage>) {
    let port_name = env::args_os().skip(1).next()
        .expect("You need to specify a serial port");
    let mut port = init_serial_port(&port_name)
        .expect("Failed to open serial port");
    let mut data_buffer: Vec<u8> = vec!();


    port.write(&[0]).unwrap();

    loop {
        read_serial_port_data(&mut port, &mut data_buffer).unwrap();
        let decoded = decode_messages(&mut data_buffer).unwrap();
        for reading in decoded {
            reading_sender.send(reading)
                .expect("Reader disconnected");
        }
    }
}

fn init_serial_port(name: &::std::ffi::OsString) -> io::Result<serial::SystemPort> {
        let mut port = serial::open(&name).unwrap();
        port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud115200)?;
            Ok(())
        })?;

        Ok(port)
}

fn read_serial_port_data<T: SerialPort>(port: &mut T, buf: &mut Vec<u8>) -> io::Result<()> {
    let mut internal_buf = [0; 100];
    let read_amount = loop {
        match port.read(&mut internal_buf) {
            Ok(val) => break val,
            Err(e) => {
                match e.kind() {
                    ::std::io::ErrorKind::TimedOut => continue,
                    _ => return Err(e)
                }
            }
        };
    };

    for b in internal_buf[..read_amount].iter() {
        buf.push(*b);
    }

    Ok(())
}

fn decode_messages(data: &mut Vec<u8>)
    -> Result<Vec<data::ClientHostMessage>, ssmarshal::Error> 
{
    let mut result = vec!();
    loop {
        match deserialize::<data::ClientHostMessage>(data) {
            Ok((reading, bytes_used)) => {
                result.push(reading);
                data.drain(0..bytes_used).collect::<Vec<_>>();
            },
            Err(ssmarshal::Error::EndOfStream) => {
                break;
            }
            Err(e) => {
                return Err(e)
            }
        }
    }

    Ok(result)
}
