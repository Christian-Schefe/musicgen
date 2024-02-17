use std::time::Duration;

use fundsp::prelude::Net64;

use crate::{instrument::Instrument, note::Note};

pub struct Song {
    instruments: Vec<Net64>,
    duration: Duration,
}

impl Song {
    pub fn new() -> Self {
        Song {
            instruments: Vec::new(),
            duration: Duration::ZERO,
        }
    }

    pub fn from_instruments(instruments: Vec<(Instrument, &[Note])>) -> Self {
        let mut song = Song::new();
        instruments
            .into_iter()
            .for_each(|(i, n)| song.add_instrument(i, n));
        song
    }

    pub fn add_instrument(&mut self, instrument: Instrument, notes: &[Note]) {
        let (net, duration) = instrument.sequence_notes(notes);
        self.duration = self.duration.max(duration);
        self.instruments.push(net);
    }

    pub fn mix(self) -> (Net64, Duration) {
        (self.instruments
            .into_iter()
            .fold(Net64::new(0, 2), |x, a| x + a), self.duration + Duration::from_secs_f64(0.2))
    }
}
