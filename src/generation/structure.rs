use rand::{Rng, rngs::ThreadRng};

use super::music::{Key, Note, Phrase, Piece, Chord};

pub fn generate_structure(rng: &mut ThreadRng, phrases: usize, bpm_range: (u16, u16)) -> Piece {
    let bpm = (rng.gen_range(bpm_range.0..=bpm_range.1) + rng.gen_range(bpm_range.0..=bpm_range.1)) / 2;
    let beats_per_measure = 4;
    let key = Key(rng.gen_bool(0.5), Note::from_midi(rng.gen_range(0..12)));

    let phrases = (0..phrases).map(|_| generate_phrase(key, rng)).collect();

    Piece { bpm, beats_per_measure, key, phrases }
}

fn generate_phrase(key: Key, rng: &mut ThreadRng) -> Phrase {
    let length = 8;
    let harmony = (0..length).map(|_| generate_chord(key, rng)).collect();

    Phrase { harmony }
}

fn generate_chord(key: Key, rng: &mut ThreadRng) -> Chord {
    key.build_chord(rng.gen_range(0..7))
}
