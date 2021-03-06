use serial;

use std::{env, io};

use serial::prelude::*;
use std::sync::mpsc::Sender;
use std::io::Write;

use api::data;
use api::Message;

pub fn serial_reader_thread(reading_sender: Sender<data::ClientHostMessage>) {
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
        //settings.set_baud_rate(serial::Baud115200)?;
        settings.set_baud_rate(serial::Baud115200)?;
        settings.set_flow_control(serial::FlowControl::FlowNone);
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
    -> Result<Vec<data::ClientHostMessage>, data::DecodingError>
{
    let mut result = vec!();
    loop {
        match data::ClientHostMessage::decode(data) {
            Ok((bytes_used, reading)) => {
                result.push(reading);
                data.drain(0..bytes_used).collect::<Vec<_>>();
            },
            Err(data::DecodingError::EndOfBytes) => {
                break;
            }
            Err(data::DecodingError::IncorrectPrefixByte(val)) => {
                let dropped = data.remove(0);
                println!("Got wrong prefix: {:x}, dropping byte", val);
            }
            Err(e) => {
                return Err(e)
            }
        }
    }

    Ok(result)
}
