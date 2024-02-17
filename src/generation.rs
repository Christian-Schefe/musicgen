use crate::note::Note;

use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct Chord(i16, ChordType, ChordInversion);

#[derive(Clone, Debug)]
pub enum ChordInversion {
    Normal,
    FirstInversion,
    SecondInversion,
}

#[derive(Clone, Debug)]
pub enum ChordType {
    Major,
    Minor,
    Diminished,
    Augmented,
}

impl Chord {
    fn base(&self) -> i16 {
        match self.2 {
            ChordInversion::Normal => self.0,
            ChordInversion::FirstInversion => self.0 + self.third_offset(),
            ChordInversion::SecondInversion => self.0 + self.fifth_offset(),
        }
    }
    fn third(&self) -> i16 {
        match self.2 {
            ChordInversion::Normal => self.0 + self.third_offset(),
            ChordInversion::FirstInversion => self.0 + self.fifth_offset(),
            ChordInversion::SecondInversion => self.0 + 12,
        }
    }
    fn fifth(&self) -> i16 {
        match self.2 {
            ChordInversion::Normal => self.0 + self.fifth_offset(),
            ChordInversion::FirstInversion => self.0 + 12,
            ChordInversion::SecondInversion => self.0 + self.third_offset() + 12,
        }
    }
    fn pitches(&self) -> [i16; 3] {
        [self.base(), self.third(), self.fifth()]
    }
    fn third_offset(&self) -> i16 {
        match self.1 {
            ChordType::Major | ChordType::Augmented => 4,
            _ => 3,
        }
    }
    fn fifth_offset(&self) -> i16 {
        match self.1 {
            ChordType::Major | ChordType::Minor => 7,
            ChordType::Diminished => 6,
            ChordType::Augmented => 8,
        }
    }
}

pub fn generate() -> Vec<Note> {
    let mut notes = Vec::new();
    let mut rng = rand::thread_rng();

    let tonic = 55;

    let chords = generate_chords(&mut rng);

    let time_per_chord = 4.0;

    for (i, mut chord) in chords.into_iter().enumerate() {
        println!("{:?}", chord);
        if chord.third() >= 12 {
            chord.0 -= 12;
            println!("modified - {:?}", chord);
        }
        notes.push(Note::midi(time_per_chord * i as f64, time_per_chord, (chord.base() + tonic) as f64, 127.0));
        notes.push(Note::midi(time_per_chord * i as f64, time_per_chord, (chord.third() + tonic - 12) as f64, 127.0));
        notes.push(Note::midi(time_per_chord * i as f64, time_per_chord, (chord.fifth() + tonic) as f64, 127.0));

        for j in 0..4 {
            let pitches = chord.pitches();
            let pitch = pitches[rng.gen_range(0..pitches.len())];
            notes.push(Note::midi(time_per_chord * i as f64 + 0.25 * time_per_chord * j as f64, time_per_chord * 0.25, (pitch + tonic + 12) as f64, 80.0));
        }
    }

    notes
}

pub fn generate_chords(rng: &mut ThreadRng) -> Vec<Chord> {
    let scale = [
        Chord(0, ChordType::Major, ChordInversion::Normal),
        Chord(2, ChordType::Minor, ChordInversion::Normal),
        Chord(4, ChordType::Minor, ChordInversion::Normal),
        Chord(5, ChordType::Major, ChordInversion::Normal),
        Chord(7, ChordType::Major, ChordInversion::Normal),
        Chord(9, ChordType::Minor, ChordInversion::Normal),
        Chord(11, ChordType::Diminished, ChordInversion::Normal),
    ];
    let mut chords = vec![scale[0].clone()];

    for _ in 0..29 {
        let mut chord = scale[rng.gen_range(0..scale.len())].clone();
        let inversion: ChordInversion = match rng.gen_range(0..=2) {
            0 => ChordInversion::Normal,
            1 => ChordInversion::FirstInversion,
            _ => ChordInversion::SecondInversion,
        };
        chord.2 = inversion;
        chords.push(chord)
    }

    chords.push(scale[4].clone());
    chords.push(scale[0].clone());
    chords
}
