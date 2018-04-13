use api::data;

#[derive(Debug, Serialize)]
pub struct RealReading {
    pub values: Vec<bool>,
    pub time: f64
}

impl RealReading {
    pub fn from_reading(frequency_hertz: u32, reading: data::Reading) -> Self {
        Self {
            values: vec!(reading.state.channel1(), reading.state.channel2()),
            time: (reading.time as f64) / (frequency_hertz / 1_000_000) as f64,
        }
    }
}
