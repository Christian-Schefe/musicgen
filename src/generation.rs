use std::rc::Rc;

use rand::{rngs::StdRng, Rng};

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

pub fn generate_section<const N: usize>(rng: &mut StdRng, settings: SectionSettings) -> Section<N> {
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
    generate_beat(rng, 2, 3, &mut bars);

    Section::from_bars(bars)
}

pub fn generate_beat<const N: usize>(
    rng: &mut StdRng,
    bassdrum_voice: usize,
    snare_voice: usize,
    bars: &mut Vec<Bar<N>>,
) {
    let pattern = rng.gen_range(0..=0);

    for i in 0..bars.len() {
        let bar = bars.get_mut(i).unwrap();

        if pattern == 0 {
            bar.add_note(bassdrum_voice, 0.0, Note::new(1.0, 0, 3, None));
            bar.add_note(snare_voice, 1.0, Note::new(1.0, 0, 3, None));
            bar.add_note(bassdrum_voice, 1.5, Note::new(1.0, 0, 3, None));
            bar.add_note(bassdrum_voice, 2.0, Note::new(1.0, 0, 3, None));
            bar.add_note(snare_voice, 3.0, Note::new(1.0, 0, 3, None));
        } else if pattern == 1 {
            bar.add_note(bassdrum_voice, 0.0, Note::new(1.0, 0, 3, None));
            bar.add_note(snare_voice, 0.70, Note::new(1.0, 0, 3, None));
            bar.add_note(bassdrum_voice, 1.0, Note::new(1.0, 0, 3, None));
            bar.add_note(snare_voice, 1.5, Note::new(1.0, 0, 3, None));
            bar.add_note(bassdrum_voice, 2.0, Note::new(1.0, 0, 3, None));
            bar.add_note(snare_voice, 2.70, Note::new(1.0, 0, 3, None));
            bar.add_note(bassdrum_voice, 3.0, Note::new(1.0, 0, 3, None));
            bar.add_note(snare_voice, 3.5, Note::new(1.0, 0, 3, None));
        } else if pattern == 2 {
            bar.add_note(bassdrum_voice, 0.0, Note::new(1.0, 0, 3, None));
            bar.add_note(snare_voice, 1.0, Note::new(1.0, 0, 3, None));
            bar.add_note(bassdrum_voice, 2.0, Note::new(1.0, 0, 3, None));
            bar.add_note(snare_voice, 3.0, Note::new(1.0, 0, 3, None));
        } else {
            bar.add_note(bassdrum_voice, 0.0, Note::new(1.0, 0, 3, None));
            bar.add_note(snare_voice, 0.5, Note::new(1.0, 0, 3, None));
            bar.add_note(bassdrum_voice, 1.0, Note::new(1.0, 0, 3, None));
            bar.add_note(snare_voice, 1.5, Note::new(1.0, 0, 3, None));
            bar.add_note(bassdrum_voice, 2.0, Note::new(1.0, 0, 3, None));
            bar.add_note(snare_voice, 2.5, Note::new(1.0, 0, 3, None));
            bar.add_note(bassdrum_voice, 3.0, Note::new(1.0, 0, 3, None));
            bar.add_note(snare_voice, 3.5, Note::new(1.0, 0, 3, None));
        }
    }
}

pub fn generate_melody<const N: usize>(rng: &mut StdRng, voice: usize, bars: &mut Vec<Bar<N>>) {
    let shapes: Vec<Vec<u8>> = vec![
        vec![6, 0, 0, 0, 0, 0, 2, 0],
        vec![2, 0, 6, 0, 0, 0, 0, 0],
        vec![6, 0, 0, 0, 0, 0, 0, 0],
        vec![8, 0, 0, 0, 0, 0, 0, 0],
        vec![4, 0, 0, 0, 4, 0, 0, 0],
        vec![2, 0, 2, 0, 4, 0, 0, 0],
        vec![4, 0, 0, 0, 2, 0, 0, 0],
        vec![2, 0, 2, 0, 2, 0, 0, 0],
        vec![4, 0, 0, 0, 2, 0, 2, 0],
        vec![2, 0, 2, 0, 2, 0, 2, 0],
    ];

    let selected_shapes: Vec<Vec<u8>> = (0..5)
        .map(|_| shapes[rng.gen_range(0..shapes.len())].clone())
        .collect();

    for i in 0..bars.len() {
        let shape = selected_shapes[rng.gen_range(0..selected_shapes.len())].as_slice();
        let bar = bars.get_mut(i).unwrap();
        for j in 0..8 {
            let pitch = rng.gen_range(0..7);
            if shape[j] == 0 {
                continue;
            }
            let duration = shape[j] as f64 * 0.5;
            bar.add_note(voice, j as f64 * 0.5, Note::new(duration, pitch, 5, None));
            bar.add_note(voice, j as f64 * 0.5, Note::new(duration, pitch, 4, None));
        }
    }
}

pub fn generate_chords<const N: usize>(
    rng: &mut StdRng,
    melody: usize,
    voice: usize,
    bars: &mut Vec<Bar<N>>,
) {
    for i in 0..bars.len() {
        let bar = bars.get_mut(i).unwrap();
        let melody_notes: Vec<Note> = bar.notes[melody].iter().map(|x| x.1.clone()).collect();
        let p1 = melody_notes
            .get(0)
            .cloned()
            .unwrap_or(Note::new(4.0, 0, 5, None));

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
        bar.add_note(voice, 1.0, Note::new(3.0, p1.pitch, 4, p1.accidental));
        bar.add_note(voice, 2.0, Note::new(2.0, p1.pitch + 2, 4, p1.accidental));
        bar.add_note(voice, 3.0, Note::new(1.0, p1.pitch + 4, 4, p1.accidental));
        if rng.gen_bool(0.5) {
            bar.add_note(
                voice,
                0.0,
                Note::new(bar.beats as f64, p1.pitch + 6, 4, p1.accidental),
            );
        }
    }
}
