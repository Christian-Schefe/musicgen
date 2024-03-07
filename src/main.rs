mod generation;
mod playback;
mod score;

use std::rc::Rc;

use fundsp::hacker::*;
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
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
    let mut seed_rng = thread_rng();
    let seed: u32 = seed_rng.gen();
    println!("Seed: {}", seed);
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let key = Rc::new(Key::new(rng.gen_range(-0..=4), rng.gen_bool(0.5)));
    let bpm = rng.gen_range(90..=130) as f64;

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

    let [keys_voice, strings_voice, drums_voice, snare_voice] = voices;

    let lead = Instrument::new(Box::new(random_lead(&mut rng, 0.65)), keys_voice);
    let chords = Instrument::new(Box::new(strings_synth(0.95)), strings_voice);
    let bassdrum = Instrument::new(Box::new(bassdrum_synth(1.0)), drums_voice);
    let snare = Instrument::new(Box::new(snare_synth(1.0)), snare_voice);

    let sound = SoundMix::mix(vec![
        Box::new(lead),
        Box::new(chords),
        Box::new(bassdrum),
        Box::new(snare),
    ]);

    println!("Saving...");
    save(&sound, format!("./output/gen_{}.wav", seed))?;
    println!("Playing...");
    playback(&sound)?;
    Ok(())
}
