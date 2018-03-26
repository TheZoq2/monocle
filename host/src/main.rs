extern crate serial;
extern crate api;
extern crate ssmarshal;

use api::data;
use ssmarshal::deserialize;

use std::{env, io};
use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;

fn main() {
    for arg in env::args_os().skip(1) {
        let mut port = serial::open(&arg).unwrap();
        loop {
            run_host(&mut port).unwrap().map(|val| {
                println!("{:?}", val);
            });
        }
    }
}

fn run_host<T: SerialPort>(port: &mut T) -> io::Result<Option<api::data::Reading>> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?;
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(2000))?;
    let mut buf = [0;10];

    let read_amount = match port.read(&mut buf) {
        Ok(val) => val,
        Err(e) => {
            match e.kind() {
                ::std::io::ErrorKind::TimedOut => return Ok(None),
                _ => return Err(e)
            }
        }
    };

    // TODO: Proper error handling
    Ok(Some(deserialize::<data::Reading>(&buf[..read_amount]).unwrap().0))
}
