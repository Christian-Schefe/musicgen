pub struct Note {
    start_time: f64,
    duration: f64,
    pitch: f64,
    velocity: f64,
}

impl Note {
    pub fn new(start_time: f64, duration: f64, pitch: f64, velocity: f64) -> Self {
        Self {
            start_time,
            duration,
            pitch,
            velocity,
        }
    }
}
