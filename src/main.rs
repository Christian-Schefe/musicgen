mod note;
mod scale;

use std::time::Duration;

use cpal::{SizedSample, FromSample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::hacker::*;

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

    let c = 0.2 * (organ_hz(midi_hz(57.0)) + (organ_hz(midi_hz(61.0))) + (organ_hz(midi_hz(64.0))));
    let c = c >> pan(0.0);
    let c = c >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));

    let mut c = c >> (declick() | declick()) >> (dcblock() | dcblock()) >> limiter_stereo((1.0, 5.0));

    c.set_sample_rate(sample_rate);
    c.allocate();

    let mut next_value = move || c.get_stereo();

    let err_fn = |err| eprintln!("An error has occured on stream: {}", err);

    let stream = device.build_output_stream(config, move |data: &mut[T], _| {
        write_data(data, channels, &mut next_value)
    }, err_fn, None)?;

    stream.play()?;

    std::thread::sleep(Duration::from_millis(50000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> (f64, f64)) where T: SizedSample + FromSample<f64> {
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