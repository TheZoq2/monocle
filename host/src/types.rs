use api::data;

#[derive(Debug, Serialize)]
pub struct RealReading {
    pub values: Vec<bool>,
    pub time: f64 // In microseconds
}

impl RealReading {
    pub fn from_reading(frequency_hertz: u32, reading: data::Reading) -> Self {
        Self {
            values: vec!(reading.state.channel1(), reading.state.channel2()),
            time: time_to_microseconds(frequency_hertz, reading.time),
        }
    }
}

pub fn time_to_microseconds(frequency_hertz: u32, time: u32) -> f64 {
    (time as f64) / (frequency_hertz / 1_000_000) as f64
}

#[derive(Debug, Serialize)]
pub enum WebMessage {
    Reading(RealReading),
    CurrentTime(f64)
}
