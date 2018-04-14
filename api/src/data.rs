
#[derive(Debug, PartialEq, Clone)]
pub struct State {
    data: u8
}

impl State {
    pub fn new(channel1: bool, channel2: bool) -> Self {
        Self {
            data: (channel1 as u8) | (channel2 as u8) << 1
        }
    }

    pub fn channel1(&self) -> bool {
        self.data & 1 == 1
    }
    pub fn channel2(&self) -> bool {
        (self.data >> 1) & 1 == 1
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Reading {
    pub state: State,
    pub time: u32
}

impl Reading {
    pub fn new(time: u32, channel1: bool, channel2: bool) -> Self {
        Self {
            state: State::new(channel1, channel2),
            time
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ClientHostMessage {
    Reading(Reading),
    FrequencyHertz(u32),
    Reset(u8), // Reset the specified channel readings
    CurrentTime(u32),
}

////////////////////////////////////////////////////////////////////////////////
//                 Encoding and decoding functions
////////////////////////////////////////////////////////////////////////////////

/**
  Decodes a `$enum_name` based on the `$prefix` and the payload ($bytes).
  Each enum type is represented by `$prefix_value => (variant_name, inner_value_type)`

  Returns the amount of bytes used *for* decoding the payload and the decoded value. You
  need to manually add the amount of bytes used for the prefix to the final result
*/
macro_rules! decode_enum_variants {
    (
        $prefix:expr, $bytes:expr, $enum_name:ident 
        {
            $( $prefix_value:expr => ($variant:ident, $inner_type:ident) ),* 
        }
    ) => {
        match $prefix {
            $(
                $prefix_value => {
                    let (len, val) = $inner_type::decode($bytes)?;
                    Ok((len, $enum_name::$variant(val)))
                }
            )*,
            byte => Err(DecodingError::UnexpectedByte(byte, "unexpected prefix for $enum_name"))
        }
    }
}

pub trait Message<S> {
    fn encode(&self, buffer: &mut [u8]) -> Result<usize, EncodingError>;
    fn decode(bytes: &[u8]) -> Result<(usize, S), DecodingError>;
}

#[derive(Debug, PartialEq)]
pub enum EncodingError {
    BufferToSmall,
}
#[derive(Debug, PartialEq)]
pub enum DecodingError {
    EndOfBytes,
    UnexpectedByte(u8, &'static str)
}

impl Message<Self> for State {
    fn encode(&self, buff: &mut [u8]) -> Result<usize, EncodingError> {
        if buff.len() < 1 {
            return Err(EncodingError::BufferToSmall);
        }
        buff[0] = self.data;
        Ok(1)
    }
    fn decode(bytes: &[u8]) -> Result<(usize, Self), DecodingError> {
        let byte = *bytes.get(0).ok_or(DecodingError::EndOfBytes)?;
        if byte > 0b11 {
            return Err(DecodingError::UnexpectedByte(byte, "State byte must be > 0b11"));
        }
        Ok((1, Self{data: byte}))
    }
}

impl Message<Self> for Reading {
    fn encode(&self, buffer: &mut [u8]) -> Result<usize, EncodingError> {
        let used_bytes = self.state.encode(buffer)?;
        Ok(used_bytes + self.time.encode(&mut buffer[used_bytes..])?)
    }

    fn decode(bytes: &[u8]) -> Result<(usize, Self), DecodingError> {
        let (used_bytes_state, state) = State::decode(bytes)?;
        let (used_bytes_time, time) = u32::decode(&bytes[used_bytes_state..])?;

        Ok((used_bytes_state + used_bytes_time, Reading{state, time}))
    }
}

impl Message<Self> for ClientHostMessage {
    fn encode(&self, buff: &mut [u8]) -> Result<usize, EncodingError> {
        if buff.len() < 1 {
            return Err(EncodingError::BufferToSmall);
        }

        buff[0] = match *self {
            ClientHostMessage::Reading(_) => 1,
            ClientHostMessage::FrequencyHertz(_) => 2,
            ClientHostMessage::Reset(_) => 3,
            ClientHostMessage::CurrentTime(_) => 4,
        };

        let remainder = &mut buff[1..];

        let used_bytes = match *self {
            ClientHostMessage::Reading(ref val) => val.encode(remainder)?,
            ClientHostMessage::FrequencyHertz(ref val) => val.encode(remainder)?,
            ClientHostMessage::Reset(ref val) => val.encode(remainder)?,
            ClientHostMessage::CurrentTime(ref val) => val.encode(remainder)?,
        };

        Ok(used_bytes + 1)
    }

    fn decode(bytes: &[u8]) -> Result<(usize, Self), DecodingError> {
        if bytes.len() < 1 {
            return Err(DecodingError::EndOfBytes);
        }

        let (len, val) = decode_enum_variants!{bytes[0], &bytes[1..], ClientHostMessage {
            1 => (Reading, Reading),
            2 => (FrequencyHertz, u32),
            3 => (Reset, u8),
            4 => (CurrentTime, u32)
        }}?;

        Ok((len + 1, val))
    }
}

impl Message<Self> for u32 {
    fn encode(&self, buff: &mut [u8]) -> Result<usize, EncodingError> {
        if buff.len() < 4 {
            return Err(EncodingError::BufferToSmall);
        }
        buff[0] = *self as u8;
        buff[1] = (*self >> 8) as u8;
        buff[2] = (*self >> 16) as u8;
        buff[3] = (*self >> 24) as u8;
        Ok(4)
    }

    fn decode(bytes: &[u8]) -> Result<(usize, Self), DecodingError> {
        if bytes.len() < 4 {
            return Err(DecodingError::EndOfBytes);
        }
        let mut result = bytes[0] as u32;
        result |= (bytes[1] as u32) << 8;
        result |= (bytes[2] as u32) << 16;
        result |= (bytes[3] as u32) << 24;
        Ok((4, result))
    }
}

impl Message<Self> for u8 {
    fn encode(&self, buff: &mut [u8]) -> Result<usize, EncodingError> {
        if buff.len() < 1 {
            return Err(EncodingError::BufferToSmall);
        }
        buff[0] = *self as u8;
        Ok(1)
    }

    fn decode(bytes: &[u8]) -> Result<(usize, Self), DecodingError> {
        if bytes.len() < 1 {
            return Err(DecodingError::EndOfBytes);
        }
        Ok((1, bytes[0]))
    }
}





#[cfg(test)]
mod encode_decode_tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum EncodeDecodeFailure<T: ::core::fmt::Debug + PartialEq> {
        NoMatch(T),
        EncodingError(EncodingError),
        DecodingError(DecodingError),
        UsedWrongAmountOfBytes(usize, usize)
    }

    macro_rules! test_encode_decode {
        ($type:ident, $value:expr, $buffer_size:expr) => {
            {
                // Val is passed to force the inference from assuming the wrong type
                // TODO: Replace the whole closure with a try/catch block in rust 2018
                (|val: $type| -> Result<(), EncodeDecodeFailure<$type>> {
                    let mut buffer = [0; $buffer_size];

                    let encoded_len = val.encode(&mut buffer)
                        .map_err(|e| EncodeDecodeFailure::EncodingError(e))?;

                    let (decoded_len, decoded) = $type::decode(&buffer)
                        .map_err(|e| EncodeDecodeFailure::DecodingError(e))?;

                    if decoded_len != encoded_len {
                        return Err(
                            EncodeDecodeFailure::UsedWrongAmountOfBytes(encoded_len, decoded_len)
                        )
                    }


                    if decoded == val {
                        Ok(())
                    }
                    else {
                        Err(EncodeDecodeFailure::NoMatch(decoded))
                    }
                })($value)
            }
        }
    }

    #[test]
    fn u32_test() {
        assert_eq!(test_encode_decode!(u32, 0, 4), Ok(()));
        assert_eq!(test_encode_decode!(u32, 12345678, 4), Ok(()));
    }

    #[test]
    fn reading_test() {
        assert_eq!(test_encode_decode!(Reading, Reading::new(123412, true, true), 6), Ok(()));
        assert_eq!(test_encode_decode!(Reading, Reading::new(123412, false, false), 6), Ok(()));
    }

    #[test]
    fn state_test() {
        assert_eq!(test_encode_decode!(State, State::new(true, true), 1), Ok(()));
        assert_eq!(test_encode_decode!(State, State::new(false, true), 1), Ok(()));
        assert_eq!(test_encode_decode!(State, State::new(true, false), 1), Ok(()));
        assert_eq!(test_encode_decode!(State, State::new(false, false), 1), Ok(()));
    }

    #[test]
    fn client_host_message_test() {
        let reading = Reading::new(123412, false, false);
        assert_eq!(test_encode_decode!(
            ClientHostMessage,
            ClientHostMessage::Reading(reading.clone()),
            6
        ), Ok(()));
        assert_eq!(test_encode_decode!(
            ClientHostMessage,
            ClientHostMessage::FrequencyHertz(12345),
            6
        ), Ok(()));
        assert_eq!(test_encode_decode!(
            ClientHostMessage,
            ClientHostMessage::Reset(5),
            6
        ), Ok(()));
        assert_eq!(test_encode_decode!(
            ClientHostMessage,
            ClientHostMessage::CurrentTime(5),
            6
        ), Ok(()));
    }
}
