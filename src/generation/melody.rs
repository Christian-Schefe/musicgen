use rand::{rngs::ThreadRng, Rng};

use crate::{
    generation::{music::Note, rhythm::gen_rythm},
    playback::tone::Tone,
};

use super::music::Piece;

pub fn gen_melody(rng: &mut ThreadRng, piece: &Piece, range: (u8, u8)) -> Vec<Tone> {
    let mut notes = Vec::new();

    let mut cur_time = 0.0;
    let mut prev_pitch = 0;

    for phrase in piece.phrases.iter() {
        for chord in phrase.harmony.iter() {
            println!("{:?}", chord);

            let pattern = gen_rythm(rng);

            let mut beats = 0.0;
            for (i, note) in pattern.iter().enumerate() {
                let offset = if rng.gen_bool(0.5) { -1 } else { 1 };
                let new_index = 0.max((prev_pitch as i8 + offset) as u8);
                prev_pitch = new_index;

                let pitch = piece.key.from_index(new_index).midi_range(range.0, range.1);

                let start_time = cur_time + beats * 60.0 / piece.bpm as f64;
                let duration = note * 60.0 as f64 / piece.bpm as f64;

                notes.push(Tone::midi(start_time, duration, pitch as f64, 127.0));
                beats += note;
                println!("{:?}: {} {}", Note::from_midi(pitch), note, pitch);
            }

            cur_time += piece.beats_per_measure as f64 * 60.0 / piece.bpm as f64;
        }
    }

    notes
}
