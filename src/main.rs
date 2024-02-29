mod generation;
mod playback;
mod score;

use fundsp::hacker::*;
use generation::structure::generate_structure;

use crate::{
    generation::{instrumentation::{generate_line, Voicing}, melody::gen_melody},
    playback::{
        instrument::{Instrument, mix_instruments},
        playback,
        synth::{keys_synth, strings_synth},
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

    let piece = generate_structure(&mut rng, 32, (60, 80));

    let melody = gen_melody(&mut rng, &piece, (60, 72));
    let chords = generate_line(&mut rng, &piece, Voicing::Chords((48, 59)));

    let sound = mix_instruments(vec![(melody_instrument, melody), (chord_instrument, chords)]);

    // let wave = Wave64::render(44100.0, duration.as_secs_f64(), &mut net);
    // let wave = wave.filter_latency(wave.duration(), &mut (limiter_stereo((5.0, 5.0))));
    // wave.save_wav32("./output/generated.wav")?;

    // net.reset();
    playback(sound)?;
    Ok(())
}
