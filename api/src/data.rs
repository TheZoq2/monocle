#[derive(Debug, PartialEq)]
pub struct State {
    data: u8
}

impl State {
    pub fn new(channel1: bool, channel2: bool) -> Self {
        Self {
            data: (channel1 as u8) | (channel2 as u8) << 1
        }
    }

    pub fn encode(&self) -> u8 {
        self.data
    }
    pub fn decode(byte: u8) -> Self {
        Self {data: byte}
    }

    pub fn channel1(&self) -> bool {
        self.data & 1 == 1
    }
    pub fn channel2(&self) -> bool {
        (self.data >> 1) & 1 == 1
    }
}

#[derive(Debug, PartialEq)]
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

    pub fn decode(bytes: [u8; 5]) -> Self {
        let state = State::decode(bytes[0]);
        let time
            = (bytes[1] as u32) << 24
            | (bytes[2] as u32) << 16
            | (bytes[3] as u32) << 8
            | (bytes[4] as u32);

        Self {
            state,
            time
        }
    }

    pub fn encode(&self) -> [u8; 5] {
        [
            self.state.encode(),
            (self.time >> 24) as u8,
            (self.time >> 16) as u8,
            (self.time >> 8) as u8,
            (self.time) as u8
        ]
    }

}


#[cfg(test)]
mod apitests {
    use super::*;

    #[test]
    fn encode_decode_works() {
        let reading = Reading::new(500, true, true);
        let encoded = reading.encode();

        let decoded = Reading::decode(encoded);

        assert_eq!(reading, decoded);
    }
}
