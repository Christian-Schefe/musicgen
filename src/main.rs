mod playback;
mod score;

use std::rc::Rc;

use fundsp::hacker::*;
use score::{Bar, Dynamic, Key, Note, Score};

use crate::playback::{
    instrument::{Instrument, Sound, SoundMaker, SoundMix},
    playback, save,
    synth::*,
};

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<(), anyhow::Error> {
    let mut score = Score::new(2);
    let key = Rc::new(Key::new(0, true));
    let bpm = 100.0;

    score.add_bar(Bar::new(4, bpm, key.clone(), Dynamic::MezzoForte));
    score.add_bar(Bar::new(4, bpm, key.clone(), Dynamic::MezzoForte));
    score.add_note(0, 0, 0.0, Note::new(0.9, 0, 5));
    score.add_note(0, 0, 1.0, Note::new(0.9, 1, 5));
    score.add_note(0, 0, 2.0, Note::new(0.9, 2, 5));
    score.add_note(0, 0, 3.0, Note::new(0.9, 3, 5));
    score.add_note(0, 1, 0.0, Note::new(4.0, 4, 5));

    // score.add_note(1, 0, 0.0, Note::new(4.0, 0, 5));
    // score.add_note(1, 0, 0.0, Note::new(4.0, 2, 5));
    // score.add_note(1, 0, 0.0, Note::new(4.0, 4, 5));

    let keys_voice = score.convert_to_playable(0);
    let strings_voice = score.convert_to_playable(1);

    println!("{:?}", keys_voice);

    let keys = Instrument::new(Box::new(keys_synth(0.5)), keys_voice);
    let strings = Instrument::new(Box::new(strings_synth(0.5)), strings_voice);
    let sound = SoundMix::mix(vec![Box::new(keys), Box::new(strings)]);

    playback(&sound)?;
    save(&sound)?;
    Ok(())
}
