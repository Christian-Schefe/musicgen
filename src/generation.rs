use std::rc::Rc;

use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};

use crate::score::*;

#[derive(Clone, Debug)]
pub struct SectionSettings {
    length: usize,
    key: Rc<Key>,
    bpm: f64,
    beats: u8,
    dynamic: Dynamic,
}

impl SectionSettings {
    pub fn new(length: usize, key: Rc<Key>, bpm: f64, beats: u8, dynamic: Dynamic) -> Self {
        Self {
            length,
            key,
            bpm,
            beats,
            dynamic,
        }
    }
}

pub fn generate_section<const N: usize>(
    rng: &mut ThreadRng,
    settings: SectionSettings,
) -> Section<N> {
    let mut bars = vec![
        Bar::new(
            settings.beats,
            settings.bpm,
            settings.key.clone(),
            settings.dynamic,
        );
        settings.length
    ];

    generate_melody(rng, 0, &mut bars);
    generate_chords(rng, 0, 1, &mut bars);

    Section::from_bars(bars)
}

pub fn generate_melody<const N: usize>(rng: &mut ThreadRng, voice: usize, bars: &mut Vec<Bar<N>>) {
    let mut shape: Vec<u8> = vec![0; bars.len()];
    shape[bars.len() - 1] = [0, 3, 4].into_iter().choose(rng).unwrap();
    for i in 1..bars.len() - 1 {
        if (i % 2) == 0 {
            shape[i] = rng.gen_range(0..7)
        }
    }
    for i in 1..bars.len() - 1 {
        if (i % 2) != 0 {
            let prev = shape[i - 1];
            let next = shape[i + 1];
            if prev == next {
                let lower = if prev < 2 { 0 } else { prev - 2 };
                let higher = if prev > 4 { 6 } else { prev + 2 };
                shape[i] = rng.gen_range(lower..=higher)
            } else if prev < next {
                shape[i] = rng.gen_range(prev..=next)
            } else if prev > next {
                shape[i] = rng.gen_range(next..=prev)
            }
        }
    }
    println!("{:?}", shape);

    for i in 0..bars.len() {
        let bar = bars.get_mut(i).unwrap();
        bar.add_note(voice, 0.0, Note::new(bar.beats as f64, shape[i], 5, None));
    }
}

pub fn generate_chords<const N: usize>(
    rng: &mut ThreadRng,
    melody: usize,
    voice: usize,
    bars: &mut Vec<Bar<N>>,
) {
    for i in 0..bars.len() {
        let bar = bars.get_mut(i).unwrap();
        let melody_notes: Vec<Note> = bar.notes[melody].iter().map(|x| x.1.clone()).collect();
        let p1 = &melody_notes[0];

        bar.add_note(
            voice,
            0.0,
            Note::new(bar.beats as f64, p1.pitch, 4, p1.accidental),
        );
        bar.add_note(
            voice,
            0.0,
            Note::new(bar.beats as f64, p1.pitch + 2, 4, p1.accidental),
        );
        bar.add_note(
            voice,
            0.0,
            Note::new(bar.beats as f64, p1.pitch + 4, 4, p1.accidental),
        );
    }
}
