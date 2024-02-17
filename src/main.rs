mod instrument;
mod mix;
mod playback;
mod sounds;
mod note;

use fundsp::hacker::*;
use sounds::*;
use mix::Song;
use playback::playback;

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn run() -> Result<(), anyhow::Error> {
    let instrument = pad_synth(1.0, 0.0);
    // let instrument2 = vibrato_sine_synth(0.0, 0.0);


    let notes: Vec<(f64, f64)> = vec![
        (54.0, 3.0),
        (61.0, 3.0),
        (68.0, 3.0),
    ];
    let notes2: Vec<(f64, f64)> = vec![(59.0, 1.0), (60.0, 3.0), (59.0, 2.0), (68.0, 2.0)];

    let song = Song::from_instruments(vec![(instrument, &notes)]);
    let (mut net, duration) = song.mix();

    let wave = Wave64::render(44100.0, duration.as_secs_f64(), &mut net);
    wave.save_wav32("./test.wav")?;

    net.reset();
    playback(net, duration)?;
    Ok(())
}
