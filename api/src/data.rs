#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientHostMessage {
    Reading(Reading),
    FrequencyHertz(u32),
    Reset(u8) // Reset the specified channel readings
}
