use std::rc::Rc;

use crate::playback::tone::Tone;

#[derive(Debug, Clone)]
pub struct Score {
    voices: Vec<Voice>,
}

impl Score {
    pub fn new() -> Self {
        Self { voices: Vec::new() }
    }
    pub fn add_voice(&mut self, voice: Voice) {
        self.voices.push(voice)
    }
    pub fn add_bar(&mut self, bar: Bar) {
        self.voices
            .iter_mut()
            .for_each(|x| x.bars.push(bar.clone()))
    }
    pub fn add_bars(&mut self, bar: Bar, count: usize) {
        self.voices
            .iter_mut()
            .for_each(|x| x.bars.extend(vec![bar.clone(); count]))
    }
    pub fn add_note(&mut self, voice: usize, bar: usize, beat: f64, note: Note) {
        self.voices[voice].bars[bar].add_note(beat, note);
    }
    pub fn convert_to_playable(&self) -> Vec<Vec<Tone>> {
        self.voices.iter().map(Voice::convert_to_playable).collect()
    }
    pub fn convert_voice_to_playable(&self, voice: usize) -> Vec<Tone> {
        self.voices[voice].convert_to_playable()
    }
}

#[derive(Debug, Clone)]
pub struct Voice {
    bars: Vec<Bar>,
}

impl Voice {
    pub fn new() -> Self {
        Self { bars: Vec::new() }
    }
    fn convert_to_playable(&self) -> Vec<Tone> {
        let mut time = 0.0;
        self.bars
            .iter()
            .flat_map(|x| x.convert_to_playable(&mut time))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Bar {
    beats: u8,
    notes: Vec<(f64, Note)>,
    bpm: f64,
    key: Rc<Key>,
    dynamic: Dynamic,
}

impl Bar {
    pub fn new(beats: u8, bpm: f64, key: Rc<Key>, dynamic: Dynamic) -> Self {
        Self {
            beats,
            notes: Vec::new(),
            bpm: 120.0,
            key,
            dynamic,
        }
    }
    fn add_note(&mut self, beat: f64, note: Note) {
        self.notes.push((beat, note))
    }
    fn convert_to_playable(&self, time: &mut f64) -> Vec<Tone> {
        let tones = self
            .notes
            .iter()
            .map(|(offset, x)| x.convert_to_playable(*time, self, *offset))
            .collect();
        *time += self.beats as f64 * 60.0 / self.bpm;
        tones
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    length: f64,
    pitch: u8,
    octave: u8,
}

impl Note {
    pub fn new(length: f64, pitch: u8, octave: u8) -> Self {
        Self {
            length,
            pitch,
            octave,
        }
    }
    fn convert_to_playable(&self, time: f64, bar: &Bar, offset: f64) -> Tone {
        let time_offset = offset * 60.0 / bar.bpm;
        Tone {
            start_time: time + time_offset,
            duration: self.length,
            pitch: bar.key.midi(self),
            velocity: bar.dynamic.velocity(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Key {
    tonic: u8,
    mode: bool,
    scale: [u8; 7],
}

impl Key {
    fn new(tonic: u8, mode: bool) -> Self {
        Self {
            tonic,
            mode,
            scale: Self::gen_scale(mode),
        }
    }
    fn gen_scale(mode: bool) -> [u8; 7] {
        if mode {
            [0, 2, 4, 5, 7, 9, 11]
        } else {
            [0, 2, 3, 5, 7, 8, 10]
        }
    }
    fn midi(&self, note: &Note) -> f64 {
        (self.tonic + self.scale[note.pitch as usize] + note.octave * 12) as f64
    }
}

#[derive(Debug, Clone)]
pub enum Dynamic {
    Piano,
    MezzoPiano,
    MezzoForte,
    Forte,
}

impl Dynamic {
    fn velocity(&self) -> f64 {
        match self {
            Self::Piano => 52.0,
            Self::MezzoPiano => 77.0,
            Self::MezzoForte => 102.0,
            Self::Forte => 127.0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::playback::{
        instrument::{mix_instruments, Instrument},
        playback,
        synth::{keys_synth, strings_synth},
    };

    use super::*;

    #[test]
    fn test_score() {
        let mut score = Score::new();
        let key = Rc::new(Key::new(0, true));
        let bpm = 100.0;

        score.add_voice(Voice::new());
        score.add_voice(Voice::new());

        score.add_bar(Bar::new(4, bpm, key.clone(), Dynamic::MezzoForte));
        score.add_bar(Bar::new(4, bpm, key.clone(), Dynamic::MezzoForte));
        score.add_note(0, 0, 0.0, Note::new(1.0, 0, 4));
        score.add_note(0, 0, 1.0, Note::new(1.0, 1, 4));
        score.add_note(0, 0, 2.0, Note::new(1.0, 2, 4));
        score.add_note(0, 0, 3.0, Note::new(1.0, 3, 4));

        let playable = score.convert_to_playable();

        let keys = Instrument::new(Box::new(keys_synth(0.5)));
        let strings = Instrument::new(Box::new(strings_synth(0.5)));
        let instruments = [strings, keys];
        let zip = instruments.into_iter().zip(playable).collect();
        let sound = mix_instruments(zip);
        playback(sound).unwrap();
    }
}
