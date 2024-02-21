use std::time::Duration;

use fundsp::hacker::*;

use crate::synth::Synth;

use super::tone::Tone;

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

    pub fn sequence_notes(mut self, notes: &[Tone]) -> (Net64, Duration) {
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
                .push(note.start_time, end_time, Fade::Smooth, 0.0, 0.0, unit);

            max_end_time = max_end_time.max(end_time);
        }
        (
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
    pub fn release_time(&self) -> f64 {
        self.synth.release_time()
    }
}

fn asdr_control(time: f64, end_time: f64) -> f64 {
    if time < end_time {
        1.0
    } else {
        -1.0
    }
}
