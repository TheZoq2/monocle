#![deny(warnings)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_semihosting;
//#[macro_use(exception, interrupt)]
extern crate stm32f103xx;

use stm32f103xx::GPIOC;
use stm32f103xx::RCC;

fn main() {
    // RCC IOPORT C Enable
    // Clock enabled for GPIOC
    unsafe {
        (*RCC.get()).apb2enr.modify(|_, w| w.iopcen().enabled());
    }

    // BIT Set Reset Register - BitSet for Bit13
    // Output register is Set as HIGH
    unsafe {
        (*GPIOC.get()).bsrr.write(|w| w.bs13().set());
    }

    // GPIO MODER register & CNF register - BIT 13 Set 
    // Configure port as PUSH-PULL HIGH speed OUTPUT port
    unsafe {
        (*GPIOC.get()).crh.modify(|_, w| w.mode13().output().cnf13().push());
    }

    // BIT Set Reset Register - BitReset for Bit13
    // Output register is Set as LOW and Active Low LED lights up
    unsafe {
        (*GPIOC.get()).bsrr.write(|w| w.br13().reset());
    }   
}