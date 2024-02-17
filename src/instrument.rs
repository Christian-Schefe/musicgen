use std::time::Duration;

use fundsp::hacker::*;

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
    pub fn new(
        sound: Box<dyn Fn() -> Net64>,
        outputs: usize,
        data: InstrumentData,
    ) -> Self {
        Self {
            sequencer: Sequencer64::new(true, outputs),
            timer: shared(0.0),
            data,
            sound,
        }
    }

    pub fn sequence_notes(mut self, notes: &[(f64, f64)]) -> (Net64, Duration) {
        let mut start_time = 0.0;
        for &(pitch, duration) in notes.iter() {
            if pitch <= 0.0 {
                start_time += duration;
                continue;
            }
            let unit = self.build_sound(pitch, start_time, duration);

            self.sequencer
                .push_duration(start_time, duration + self.data.asdr.3, Fade::Smooth, 0.0, 0.0, unit);
            start_time += duration;
        }
        (
            Net64::wrap(Box::new(self.sequencer)) | timer(&self.timer),
            Duration::from_secs_f64(start_time + self.data.asdr.3),
        )
    }

    fn build_sound(&mut self, pitch: f64, start_time: f64, duration: f64) -> Box<dyn AudioUnit64> {
        Box::new(
            (constant(midi_hz(pitch))
                | var_fn(&self.timer, move |time| {
                    asdr_control(time, start_time + duration)
                }))
                >> self.get_sound(),
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
