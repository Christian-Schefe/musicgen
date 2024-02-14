pub struct Note {
    pitch: u8,
    duration: u8,
    velocity: u8,
}

impl Note {
    pub fn new(pitch: u8, duration: u8, velocity: u8) -> Self {
        Note {
            pitch,
            duration,
            velocity
        }
    }
}