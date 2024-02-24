use rand::{rngs::ThreadRng, Rng};

use crate::{generation::music::Note, playback::tone::Tone};

use super::music::Piece;

#[derive(Debug, Clone)]
struct Pattern(Vec<(f64, bool)>);

impl Pattern {
    fn modify(&mut self, rng: &mut ThreadRng, p: f64) {
        self.0.iter_mut().for_each(|x| x.1 = x.1 & rng.gen_bool(p));
    }
}

pub fn gen_melody(rng: &mut ThreadRng, piece: &Piece, range: (u8, u8)) -> Vec<Tone> {
    let mut notes = Vec::new();

    let mut cur_time = 0.0;

    for phrase in piece.phrases.iter() {
        for chord in phrase.harmony.iter() {
            println!("{:?}", chord);

            let pattern = gen_pattern(rng, piece);
            let mut pattern_tones = vec![None; pattern.0.len()];

            for (i, note) in pattern.0.iter().enumerate() {
                if !note.1 {
                    let tone = chord.rand_from_range(rng, &range);
                    pattern_tones[i] = Some(tone);
                }
            }

            for (i, note) in pattern.0.iter().enumerate() {
                if note.1 {
                    let prev = if i > 0 { pattern_tones[i - 1] } else { None };
                    let next = if i + 1 < pattern_tones.len() {
                        pattern_tones[i + 1]
                    } else {
                        None
                    };
                    let prev_i = prev.and_then(|x| piece.key.to_index(x));
                    let next_i = next.and_then(|x| piece.key.to_index(x));

                    let tone = match (prev_i, next_i) {
                        (None, None) => chord.rand_from_range(rng, &range),
                        (None, Some(n)) => piece
                            .key
                            .from_index(if rng.gen_bool(0.5) { n + 1 } else { n - 1 })
                            .midi_range(range.0, range.1),
                        (Some(n), None) => piece
                            .key
                            .from_index(if rng.gen_bool(0.5) { n + 1 } else { n - 1 })
                            .midi_range(range.0, range.1),
                        (Some(n), Some(n2)) if n == n2 => piece
                            .key
                            .from_index(if rng.gen_bool(0.5) { n + 1 } else { n - 1 })
                            .midi_range(range.0, range.1),
                        (Some(n), Some(n2)) if n.abs_diff(n2) <= 1 => piece
                            .key
                            .from_index(if rng.gen_bool(0.5) { n } else { n2 })
                            .midi_range(range.0, range.1),
                        (Some(n), Some(n2)) => piece
                            .key
                            .from_index(rng.gen_range(n.min(n2) + 1..n.max(n2)))
                            .midi_range(range.0, range.1),
                    };

                    pattern_tones[i] = Some(tone);
                }
            }

            let mut beats = 0.0;
            for (i, note) in pattern.0.iter().enumerate() {
                let pitch = pattern_tones[i].unwrap();

                let start_time =
                    cur_time + beats * 60.0 * piece.beats_per_measure as f64 / piece.bpm as f64;
                let duration = note.0 * 60.0 * piece.beats_per_measure as f64 / piece.bpm as f64;

                notes.push(Tone::midi(start_time, duration, pitch as f64, 127.0));
                beats += note.0;
                println!("{:?}: {} {}", Note::from_midi(pitch), note.0, pitch);
            }

            cur_time += piece.beats_per_measure as f64 * 60.0 / piece.bpm as f64;
        }
    }

    notes
}

fn gen_pattern(rng: &mut ThreadRng, piece: &Piece) -> Pattern {
    let patterns = [
        Pattern(vec![(1.0, false)]),
        Pattern(vec![(0.5, false), (0.5, false)]),
        Pattern(vec![(0.25, true), (0.75, false)]),
        Pattern(vec![(0.75, false), (0.25, true)]),
        Pattern(vec![(0.5, false), (0.25, true), (0.25, false)]),
        Pattern(vec![(0.25, false), (0.25, true), (0.5, false)]),
        Pattern(vec![(0.25, false), (0.25, true), (0.25, true), (0.25, false)]),
    ];
    let pattern_i = rng.gen_range(0..patterns.len());
    let mut pattern = patterns[pattern_i].clone();
    pattern.modify(rng, 0.7);
    pattern
}
