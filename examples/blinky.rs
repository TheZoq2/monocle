#![deny(warnings)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_semihosting;
//#[macro_use(exception, interrupt)]
extern crate stm32f103xx;

use core::cell::RefCell;
//use core::fmt::Write;

use cortex_m::interrupt::{self, Mutex};
//use cortex_m::peripheral::SystClkSource;
use cortex_m_semihosting::hio::{self, HStdout};
//use stm32f103xx::Interrupt;
use stm32f103xx::GPIOC;
use stm32f103xx::RCC;

static HSTDOUT: Mutex<RefCell<Option<HStdout>>> =
    Mutex::new(RefCell::new(None));

fn main() {
    interrupt::free(|cs| {
        let hstdout = HSTDOUT.borrow(cs);
        if let Ok(fd) = hio::hstdout() {
            *hstdout.borrow_mut() = Some(fd);
        }

        unsafe {
            (*RCC.get()).apb2enr.modify(|_, w| w.iopcen().enabled());
        }

        unsafe {
            (*GPIOC.get()).bsrr.write(|w| w.bs13().set());
            (*GPIOC.get()).crh.modify(|_, w| w.mode13().output().cnf13().push());
        }

        unsafe {
            (*GPIOC.get()).bsrr.write(|w| w.br13().reset());
        }
    });
}