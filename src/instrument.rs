use std::time::Duration;

use fundsp::hacker::*;

use crate::note::Note;

pub struct InstrumentData {
    pub asdr: (f64, f64, f64, f64),
    pub pan: f64,
    pub volume: f64,
}

pub struct Instrument {
    sequencer: Sequencer64,
    timer: Shared<f64>,
    data: InstrumentData,
    sound: Box<dyn Fn() -> Net64>,
}

impl Instrument {
    pub fn new(sound: Box<dyn Fn() -> Net64>, outputs: usize, data: InstrumentData) -> Self {
        Self {
            sequencer: Sequencer64::new(true, outputs),
            timer: shared(0.0),
            data,
            sound,
        }
    }

    pub fn sequence_notes(mut self, notes: &[Note]) -> (Net64, Duration) {
        let mut end_time = 0.0;

        for note in notes.iter() {
            if note.pitch <= 0.0 {
                end_time = note.start_time + note.duration;
                continue;
            }
            let unit = self.build_sound(note.clone());

            self.sequencer.push_duration(
                note.start_time,
                note.duration + self.data.asdr.3,
                Fade::Smooth,
                0.0,
                0.0,
                unit,
            );

            end_time = max(end_time, note.start_time + note.duration + self.data.asdr.3);
        }
        (
            Net64::wrap(Box::new(self.sequencer)) | timer(&self.timer),
            Duration::from_secs_f64(end_time),
        )
    }

    fn build_sound(&mut self, note: Note) -> Box<dyn AudioUnit64> {
        Box::new(
            ((constant(note.pitch)
                | var_fn(&self.timer, move |time| {
                    asdr_control(time, note.start_time + note.duration)
                }))
                >> self.get_sound()) * (constant(note.velocity) | constant(note.velocity)),
        )
    }

    fn get_sound(&mut self) -> Net64 {
        (self.sound)()
    }
}

impl Clone for InstrumentData {
    fn clone(&self) -> Self {
        Self {
            asdr: self.asdr.clone(),
            pan: self.pan,
            volume: self.volume,
        }
    }
}

impl InstrumentData {
    pub fn new(asdr: (f64, f64, f64, f64), pan: f64, volume: f64) -> Self {
        Self { asdr, pan, volume }
    }
}

fn asdr_control(time: f64, end_time: f64) -> f64 {
    if time < end_time {
        1.0
    } else {
        -1.0
    }
}
