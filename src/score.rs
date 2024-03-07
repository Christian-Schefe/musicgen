use std::rc::Rc;

use crate::playback::instrument::Tone;

#[derive(Debug, Clone)]
pub struct Score<const N: usize> {
    sections: Vec<Section<N>>,
}

impl<const N: usize> Score<N> {
    pub fn from_sections(sections: Vec<Section<N>>) -> Self {
        Self { sections }
    }
    pub fn convert_to_playable(&self) -> [Vec<Tone>; N] {
        std::array::from_fn(|i| {
            let mut time = 0.0;
            self.sections
                .iter()
                .flat_map(|x| x.convert_to_playable(i, &mut time))
                .collect()
        })
    }
}

#[derive(Debug, Clone)]
pub struct Section<const N: usize> {
    bars: Vec<Bar<N>>,
}

impl<const N: usize> Section<N> {
    pub fn from_bars(bars: Vec<Bar<N>>) -> Self {
        Self { bars }
    }
    pub fn convert_to_playable(&self, voice: usize, time: &mut f64) -> Vec<Tone> {
        self.bars
            .iter()
            .flat_map(|x| x.convert_to_playable(voice, time))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Bar<const N: usize> {
    pub beats: u8,
    pub notes: [Vec<(f64, Note)>; N],
    pub bpm: f64,
    pub key: Rc<Key>,
    pub dynamic: Dynamic,
}

impl<const N: usize> Bar<N> {
    pub fn new(beats: u8, bpm: f64, key: Rc<Key>, dynamic: Dynamic) -> Self {
        Self {
            beats,
            notes: vec![Vec::new(); N].try_into().unwrap(),
            bpm,
            key,
            dynamic,
        }
    }
    pub fn add_note(&mut self, voice: usize, beat: f64, note: Note) {
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
    pub length: f64,
    pub pitch: u8,
    pub octave: u8,
    pub accidental: Option<bool>,
}

impl Note {
    pub fn new(length: f64, pitch: u8, octave: u8, accidental: Option<bool>) -> Self {
        Self {
            length,
            pitch: pitch % 7,
            octave,
            accidental,
        }
    }
    fn convert_to_playable<const N: usize>(&self, time: f64, bar: &Bar<N>, offset: f64) -> Tone {
        let secs_per_beat = 60.0 / bar.bpm;
        let time_offset = offset * secs_per_beat;
        Tone::midi(
            time + time_offset,
            self.length * secs_per_beat,
            bar.key.midi(self),
            bar.dynamic.velocity(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Key {
    tonic: i8,
    scale: [i8; 7],
}

impl Key {
    pub fn new(tonic: i8, mode: bool) -> Self {
        Self {
            tonic,
            scale: Self::gen_scale(mode),
        }
    }
    fn gen_scale(mode: bool) -> [i8; 7] {
        if mode {
            [0, 2, 4, 5, 7, 9, 11]
        } else {
            [0, 2, 3, 5, 7, 8, 11]
        }
    }
    fn midi(&self, note: &Note) -> f64 {
        let octave = (note.octave * 12) as i8;
        let offset = octave + note.accidental.map_or(0, |b| if b { 1 } else { -1 });
        (self.tonic + self.scale[note.pitch as usize] + offset) as f64
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
