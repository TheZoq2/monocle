extern crate serial;
extern crate api;
extern crate ssmarshal;

use api::data;
use ssmarshal::deserialize;

use std::{env, io};

use std::io::prelude::*;
use serial::prelude::*;

fn main() {
    for arg in env::args_os().skip(1) {
        let mut port = init_serial_port(&arg).unwrap();
        let mut data_buffer: Vec<u8> = vec!();

        loop {
            read_serial_port_data(&mut port, &mut data_buffer).unwrap();
            println!("{}", data_buffer.len());
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

fn decode_readings(data: &mut Vec<u8>) -> Result<(), something> {
    
}
