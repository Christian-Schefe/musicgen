mod instrument;

use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SizedSample};
use fundsp::hacker::*;
use instrument::{create_instrument, Song};

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No default output device");
    let config = device.default_output_config().unwrap();

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()).unwrap(),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()).unwrap(),
        _ => panic!("Unsupported format"),
    }
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f64>,
{
    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    let i1 = create_instrument(64.0, 0.4);
    let i2 = create_instrument(61.0, 0.4);
    let i3 = create_instrument(57.0, 0.4);

    let mut song = Song::new(
        vec![i1, i2, i3],
        vec![
            (vec![(61.0, 0.6), (65.0, 0.6), (68.0, 0.6)], 1000),
            (vec![(63.0, 0.6), (66.0, 0.6), (70.0, 0.6)], 1000),
            (vec![(65.0, 0.6), (68.0, 0.6), (72.0, 0.6)], 2000),
            (vec![(63.0, 0.6), (66.0, 0.6), (70.0, 0.6)], 1000),
            (vec![(65.0, 0.6), (68.0, 0.6), (72.0, 0.6)], 500),
            (vec![(63.0, 0.6), (66.0, 0.6), (70.0, 0.6)], 500),
            (vec![(61.0, 0.6), (65.0, 0.6), (68.0, 0.6)], 2000),
            (vec![(64.0, 0.0), (64.0, 0.0), (64.0, 0.0)], 1000)
        ],
    );
    let mut c = song.build_network();

    c.set_sample_rate(sample_rate);
    c.allocate();

    let mut next_value = move || c.get_stereo();

    let err_fn = |err| eprintln!("An error has occured on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _| write_data(data, channels, &mut next_value),
        err_fn,
        None,
    )?;

    stream.play()?;

    let play_thread = std::thread::spawn(move || {
        song.play();
    });

    let r = play_thread.join();
    println!("finished");
    std::thread::sleep(Duration::from_millis(1000));

    Ok(())
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
