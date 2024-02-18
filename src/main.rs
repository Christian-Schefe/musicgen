mod generation;
mod playback;
mod sounds;

use fundsp::hacker::*;
use generation::structure::generate_structure;
use playback::song::Song;
use sounds::*;

use crate::{
    generation::instrumentation::{generate_line, Voicing},
    playback::playback,
};

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<(), anyhow::Error> {
    let mut rng = rand::thread_rng();

    let melody_instrument = lead(0.8, 0.0);
    let melody_instrument2 = lead(0.2, 0.0);

    // let chord_instrument = vibrato_sine_synth(1.0, 0.0);
    let chord_instrument = generic_synth((0.1, 0.0, 1.0, 0.1), 0.7, 0.0, || {
        Net64::wrap(Box::new(
            ((triangle() * 0.7) & (saw() * 0.1) & (square() * 0.2)) >> moog_hz(660.0, 0.5),
        ))
    });

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
    // let song = Song::from_instruments(vec![(melody_instrument, &melody)]);
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
