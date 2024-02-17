mod instrument;
mod mix;
mod playback;
mod sounds;
mod note;
mod generation;

use fundsp::hacker::*;
use generation::generate;
use note::Note;
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
    let instrument = keys_synth(1.0, 0.0);

    // let notes: Vec<Note> = vec![
    //     Note::midi(0.0, 3.0, 61.0, 127.0),
    //     Note::midi(1.0, 2.0, 54.0, 127.0),
    //     Note::midi(2.0, 1.0, 58.0, 127.0),
    //     Note::midi(2.0, 1.0, 66.0, 127.0),
    // ];

    let notes = generate();

    let song = Song::from_instruments(vec![(instrument, &notes)]);
    let (mut net, duration) = song.mix();

    let wave = Wave64::render(44100.0, duration.as_secs_f64(), &mut net);

    let wave = wave.filter(
        wave.duration(),
        &mut (multipass() & 0.15 * reverb_stereo(10.0, 5.0)),
    );
    let wave = wave.filter_latency(wave.duration(), &mut (limiter_stereo((5.0, 5.0))));

    wave.save_wav32("./output/generated.wav")?;

    // net.reset();
    // playback(net, duration)?;
    Ok(())
}
