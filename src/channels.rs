use stm32f103xx::EXTI;

macro_rules! enable_channel {
    ($exti:ident, $mr:ident, $tr:ident) => {
        // Block to make the macro valid in expression contexts
        {
            // Mask the exti
            $exti.imr.modify(|_r, w| w.$mr().set_bit());
            // Trigger on both rising and falling edge
            $exti.rtsr.modify(|_r, w| w.$tr().set_bit());
            $exti.ftsr.modify(|_r, w| w.$tr().set_bit());
        }
    }
}

pub enum Error {
    NoSuchChannel(u8)
}

pub fn enable_channel(exti: &EXTI, index: u8) -> Result<(), Error> {
    match index {
        0 => enable_channel!(exti, mr8, tr8),
        1 => enable_channel!(exti, mr9, tr9),
        _ => return Err(Error::NoSuchChannel(index))
    }
    Ok(())
}
