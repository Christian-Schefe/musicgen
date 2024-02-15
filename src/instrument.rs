use std::time::Duration;
use std::time::{self, Instant};

use fundsp::hacker::*;

pub struct Instrument {
    pitch: Shared<f64>,
    volume: Shared<f64>,
    final_volume: Shared<f64>,
    generator: Net64,
}

impl Instrument {
    pub fn set_pitch(&mut self, frequency: f64) {
        self.pitch.set(frequency);
    }
    pub fn change_pitch(&mut self, semitones: f64) {
        self.set_pitch(self.pitch.value() * pow(2.0, semitones / 12.0));
    }
    pub fn set_volume(&mut self, volume: f64) {
        self.volume.set(volume);
    }
    pub fn set_final_volume(&mut self, volume: f64) {
        self.final_volume.set(volume);
    }
}

pub struct Song {
    instruments: Vec<Instrument>,
    notes: Vec<(Vec<(f64, f64)>, u64)>,
    timer: Shared<f64>,
}

impl Song {
    pub fn new(instruments: Vec<Instrument>, notes: Vec<(Vec<(f64, f64)>, u64)>) -> Self {
        Song {
            instruments,
            notes,
            timer: shared(0.0),
        }
    }
    pub fn build_network(&self) -> Net64 {
        let mut net = self
            .instruments
            .iter()
            .fold(Net64::new(0, 2), |x, a| a.generator.clone() + x);
        net.push(Box::new(timer(&self.timer)));
        net
    }
    pub fn play(&mut self) {
        for (notes, duration) in self.notes.iter() {
            for (i, (pitch, volume)) in notes.iter().enumerate() {
                self.instruments[i].set_pitch(midi_hz(*pitch));
                self.instruments[i].set_volume(*volume);
                self.instruments[i].set_final_volume(1.0);
            }
            let time_started = self.timer.value();
            let mut cur_time = self.timer.value();
            let mut passed_millis = 0;

            while passed_millis < (*duration).into() {
                passed_millis = Duration::from_secs_f64(cur_time - time_started).as_millis();
                cur_time = self.timer.value();
                let alpha = 1.0 - passed_millis as f64 / *duration as f64;
                let alpha = alpha.clamp(0.0, 1.0);
                println!("{} {}", self.timer.value(), alpha);
                self.instruments
                    .iter_mut()
                    .for_each(|x| x.set_final_volume(alpha))
            }
        }
    }
}

pub fn create_instrument(initial_pitch: f64, initial_volume: f64) -> Instrument {
    let pitch = shared(midi_hz(initial_pitch));
    let volume = shared(initial_volume);
    let final_volume = shared(1.0);

    let c = var(&pitch) >> triangle();
    let c = c * var(&volume) * var(&final_volume);
    let c = c >> pan(0.0);
    let c = c >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));

    let c = c >> (declick() | declick()) >> (dcblock() | dcblock()) >> limiter_stereo((1.0, 5.0));

    let mut generator = Net64::new(0, 2);
    let node_id = generator.push(Box::new(c));
    generator.pipe_input(node_id);
    generator.pipe_output(node_id);

    Instrument {
        pitch,
        volume,
        final_volume,
        generator,
    }
}
