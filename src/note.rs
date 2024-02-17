use fundsp::prelude::midi_hz;

#[derive(Clone)]
pub struct Note {
    pub start_time: f64,
    pub duration: f64,
    pub pitch: f64,
    pub velocity: f64,
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

    pub fn midi(start_time: f64, duration: f64, pitch: f64, velocity: f64) -> Self {
        Self {
            start_time,
            duration,
            pitch: midi_hz(pitch),
            velocity: velocity / 127.0,
        }
    }
}
