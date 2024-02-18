use fundsp::hacker::*;

use crate::playback::instrument::{Instrument, InstrumentData};

pub fn generic_synth<F>(asdr: (f64, f64, f64, f64), volume: f64, pan: f64, synth_gen: F) -> Instrument where F: Fn() -> Net64 + 'static {
    let data = InstrumentData::new(asdr, pan, volume);

    let sound_fn = move |data: InstrumentData| -> Net64 {
        let sound = synth_gen() * data.volume;
        let sound = sound * adsr_live(data.asdr.0, data.asdr.1, data.asdr.2, data.asdr.3);
        let sound = sound >> fundsp::hacker::pan(data.pan);
        let sound = sound >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));
        Net64::wrap(Box::new(sound))
    };

    Instrument::new(Box::new(sound_fn), 2, data)
}

pub fn square_synth(volume: f64, pan: f64) -> Instrument {
    let data = InstrumentData::new((0.05, 0.0, 1.0, 0.05), pan, volume);

    let sound_fn: fn(InstrumentData) -> Net64 = |data: InstrumentData| -> Net64 {
        let sound = saw() * data.volume;
        let sound = sound * adsr_live(data.asdr.0, data.asdr.1, data.asdr.2, data.asdr.3);
        let sound = sound >> fundsp::hacker::pan(data.pan);
        let sound = sound >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));
        Net64::wrap(Box::new(sound))
    };

    Instrument::new(Box::new(sound_fn), 2, data)
}

pub fn lead(volume: f64, pan: f64) -> Instrument {
    let data = InstrumentData::new((0.05, 0.2, 0.75, 0.1), pan, volume);

    let sound_fn: fn(InstrumentData) -> Net64 = |data: InstrumentData| -> Net64 {
        let sound = (sine_hz(5.0) * 0.015 * (constant(1.0) >> adsr_live(2.0, 0.0, 1.0, 0.0)) + 1.0) * pass();
        let sound = sound >> ((triangle() * data.volume * 0.7) & (0.5 * pass() >> square() * data.volume * 0.3 >> lowpass_hz(1000.0, 0.5))) >> pass() * 0.7;
        let sound = sound * adsr_live(data.asdr.0, data.asdr.1, data.asdr.2, data.asdr.3);
        let sound = sound >> fundsp::hacker::pan(data.pan);
        Net64::wrap(Box::new(sound))
    };

    Instrument::new(Box::new(sound_fn), 2, data)
}

pub fn vibrato_sine_synth(volume: f64, pan: f64) -> Instrument {
    let data = InstrumentData::new((0.05, 0.4, 0.5, 0.2), pan, volume);

    let sound_fn: fn(InstrumentData) -> Net64 = |data: InstrumentData| -> Net64 {
        let sound = (sine_hz(5.0) * 0.015 + 1.0) * pass();
        let sound = sound >> ((triangle() * data.volume * 0.7) & (0.5 * pass() >> sine() * data.volume * 0.3)) >> lowpass_hz(midi_hz(80.0), 1.0) * 0.7;
        let sound = sound * adsr_live(data.asdr.0, data.asdr.1, data.asdr.2, data.asdr.3);
        let sound = sound >> fundsp::hacker::pan(data.pan);
        // let sound = sound >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));
        Net64::wrap(Box::new(sound))
    };

    Instrument::new(Box::new(sound_fn), 2, data)
}

pub fn pad_synth(volume: f64, pan: f64) -> Instrument {
    let data = InstrumentData::new((0.5, 0.0, 1.0, 0.5), pan, volume);

    let sound_fn: fn(InstrumentData) -> Net64 = |data: InstrumentData| -> Net64 {
        let sound = (sine_hz(5.0) * 0.015 + 1.0) * pass();
        let sound = sound >> (triangle() * data.volume * 0.3)
            & (0.5 * pass() >> triangle() * data.volume * 0.6)
            & (2.0 * pass() >> triangle() * data.volume * 0.1);
        let sound = sound * adsr_live(data.asdr.0, data.asdr.1, data.asdr.2, data.asdr.3);
        let sound = sound >> fundsp::hacker::pan(data.pan);
        let sound = sound >> (chorus(0, 0.0, 0.01, 0.2) | chorus(1, 0.0, 0.01, 0.2));
        Net64::wrap(Box::new(sound))
    };

    Instrument::new(Box::new(sound_fn), 2, data)
}
