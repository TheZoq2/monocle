use data;
use util;
use error::{Result, Error};

pub enum HardwareMessage {
    Frequency(u32),
    Reading(data::Reading),
}

impl HardwareMessage {
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize> {
        let length = self.check_message_length(buf)?;
        match *self {
            HardwareMessage::Frequency(freq) => {
                buf[0] = 1;

                util::encode_u32(freq, &mut buf[1..]);
            },
            HardwareMessage::Reading(ref reading) => {
                buf[0] = 2;
                reading.encode_to_buffer(&mut buf[1..])?;
            }
        }
        Ok(length)
    }

    fn check_message_length(&self, buf: &mut[u8]) -> Result<usize> {
        let length = match *self {
            HardwareMessage::Frequency(_) => 5,
            HardwareMessage::Reading(_) => 6,
        };

        if buf.len() < length {
            Err(Error::BufferTooSmall(buf.len(), length))
        }
        else {
            Ok(length)
        }
    }

    pub fn decode() {
        
    }
}

#[repr(u8)]
pub enum ControlMessage {
    GetFrequency
}


#[cfg(test)]
mod encoding_tests {
    use super::*;

    #[test]
    fn encoding_frequency_works() {
        let hwmessage = HardwareMessage::Frequency(10);
        let mut buffer = [0;5];
        hwmessage.encode(&mut buffer).unwrap();

        assert_eq!(buffer, [1,0,0,0,10]);
    }

    #[test]
    fn encoding_reading_works() {
        let hwmessage = HardwareMessage::Reading(data::Reading::new(10,true,false));
        let mut buffer = [0; 6];
        hwmessage.encode(&mut buffer).unwrap();

        assert_eq!(buffer, [2,1,0,0,0,10]);
    }

    #[test]
    fn encoding_into_undersized_buffers() {
        let hwmessage = HardwareMessage::Reading(data::Reading::new(10,false,true));
        let mut buffer = [0;5];
        assert!(hwmessage.encode(&mut buffer).is_err());

        let hwmessage = HardwareMessage::Frequency(10);
        let mut buffer = [0;4];
        assert!(hwmessage.encode(&mut buffer).is_err());
    }
}
