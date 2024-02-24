mod generation;
mod playback;

use fundsp::hacker::*;
use generation::structure::generate_structure;

use crate::{
    generation::instrumentation::{generate_line, Voicing},
    playback::{
        instrument::{Instrument, mix_instruments},
        playback,
        synth::{keys_synth, strings_synth}, tone::Tone,
    },
};

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<(), anyhow::Error> {
    let mut rng = rand::thread_rng();

    let melody_instrument = Instrument::new(Box::new(strings_synth(0.5)));
    let chord_instrument = Instrument::new(Box::new(keys_synth(1.0)));

    let structure = generate_structure(&mut rng, 8);

    let melody = generate_line(&mut rng, &structure, Voicing::Melody((60, 71)));
    let chords = generate_line(&mut rng, &structure, Voicing::Chords((48, 59)));

    println!("{:?}", structure);

    structure
        .phrases
        .iter()
        .for_each(|x| x.harmony.iter().for_each(|y| println!("{:?}", y)));

    let sound = mix_instruments(vec![(melody_instrument, melody), (chord_instrument, chords)]);
    // let sound = mix_instruments(vec![(melody_instrument, vec![Tone::midi(0.0, 20.0, 70.0, 127.0)])]);

    // let wave = Wave64::render(44100.0, duration.as_secs_f64(), &mut net);
    // let wave = wave.filter_latency(wave.duration(), &mut (limiter_stereo((5.0, 5.0))));
    // wave.save_wav32("./output/generated.wav")?;

    // net.reset();
    playback(sound)?;
    Ok(())
}
