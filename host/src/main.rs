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
            println!("{:?}", run_host(&mut port).unwrap());
        }
    }
}

fn run_host<T: SerialPort>(port: &mut T) -> io::Result<api::data::Reading> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?;
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(2000))?;
    let mut buf = [0;10];

    let read_amount = port.read(&mut buf)?;

    // TODO: Proper error handling
    Ok(deserialize::<data::Reading>(&buf[..read_amount]).unwrap().0)
}
