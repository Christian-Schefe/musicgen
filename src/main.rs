mod generation;
mod playback;
mod score;

use std::rc::Rc;

use fundsp::hacker::*;
use score::{Bar, Dynamic, Key, Note, Score};

use crate::playback::{
    instrument::{mix_instruments, Instrument},
    playback,
    synth::{guitar_synth, keys_synth, strings_synth},
};

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<(), anyhow::Error> {
    // let wave = Wave64::render(44100.0, duration.as_secs_f64(), &mut net);
    // let wave = wave.filter_latency(wave.duration(), &mut (limiter_stereo((5.0, 5.0))));
    // wave.save_wav32("./output/generated.wav")?;

    // net.reset();

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

    println!("{:?}", strings_voice);

    let keys = Instrument::new(Box::new(guitar_synth(0.5)));
    let strings = Instrument::new(Box::new(strings_synth(0.5)));
    let instruments = vec![(keys, keys_voice), (strings, strings_voice)];
    let sound = mix_instruments(instruments);

    playback(sound)?;
    Ok(())
}
