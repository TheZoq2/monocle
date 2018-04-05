use data;

#[derive(Debug, Serialize)]
pub struct RealReading {
    pub channel1: bool,
    pub channel2: bool,
    pub time: f64
}

impl RealReading {
    pub fn from_reading(frequency_hertz: u32, reading: data::Reading) -> Self {
        Self {
            channel1: reading.state.channel1(),
            channel2: reading.state.channel2(),
            time: (reading.time as f64) / (frequency_hertz / 1_000_000) as f64,
        }
    }
}
