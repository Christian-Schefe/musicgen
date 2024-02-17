use crate::instrument::{Instrument, InstrumentData};
use fundsp::hacker::*;

pub fn saw_synth(volume: f64, pan: f64) -> Instrument {
    let asdr = (0.05, 0.0, 1.0, 0.05);
    let data = InstrumentData::new(asdr, pan, volume);

    let sound_fn = move || -> Net64 {
        let sound = saw() * data.volume;
        let sound = sound * adsr_live(data.asdr.0, data.asdr.1, data.asdr.2, data.asdr.3);
        let sound = sound >> fundsp::hacker::pan(data.pan);
        let sound = sound >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));
        Net64::wrap(Box::new(sound))
    };

    Instrument::new(
        Box::new(sound_fn),
        2,
        InstrumentData::new(asdr, pan, volume),
    )
}

pub fn vibrato_sine_synth(volume: f64, pan: f64) -> Instrument {
    let asdr = (0.05, 0.0, 1.0, 0.5);
    let data = InstrumentData::new(asdr, pan, volume);

    let sound_fn = move || -> Net64 {
        let sound = (sine_hz(5.0) * 0.015 + 1.0) * pass();
        let sound = sound >> (triangle() * data.volume);
        let sound = sound * adsr_live(data.asdr.0, data.asdr.1, data.asdr.2, data.asdr.3);
        let sound = sound >> fundsp::hacker::pan(data.pan);
        let sound = sound >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));
        Net64::wrap(Box::new(sound))
    };

    Instrument::new(
        Box::new(sound_fn),
        2,
        InstrumentData::new(asdr, pan, volume),
    )
}

pub fn keys_synth(volume: f64, pan: f64) -> Instrument {
    let asdr = (0.001, 0.0, 1.0, 0.1);
    let data = InstrumentData::new(asdr, pan, volume);

    let sound_fn = move || -> Net64 {
        let sound = (sine_hz(5.0) * 0.015 + 1.0) * pass();
        let sound = sound >> (triangle() * data.volume);
        let sound = sound * adsr_live(data.asdr.0, data.asdr.1, data.asdr.2, data.asdr.3);
        let sound = sound >> fundsp::hacker::pan(data.pan);
        let sound = sound >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));
        Net64::wrap(Box::new(sound))
    };

    Instrument::new(
        Box::new(sound_fn),
        2,
        InstrumentData::new(asdr, pan, volume),
    )
}

pub fn pad_synth(volume: f64, pan: f64) -> Instrument {
    let asdr = (0.5, 0.0, 1.0, 0.5);
    let data = InstrumentData::new(asdr, pan, volume);

    let sound_fn = move || -> Net64 {
        let sound = (sine_hz(5.0) * 0.015 + 1.0) * pass();
        let sound = sound >> (triangle() * data.volume * 0.3) & (0.5 * pass() >> triangle() * data.volume * 0.6) & (2.0 * pass() >> triangle() * data.volume * 0.1);
        let sound = sound * adsr_live(data.asdr.0, data.asdr.1, data.asdr.2, data.asdr.3);
        let sound = sound >> fundsp::hacker::pan(data.pan);
        let sound = sound >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));
        Net64::wrap(Box::new(sound))
    };

    Instrument::new(
        Box::new(sound_fn),
        2,
        InstrumentData::new(asdr, pan, volume),
    )
}