use std::time::Duration;

use fundsp::hacker::*;

use super::{tone::Tone, synth::Synth};

pub struct Instrument {
    sequencer: Sequencer64,
    timer: Shared<f64>,
    synth: Box<dyn Synth>,
}

impl Instrument {
    pub fn new(synth: Box<dyn Synth>) -> Self {
        Self {
            sequencer: Sequencer64::new(true, 2),
            timer: shared(0.0),
            synth,
        }
    }

    pub fn sequence_notes(mut self, notes: &[Tone]) -> Sound {
        let mut max_end_time = 0.0;

        for note in notes.iter() {
            if note.pitch <= 0.0 {
                max_end_time = note.start_time + note.duration;
                continue;
            }
            let unit = self.build_sound(note.clone());
            let release_time = self.synth.release_time();
            let end_time = note.start_time + note.duration + release_time;

            self.sequencer
                .push(note.start_time, end_time, Fade::Smooth, 0.01, 0.01, unit);

            max_end_time = max_end_time.max(end_time);
        }
        Sound(
            Net64::wrap(Box::new(self.sequencer)) | timer(&self.timer),
            Duration::from_secs_f64(max_end_time),
        )
    }

    fn build_sound(&mut self, note: Tone) -> Box<dyn AudioUnit64> {
        Box::new(
            (constant(note.pitch)
                | var_fn(&self.timer, move |time| {
                    asdr_control(time, note.start_time + note.duration)
                })
                | constant(note.velocity))
                >> self.synth.instantiate(),
        )
    }
}

fn asdr_control(time: f64, end_time: f64) -> f64 {
    if time < end_time {
        1.0
    } else {
        -1.0
    }
}

pub struct Sound(pub Net64, pub Duration);

impl Sound {
    fn mix(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1.max(other.1))
    }
}

pub fn mix_instruments(instruments: Vec<(Instrument, Vec<Tone>)>) -> Sound {
    instruments
        .into_iter()
        .map(|(i, n)| i.sequence_notes(&n))
        .reduce(Sound::mix)
        .unwrap()
}
