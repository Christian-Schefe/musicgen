use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Debug)]
pub struct Piece {
    pub bpm: u16,
    pub beats_per_measure: u16,
    pub key: Key,
    pub phrases: Vec<Phrase>,
}

#[derive(Clone, Debug)]
pub struct Phrase {
    pub harmony: Vec<Chord>,
}

#[derive(Clone, Debug)]
pub struct Chord(pub Vec<Note>);

impl Chord {
    pub fn rand_from_range(&self, rng: &mut ThreadRng, range: &(u8, u8)) -> u8 {
        let i = rng.gen_range(0..self.0.len());
        self.i_from_range(rng, i, range)
    }
    pub fn i_from_range(&self, rng: &mut ThreadRng, i: usize, range: &(u8, u8)) -> u8 {
        let v = self.0[i];
        let in_range = v.midi_range(range.0, range.1);
        let mut possibs = vec![in_range];

        let mut cur = in_range + 12;
        while cur <= range.1 {
            possibs.push(cur);
            cur += 12;
        }
        cur = in_range - 12;
        while cur >= range.0 {
            possibs.push(cur);
            cur -= 12;
        }
        possibs[rng.gen_range(0..possibs.len())]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Key(pub bool, pub Note);

impl Key {
    pub fn scale(&self) -> Vec<Note> {
        self.offsets()
            .into_iter()
            .map(|x| self.1.shift_by(x))
            .collect()
    }
    pub fn offsets(&self) -> [u8; 7] {
        if self.0 {
            [0, 2, 4, 5, 7, 9, 11]
        } else {
            [0, 2, 3, 5, 7, 8, 10]
        }
    }
    pub fn build_chord(&self, tonic: u8) -> Chord {
        let index = tonic as usize;
        let scale = self.scale();
        Chord(
            [
                scale[index % 7],
                scale[(index + 2) % 7],
                scale[(index + 4) % 7],
            ]
            .to_vec(),
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Note {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl Note {
    pub fn shift_by(&self, semitones: u8) -> Self {
        Note::from_midi(self.midi() + semitones)
    }
    pub fn midi(&self) -> u8 {
        60u8 + *self as u8
    }
    pub fn from_midi(midi: u8) -> Self {
        match midi % 12 {
            0 => Note::C,
            1 => Note::Db,
            2 => Note::D,
            3 => Note::Eb,
            4 => Note::E,
            5 => Note::F,
            6 => Note::Gb,
            7 => Note::G,
            8 => Note::Ab,
            9 => Note::A,
            10 => Note::Bb,
            _ => Note::B,
        }
    }
    pub fn midi_range(&self, min: u8, max: u8) -> u8 {
        let v = self.midi();
        if v >= min && v <= max {
            v
        } else if v < min {
            v + 12 * (1 + (min - v) / 12)
        } else {
            v - 12 * (1 + (v - max) / 12)
        }
    }
}
