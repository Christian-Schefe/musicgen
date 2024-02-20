mod generation;
mod playback;
mod synth;

use fundsp::hacker::*;
use generation::structure::generate_structure;
use playback::song::Song;

use crate::{
    generation::instrumentation::{generate_line, Voicing},
    playback::{instrument::Instrument, playback},
    synth::keys_synth,
};

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<(), anyhow::Error> {
    let mut rng = rand::thread_rng();

    let melody_instrument = Instrument::new(Box::new(keys_synth(1.0, 0.0)));
    let melody_instrument2 = Instrument::new(Box::new(keys_synth(1.0, 0.0)));

    let chord_instrument = Instrument::new(Box::new(keys_synth(1.0, 0.0)));

    let structure = generate_structure(&mut rng, 8);

    let melody = generate_line(&mut rng, &structure, Voicing::Melody((55, 66)));
    let melody2 = generate_line(&mut rng, &structure, Voicing::Melody((62, 75)));
    let chords = generate_line(&mut rng, &structure, Voicing::Chords((40, 61)));

    println!("{:?}", structure);

    structure
        .phrases
        .iter()
        .for_each(|x| x.harmony.iter().for_each(|y| println!("{:?}", y)));

    let song = Song::from_instruments(vec![
        (melody_instrument, &melody),
        (melody_instrument2, &melody2),
        (chord_instrument, &chords),
    ]);
    
    let (mut net, duration) = song.mix();

    let wave = Wave64::render(44100.0, duration.as_secs_f64(), &mut net);

    let wave = wave.filter(
        wave.duration(),
        &mut (multipass() & 0.15 * reverb_stereo(10.0, 5.0)),
    );
    let wave = wave.filter_latency(wave.duration(), &mut (limiter_stereo((5.0, 5.0))));

    wave.save_wav32("./output/generated.wav")?;

    net.reset();
    playback(net, duration)?;
    Ok(())
}
