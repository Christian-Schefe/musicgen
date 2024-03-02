use crate::hacker::*;

use super::math::{compress, compress_up, distort, hz_midi, remap};

#[derive(Debug, Clone)]
pub struct Envelope(pub f64, pub f64, pub f64, pub f64);

impl Envelope {
    pub fn as_asdr(&self) -> An<impl AudioNode<Sample = f64, Inputs = U1, Outputs = U1>> {
        adsr_live(self.0, self.1, self.2, self.3)
    }
    pub fn ranged_asdr(
        &self,
        min: f64,
        max: f64,
    ) -> An<impl AudioNode<Sample = f64, Inputs = U1, Outputs = U1>> {
        min + (max - min) * self.as_asdr()
    }
}

#[derive(Debug, Clone)]
pub enum Parameter {
    Const(f64),
    Enveloped(Envelope, f64, f64),
    KeyTracked((f64, f64), (f64, f64), bool),
}

impl Parameter {
    pub fn as_node(&self) -> Net64 {
        match self {
            Self::Const(val) => Net64::wrap(Box::new(constant(*val) | multisink::<U3>())),
            Self::Enveloped(envelope, min_val, max_val) => Net64::wrap(Box::new(
                sink() | envelope.ranged_asdr(*min_val, *max_val) | sink(),
            )),
            Self::KeyTracked(in_range, out_range, is_midi) => Net64::wrap(Box::new(
                self.keytracked(in_range, out_range, *is_midi) | multisink::<U2>(),
            )),
        }
    }
    fn keytracked(
        &self,
        in_range: &(f64, f64),
        out_range: &(f64, f64),
        track_midi: bool,
    ) -> An<impl AudioNode<Sample = f64, Inputs = U1, Outputs = U1>> {
        let in_min = in_range.0;
        let in_max = in_range.1;
        let out_min = out_range.0;
        let out_max = out_range.1;
        map(move |x: &Frame<f64, U1>| {
            let midi = if track_midi { hz_midi(x[0]) } else { x[0] };
            let val = remap(midi, in_min, in_max, out_min, out_max);
            val
        })
    }
}

pub trait Synth {
    fn instantiate(&self) -> Net64;
    fn release_time(&self) -> f64;
}

#[derive(Debug, Clone)]
pub struct WaveMix {
    square: f64,
    saw: f64,
    sine: f64,
    triangle: f64,
    pulse: f64,
    noise: f64,
}

