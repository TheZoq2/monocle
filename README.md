# Monocle

A multi-channel digital signal analyser written in rust that runs on a [blue pill board](http://wiki.stm32duino.com/index.php?title=Blue_Pill).

![Picture of prototype](prototype1.jpg "Current prototype")

It currently uses a separate chip for usb communication. Any device that can read serial
data from a source and send it over USB should work.


## Structure

The project consists of 3 parts:

- The code running on the hardware
- A serial reader and web-server running on a PC
- A gui written in elm that runs in a webbrowser

The hardware part reads signals and sends the raw data via serial to the
host PC where the serial reader parses and stores the data. It then sends
the data forward via websockets to the gui.


## Build instructions

First of all, you will need some hardware. Naturally you will need a blue pill board,
or at least some board with a stm32f103 chip. You will also need a separate serial
to usb converter. I use a teensy lc running a simple relay sketch, but you could also
use something like a FTDI adapter.

No hardware diagram is available at the moment so the best way to find the pins
used by the project is to look in the init function in `src/main.rs`. The pins
assigned to `rx` and `tx` are the pins used for serial and the pins assigned
to `pin1` and `pin2` are the ones used for reading data.

Run openocd using `make openocd` and then run `make` to build the project in release
mode and upload it to the device.

The host program is in `host/`. Run it using `cargo run` and specify the file
for the serial reader (usually /dev/ttyACMx or /dev/ttyUSBx).

Run `git submodule init && git submodule update` to pull the graph rendering library

Finally, enter the `host/frontend` directory and run `elm-reactor`. Open `src/Main.elm`
and it should connect to the host server and receive updates.


Based on https://github.com/boseji/rust-bluepill-quickstart.git
