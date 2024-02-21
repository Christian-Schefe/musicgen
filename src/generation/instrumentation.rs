use rand::{rngs::ThreadRng, Rng};

use crate::playback::tone::Tone;

use super::music::{Chord, Phrase, Piece};

pub enum Voicing {
    Melody((u8, u8)),
    Chords((u8, u8)),
}

pub fn generate_line(rng: &mut ThreadRng, piece: &Piece, voicing: Voicing) -> Vec<Tone> {
    let mut passed_beats = 0;

    piece
        .phrases
        .iter()
        .map(|x| generate_phrase_line(rng, piece, x, &voicing, &mut passed_beats))
        .flatten()
        .collect()
}

fn generate_phrase_line(
    rng: &mut ThreadRng,
    piece: &Piece,
    phrase: &Phrase,
    voicing: &Voicing,
    beats_passed: &mut usize,
) -> Vec<Tone> {
    phrase
        .harmony
        .iter()
        .map(|x| match voicing {
            Voicing::Melody(range) => {
                generate_melody_over_chord(rng, piece, x, beats_passed, range)
            }
            Voicing::Chords(range) => generate_chord_notes(rng, piece, x, beats_passed, range),
        })
        .flatten()
        .collect()
}

fn generate_melody_over_chord(
    rng: &mut ThreadRng,
    piece: &Piece,
    chord: &Chord,
    beats_passed: &mut usize,
    range: &(u8, u8),
) -> Vec<Tone> {
    let patterns = [
        vec![1.0],
        vec![0.5, 0.5],
        vec![0.75, 0.25],
        vec![0.25, 0.75],
        vec![0.25, 0.25, 0.5],
        vec![0.5, 0.25, 0.25],
        vec![0.25, 0.25, 0.25, 0.25],
    ];
    let pattern = &patterns[rng.gen_range(0..patterns.len())];
    pattern_melody(rng, pattern, piece, chord, beats_passed, range)
}

fn pattern_melody(
    rng: &mut ThreadRng,
    pattern: &[f64],
    piece: &Piece,
    chord: &Chord,
    beats_passed: &mut usize,
    range: &(u8, u8),
) -> Vec<Tone> {
    let mut tones = Vec::with_capacity(pattern.len());
    let mut last_pitch: Option<u8> = None;
    let mut start_beats = *beats_passed as f64;

    for &dur in pattern {
        let duration = dur * piece.beats_per_measure as f64;
        let options: Vec<u8> = (0..5).map(|_| chord.rand_from_range(rng, range)).collect();
        let pitch = if let Some(p) = last_pitch {
            let mut best = options[0];
            let mut best_dist = u8::MAX;
            for &opt in options.iter() {
                let dist = p.abs_diff(opt);
                if dist < best_dist && dist > 0 {
                    best = opt;
                    best_dist = dist;
                }
            }
            best
        } else {
            options[0]
        };

        tones.push(Tone::midi(
            start_beats / piece.bpm as f64 * 60.0,
            duration / piece.bpm as f64 * 60.0,
            pitch as f64,
            127.0,
        ));

        last_pitch = Some(pitch);
        start_beats += duration;
    }
    *beats_passed += piece.beats_per_measure as usize;
    tones
}

fn generate_chord_notes(
    rng: &mut ThreadRng,
    piece: &Piece,
    chord: &Chord,
    beats_passed: &mut usize,
    range: &(u8, u8),
) -> Vec<Tone> {
    let start_time = *beats_passed as f64 / piece.bpm as f64 * 60.0;
    *beats_passed += piece.beats_per_measure as usize;
    (0..3)
        .map(|x| {
            let pitch = chord.i_from_range(rng, x, range);
            let duration = piece.beats_per_measure as f64 * 60.0 / piece.bpm as f64;
            Tone::midi(start_time, duration, pitch as f64, 127.0)
        })
        .collect()
}
