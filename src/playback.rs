pub mod instrument;
pub mod math;
pub mod synth;

use std::path::Path;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SizedSample, Stream};
use fundsp::hacker::*;

use self::instrument::SoundMaker;

pub fn playback(sound: &dyn SoundMaker) -> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No default output device");
    let config = device.default_output_config().unwrap();

    let mut sound = sound.build();
    sound.0.reset();

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), sound.0),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), sound.0),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), sound.0),
        _ => panic!("Unsupported format"),
    }?;

    stream.play()?;
    std::thread::sleep(sound.1);
    Ok(())
}

pub fn save<P>(sound: &dyn SoundMaker, path: P) -> Result<(), anyhow::Error>
where
    P: AsRef<Path>,
{
    let mut sound = sound.build();
    sound.0.reset();

    let wave = Wave64::render(44100.0, sound.1.as_secs_f64(), &mut sound.0);
    let wave = wave.filter_latency(wave.duration(), &mut (limiter_stereo((5.0, 5.0))));
    wave.save_wav32(path)?;

    Ok(())
}

fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut sound: Net64,
) -> Result<Stream, anyhow::Error>
where
    T: SizedSample + FromSample<f64>,
{
    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    sound.set_sample_rate(sample_rate);
    sound.allocate();

    let mut next_value = move || sound.get_stereo();

    let err_fn = |err| eprintln!("An error has occured on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _| write_data(data, channels, &mut next_value),
        err_fn,
        None,
    )?;

    Ok(stream)
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> (f64, f64))
where
    T: SizedSample + FromSample<f64>,
{
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left = T::from_sample(sample.0);
        let right = T::from_sample(sample.1);

        for (channel, sample) in frame.iter_mut().enumerate() {
            if channel & 1 == 0 {
                *sample = left;
            } else {
                *sample = right;
            }
        }
    }
}
