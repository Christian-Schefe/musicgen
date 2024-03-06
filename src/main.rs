mod generation;
mod playback;
mod score;

use std::rc::Rc;

use fundsp::hacker::*;
use rand::{thread_rng, Rng};
use score::{Dynamic, Key, Score};

use crate::{
    generation::{generate_section, SectionSettings},
    playback::{
        instrument::{Instrument, SoundMix},
        playback, save,
        synth::*,
    },
};

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<(), anyhow::Error> {
    let mut rng = thread_rng();
    let key = Rc::new(Key::new(rng.gen_range(-2..=2), rng.gen_bool(0.5)));
    let bpm = 110.0;

    let intro = generate_section(
        &mut rng,
        SectionSettings::new(4, key.clone(), bpm, 4, Dynamic::MezzoForte),
    );

    let sec_a = generate_section(
        &mut rng,
        SectionSettings::new(8, key.clone(), bpm, 4, Dynamic::Forte),
    );

    let sec_b = generate_section(
        &mut rng,
        SectionSettings::new(8, key.clone(), bpm, 4, Dynamic::MezzoForte),
    );

    let outro = generate_section(
        &mut rng,
        SectionSettings::new(4, key.clone(), bpm, 4, Dynamic::MezzoPiano),
    );

    let score = Score::from_sections(vec![
        intro,
        sec_a.clone(),
        sec_b.clone(),
        sec_a,
        sec_b,
        outro,
    ]);

    let voices = score.convert_to_playable();

    println!("{:?}", voices);
    let [keys_voice, strings_voice] = voices;

    println!("{:?}", strings_voice);
    let keys = Instrument::new(Box::new(strings_synth(0.65)), keys_voice);
    let strings = Instrument::new(Box::new(sustain_keys_synth(0.5)), strings_voice);
    let sound = SoundMix::mix(vec![Box::new(keys), Box::new(strings)]);

    save(&sound)?;
    playback(&sound)?;
    Ok(())
}
