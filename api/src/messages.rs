use data;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum HardwareMessage {
    Frequency(u32),
    Reading(data::Reading),
}

#[repr(u8)]
pub enum ControlMessage {
    GetFrequency
}

#[cfg(test)]
mod hwmessage_encode_tests {
    use ssmarshal::deserialize;
    use ssmarshal::serialize;

    use super::*;

    macro_rules! test_encode_decode {
        ($val:expr, $buffer_size:expr, $type:ident) => {
            let message = $val;
            let mut buffer = [0;$buffer_size];
            let bytes = serialize(&mut buffer, &message).unwrap();

            let des = deserialize::<$type>(&buffer[..bytes]).unwrap();
            assert_eq!(des.0, $val);
        }
    }

    #[test]
    fn encodings() {
        test_encode_decode!(HardwareMessage::Frequency(500), 10, HardwareMessage);
        test_encode_decode!(
            HardwareMessage::Reading(data::Reading::new(100,false,false)),
            10,
            HardwareMessage
        );
    }
}