impl WaveMix {
    fn new(square: f64, saw: f64, sine: f64, triangle: f64, pulse: f64, noise: f64) -> Self {
        Self {
            square,
            saw,
            sine,
            triangle,
            pulse,
            noise,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimpleSynth {
    pub envelope: Envelope,
    pub mix: WaveMix,
    pub harmonics: Vec<(f64, f64)>,
}

impl Synth for SimpleSynth {
    fn instantiate(&self) -> Net64 {
        self.volume_adjusted(self.harmonics_mix())
    }
    fn release_time(&self) -> f64 {
        self.envelope.3
    }
}

impl SimpleSynth {
    fn new(envelope: Envelope, mix: WaveMix, harmonics: Vec<(f64, f64)>) -> Self {
        Self {
            envelope,
            mix,
            harmonics,
        }
    }
    fn volume_adjusted(&self, net: Net64) -> Net64 {
        net * self.envelope.as_asdr() * pass()
    }
    fn harmonics_mix(&self) -> Net64 {
        let mut net = Net64::new(1, 1);
        for &(freq, vol) in self.harmonics.iter() {
            let freq_mod = (pass() * freq) >> (self.waveform_mix() * vol);
            net = net & freq_mod;
        }
        net
    }
    fn waveform_mix(&self) -> An<impl AudioNode<Sample = f64, Inputs = U1, Outputs = U1>> {
        (saw() * self.mix.saw)
            & (square() * self.mix.square)
            & (triangle() * self.mix.triangle)
            & (sine() * self.mix.sine)
            & (((pass() | constant(0.0)) >> pulse()) * self.mix.pulse)
            & (sink() | noise() * self.mix.noise)
    }
}

#[derive(Debug, Clone)]
pub struct Filter(Parameter, f64);

pub struct SynthFilter {
    pub lowpass: Option<Filter>,
    pub highpass: Option<Filter>,
    pub synth: Box<dyn Synth>,
}

impl Synth for SynthFilter {
    fn instantiate(&self) -> Net64 {
        self.highpassed(self.lowpassed(self.synth.instantiate()))
    }
    fn release_time(&self) -> f64 {
        self.synth.release_time()
    }
}

impl SynthFilter {
    pub fn new(synth: Box<dyn Synth>, lowpass: Option<Filter>, highpass: Option<Filter>) -> Self {
        Self {
            lowpass,
            highpass,
            synth,
        }
    }
    fn lowpassed(&self, synth: Net64) -> Net64 {
        if let Some(filter) = &self.lowpass {
            (synth ^ filter.0.as_node()) >> lowpass_q(filter.1)
        } else {
            synth
        }
    }
    fn highpassed(&self, synth: Net64) -> Net64 {
        if let Some(filter) = &self.highpass {
            (synth ^ filter.0.as_node()) >> highpass_q(filter.1)
        } else {
            synth
        }
    }
}

pub struct SynthMaster {
    pub synth: Box<dyn Synth>,
    pub reverb_size: f64,
    pub reverb_time: f64,
    pub reverb_mix: f64,
    pub pan: f64,
    pub volume: f64,
}

impl Synth for SynthMaster {
    fn instantiate(&self) -> Net64 {
        (self.synth.instantiate()
            >> pan(self.pan)
            >> (multipass()
                & (self.reverb_mix * reverb_stereo(self.reverb_size, self.reverb_time))))
            * self.volume
    }
    fn release_time(&self) -> f64 {
        self.synth.release_time()
    }
}

impl SynthMaster {
    pub fn new(
        synth: Box<dyn Synth>,
        reverb_size: f64,
        reverb_time: f64,
        reverb_mix: f64,
        pan: f64,
        volume: f64,
    ) -> Self {
        Self {
            synth,
            reverb_size,
            reverb_time,
            reverb_mix,
            pan,
            volume,
        }
    }
}

pub struct SynthVibrato {
    pub synth: Box<dyn Synth>,
    pub frequency: Parameter,
    pub amplitude: Parameter,
}

impl Synth for SynthVibrato {
    fn instantiate(&self) -> Net64 {
        self.vibrato(self.synth.instantiate())
    }
    fn release_time(&self) -> f64 {
        self.synth.release_time()
    }
}

impl SynthVibrato {
    pub fn new(synth: Box<dyn Synth>, frequency: Parameter, amplitude: Parameter) -> Self {
        Self {
            synth,
            frequency,
            amplitude,
        }
    }
    fn vibrato(&self, synth: Net64) -> Net64 {
        // (multipass::<U3>() ^ self.amplitude.as_node() ^ (self.frequency.as_node() >> sine()))
        //     >> map(|x: &Frame<f64, U5>| (x[0] * (1.0 + x[3] * x[4]), x[1], x[2]))
        //     >> synth
        (multipass::<U3>() ^ self.amplitude.as_node() ^ (self.frequency.as_node() >> sine()))
            >> map(|x: &Frame<f64, U5>| {
                (
                    x[0] * xerp(1.0 + x[3], 1.0 / (1.0 + x[3]), x[4]),
                    x[1],
                    x[2],
                )
            })
            >> synth
    }
}

pub struct SynthEffect<F, T>
where
    F: Fn() -> T,
    T: AudioUnit64,
{
    pub synth: Box<dyn Synth>,
    pub effect: F,
}

impl<F, T> Synth for SynthEffect<F, T>
where
    F: Fn() -> T,
    T: AudioUnit64 + 'static,
{
    fn instantiate(&self) -> Net64 {
        let node = (self.effect)();
        self.synth.instantiate() >> Net64::wrap(Box::new(node))
    }
    fn release_time(&self) -> f64 {
        self.synth.release_time()
    }
}

impl<F, T> SynthEffect<F, T>
where
    F: Fn() -> T,
    T: AudioUnit64,
{
    pub fn new(synth: Box<dyn Synth>, effect: F) -> Self {
        Self { synth, effect }
    }
}

pub struct SynthLayer {
    pub layers: Vec<(Box<dyn Synth>, f64)>,
}

impl Synth for SynthLayer {
    fn instantiate(&self) -> Net64 {
        self.layers.iter().fold(Net64::new(3, 1), Self::mix_layer)
    }
    fn release_time(&self) -> f64 {
        self.layers
            .iter()
            .map(|x| x.0.release_time())
            .reduce(f64::max)
            .unwrap()
    }
}

impl SynthLayer {
    pub fn new(layers: Vec<(Box<dyn Synth>, f64)>) -> Self {
        Self { layers }
    }
    fn mix_layer(synth: Net64, layer: &(Box<dyn Synth>, f64)) -> Net64 {
        synth & (layer.0.instantiate() * layer.1)
    }
}

pub fn keys_synth(volume: f64) -> impl Synth {
    let synth = SimpleSynth::new(
        Envelope(0.02, 0.45, 0.0, 0.45),
        WaveMix::new(0.0, 0.05, 0.75, 0.2, 0.0, 0.0),
        vec![(1.0, 0.8), (0.5, 0.1), (2.0, 0.1)],
    );

    let synth2 = SimpleSynth::new(
        Envelope(0.02, 2.0, 0.0, 0.0),
        WaveMix::new(0.0, 0.0, 0.5, 0.5, 0.0, 0.0),
        vec![(1.0, 0.9), (0.5, 0.05), (2.0, 0.05)],
    );

    let low_filter = Some(Filter(
        Parameter::KeyTracked((60.0, 72.0), (4000.0, 6000.0), true),
        0.1,
    ));
    let high_filter = Some(Filter(Parameter::Const(200.0), 0.5));

    let filtered_synth = SynthFilter::new(Box::new(synth), low_filter, high_filter);
    let layerd_synth = SynthLayer::new(vec![
        (Box::new(filtered_synth), 1.0),
        (Box::new(synth2), 0.1),
    ]);

    SynthMaster::new(Box::new(layerd_synth), 10.0, 2.5, 0.0, 0.0, volume)
}

pub fn strings_synth(volume: f64) -> impl Synth {
    let synth = SimpleSynth::new(
        Envelope(0.3, 1.0, 0.8, 0.1),
        WaveMix::new(0.7, 0.2, 0.05, 0.05, 0.0, 0.0),
        vec![(1.0, 0.75), (0.5, 0.1), (2.0, 0.15)],
    );

    let low_filter = Some(Filter(
        Parameter::KeyTracked((60.0, 72.0), (6000.0, 10000.0), true),
        0.1,
    ));
    let high_filter = Some(Filter(Parameter::Const(200.0), 0.5));

    let filtered_synth = SynthFilter::new(Box::new(synth), low_filter, high_filter);
    let vibrato_synth = SynthVibrato::new(
        Box::new(filtered_synth),
        Parameter::Const(5.0),
        Parameter::Enveloped(Envelope(2.0, 0.0, 1.0, 0.0), 0.0, 0.006),
    );

    SynthMaster::new(Box::new(vibrato_synth), 10.0, 2.5, 1.0, 0.0, volume)
}

pub fn guitar_synth(volume: f64) -> impl Synth {
    let synth = SimpleSynth::new(
        Envelope(0.02, 2.0, 0.0, 2.0),
        WaveMix::new(0.31, 0.31, 0.0, 0.0, 0.31, 0.06),
        vec![(1.0, 1.0)],
    );

    let low_filter = Some(Filter(
        Parameter::KeyTracked((60.0, 72.0), (6000.0, 10000.0), true),
        0.3,
    ));
    let high_filter = Some(Filter(Parameter::Const(200.0), 0.5));

    let filtered_synth = SynthFilter::new(Box::new(synth), low_filter, high_filter);
    let vibrato_synth = SynthVibrato::new(
        Box::new(filtered_synth),
        Parameter::Const(2.0),
        Parameter::Enveloped(Envelope(2.0, 0.0, 1.0, 0.0), 0.0, 0.004),
    );

    let effect = SynthEffect::new(Box::new(vibrato_synth), || {
        moog_hz(600.0, 0.3)
            >> map(|x: &Frame<f64, U1>| {
                let y = compress(x[0], 0.05, 0.0) * 20.0;
                let y = distort(y, 20.0, 20.0);
                y
            })
            >> clip()
    });

    SynthMaster::new(Box::new(effect), 40.0, 4.5, 0.7, 0.0, volume)
}
