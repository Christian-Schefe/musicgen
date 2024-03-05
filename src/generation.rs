use std::rc::Rc;

use rand::{rngs::ThreadRng, Rng};

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

    Section::from_bars(bars)
}

pub fn generate_melody<const N: usize>(rng: &mut ThreadRng, voice: usize, bars: &mut Vec<Bar<N>>) {
    let mut shape = vec![0; N];
    for i in 1..N {
        if (i % 2) == 0 {
            shape.push(rng.gen_range(0..7))
        }
    }
    for i in 0..N - 1 {
        if (i % 2) != 0 {
            let prev = shape[i - 1];
            let next = shape[i + 1];
            shape.push(rng.gen_range(prev..=next))
        }
    }

    for i in 0..N {
        let bar = bars.get_mut(i).unwrap();
        bar.add_note(voice, 0.0, Note::new(bar.beats as f64, shape[i], 5, None));
    }
}
