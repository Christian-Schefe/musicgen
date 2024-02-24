use crate::hacker::*;

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
    KeyTracked(f64, f64, f64, f64),
}

impl Parameter {
    pub fn as_node(&self) -> Net64 {
        match self {
            Self::Const(val) => Net64::wrap(Box::new(constant(*val) | multisink::<U3>())),
            Self::Enveloped(envelope, min_val, max_val) => Net64::wrap(Box::new(
                sink() | envelope.ranged_asdr(*min_val, *max_val) | sink(),
            )),
            Self::KeyTracked(kmi, kma, vmi, vma) => Net64::wrap(Box::new(
                self.keytracked_freq((*kmi, *kma), (*vmi, *vma)) | multisink::<U2>(),
            )),
        }
    }
    fn keytracked_freq(
        &self,
        freq_range: (f64, f64),
        key_range: (f64, f64),
    ) -> An<impl AudioNode<Sample = f64, Inputs = U1, Outputs = U1>> {
        freq_range.0
            + (freq_range.1 - freq_range.0)
                * (pass() - midi_hz(key_range.0))
                * (1.0 / (midi_hz(key_range.1) - midi_hz(key_range.0)))
    }
}

pub trait Synth {
    fn instantiate(&self) -> Net64;
    fn release_time(&self) -> f64;
}

#[derive(Debug, Clone)]
pub struct SimpleSynth {
    pub envelope: Envelope,
    pub mix: [f64; 4],
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
    fn new(envelope: Envelope, mix: [f64; 4], harmonics: Vec<(f64, f64)>) -> Self {
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
        (saw() * self.mix[0])
            & (square() * self.mix[1])
            & (triangle() * self.mix[2])
            & (sine() * self.mix[3])
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
        self.freq_mod(self.synth.instantiate())
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
    fn freq_mod(&self, synth: Net64) -> Net64 {
        let stream = (pass() | sink() | sink())
            ^ self.amplitude.as_node()
            ^ (self.frequency.as_node() >> sine())
            ^ (sink() | pass() | pass());
        stream >> ((pass() * (1.0 + pass() * pass())) | pass() | pass()) >> synth
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
        [0.0, 0.05, 0.75, 0.2],
        vec![(1.0, 0.8), (0.5, 0.1), (2.0, 0.1)],
    );

    let synth2 = SimpleSynth::new(
        Envelope(0.02, 2.0, 0.0, 0.0),
        [0.0, 0.0, 0.5, 0.5],
        vec![(1.0, 0.9), (0.5, 0.05), (2.0, 0.05)],
    );

    let low_filter = Some(Filter(
        Parameter::KeyTracked(4000.0, 6000.0, 60.0, 72.0),
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
        [0.7, 0.2, 0.05, 0.05],
        vec![(1.0, 0.75), (0.5, 0.1), (2.0, 0.15)],
    );

    let low_filter = Some(Filter(
        Parameter::KeyTracked(6000.0, 10000.0, 60.0, 72.0),
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
