use std::rc::Rc;

use crate::playback::instrument::Tone;

#[derive(Debug, Clone)]
pub struct Score {
    voices: usize,
    bars: Vec<Bar>,
}

impl Score {
    pub fn new(voices: usize) -> Self {
        Self {
            voices,
            bars: Vec::new(),
        }
    }
    pub fn add_bar(&mut self, mut bar: Bar) {
        bar.set_voice_count(self.voices);
        self.bars.push(bar)
    }
    pub fn add_bars(&mut self, mut bar: Bar, count: usize) {
        bar.set_voice_count(self.voices);
        self.bars.extend(vec![bar; count])
    }
    pub fn add_note(&mut self, voice: usize, bar: usize, beat: f64, note: Note) {
        self.bars[bar].add_note(voice, beat, note);
    }
    pub fn convert_to_playable(&self, voice: usize) -> Vec<Tone> {
        let mut time = 0.0;
        self.bars.iter().flat_map(|x| x.convert_to_playable(voice, &mut time)).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Bar {
    beats: u8,
    notes: Vec<Vec<(f64, Note)>>,
    bpm: f64,
    key: Rc<Key>,
    dynamic: Dynamic,
}

impl Bar {
    pub fn new(beats: u8, bpm: f64, key: Rc<Key>, dynamic: Dynamic) -> Self {
        Self {
            beats,
            notes: Vec::new(),
            bpm,
            key,
            dynamic,
        }
    }
    fn set_voice_count(&mut self, voices: usize) {
        self.notes.resize_with(voices, Vec::new)
    }
    fn add_note(&mut self, voice: usize, beat: f64, note: Note) {
        self.notes[voice].push((beat, note))
    }
    fn convert_to_playable(&self, voice: usize, time: &mut f64) -> Vec<Tone> {
        let tones = self.notes[voice]
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
        Tone::midi(
            time + time_offset,
            self.length,
            bar.key.midi(self),
            bar.dynamic.velocity(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Key {
    tonic: u8,
    scale: [u8; 7],
}

impl Key {
    pub fn new(tonic: u8, mode: bool) -> Self {
        Self {
            tonic,
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
