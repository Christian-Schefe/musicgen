use std::time::Duration;

use fundsp::hacker::*;

use super::synth::Synth;

pub struct SingleVoiceInstrument {
    timer: Shared<f64>,
    synth: Box<dyn Synth>,
    notes: Vec<Tone>,
    duration: Duration,
}

impl SingleVoiceInstrument {
    pub fn new(synth: Box<dyn Synth>, notes: Vec<Tone>) -> Self {
        let release_time = synth.release_time();
        let duration = Self::find_duration(&notes, release_time);
        Self {
            timer: shared(0.0),
            synth,
            notes,
            duration,
        }
    }

    fn find_duration(notes: &[Tone], release_time: f64) -> Duration {
        let dur = notes
            .iter()
            .map(|note| note.start_time + note.duration + release_time)
            .reduce(f64::max)
            .unwrap_or(0.0);

        Duration::from_secs_f64(dur)
    }

    fn build_sound(&self) -> Box<dyn AudioUnit64> {
        let mut notes: Vec<Tone> = self.notes.clone();
        notes.sort_by(|a, b| b.start_time.total_cmp(&a.start_time));
        let pitch_source = move |x| Self::get_pitch_by_time(&notes, x);
        Box::new(
            (var_fn(&self.timer, pitch_source) | constant(1.0) | constant(1.0))
                >> self.synth.instantiate(),
        )
    }

    fn get_pitch_by_time(notes: &[Tone], time: f64) -> f64 {
        let i = notes
            .iter()
            .find(|x| x.start_time <= time && x.start_time + x.duration > time);
        if let Some(j) = i {
            j.pitch
        } else {
            0.0
        }
    }
}

impl Synth for SingleVoiceInstrument {
    fn instantiate(&self) -> Net64 {
        let sound = self.build_sound();

        Net64::wrap(sound) | timer(&self.timer)
    }
    fn release_time(&self) -> f64 {
        self.synth.release_time()
    }
}

impl SoundMaker for SingleVoiceInstrument {
    fn build(&self) -> Sound {
        Sound(self.instantiate(), self.duration)
    }
}

pub struct Instrument {
    timer: Shared<f64>,
    synth: Box<dyn Synth>,
    notes: Vec<Tone>,
    duration: Duration,
}

impl Instrument {
    pub fn new(synth: Box<dyn Synth>, notes: Vec<Tone>) -> Self {
        let release_time = synth.release_time();
        let duration = Self::find_duration(&notes, release_time);
        Self {
            timer: shared(0.0),
            synth,
            notes,
            duration,
        }
    }

    fn find_duration(notes: &[Tone], release_time: f64) -> Duration {
        let dur = notes
            .iter()
            .map(|note| note.start_time + note.duration + release_time)
            .reduce(f64::max)
            .unwrap_or(0.0);

        Duration::from_secs_f64(dur)
    }

    fn build_sound(&self, note: Tone) -> Box<dyn AudioUnit64> {
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

impl Synth for Instrument {
    fn instantiate(&self) -> Net64 {
        let mut sequencer = Sequencer64::new(true, 2);
        let release_time = self.synth.release_time();

        self.notes.iter().for_each(|note| {
            let unit = self.build_sound(note.clone());
            let end_time = note.start_time + note.duration + release_time;
            sequencer.push(note.start_time, end_time, Fade::Smooth, 0.01, 0.01, unit);
        });

        Net64::wrap(Box::new(sequencer)) | timer(&self.timer)
    }
    fn release_time(&self) -> f64 {
        self.synth.release_time()
    }
}

impl SoundMaker for Instrument {
    fn build(&self) -> Sound {
        Sound(self.instantiate(), self.duration)
    }
}

#[inline]
fn asdr_control(time: f64, end_time: f64) -> f64 {
    if time < end_time {
        1.0
    } else {
        -1.0
    }
}

pub trait SoundMaker {
    fn build(&self) -> Sound;
}

pub struct SoundMix {
    sounds: Vec<Box<dyn SoundMaker>>,
}

impl SoundMix {
    pub fn mix(sounds: Vec<Box<dyn SoundMaker>>) -> SoundMix {
        SoundMix { sounds }
    }
}

impl SoundMaker for SoundMix {
    fn build(&self) -> Sound {
        self.sounds
            .iter()
            .map(|x| x.build())
            .reduce(Sound::mix)
            .unwrap()
    }
}

pub struct Sound(pub Net64, pub Duration);

impl Sound {
    fn mix(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1.max(other.1))
    }
}

#[derive(Clone, Debug)]
pub struct Tone {
    pub start_time: f64,
    pub duration: f64,
    pub pitch: f64,
    pub velocity: f64,
}

impl Tone {
    pub fn new(start_time: f64, duration: f64, pitch: f64, velocity: f64) -> Self {
        Self {
            start_time,
            duration,
            pitch,
            velocity,
        }
    }

    pub fn midi(start_time: f64, duration: f64, pitch: f64, velocity: f64) -> Self {
        Self::new(start_time, duration, midi_hz(pitch), velocity / 127.0)
    }
}
