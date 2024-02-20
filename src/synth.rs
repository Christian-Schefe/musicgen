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
        self.enveloped(self.harmonics_mix())
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
    fn enveloped(&self, net: Net64) -> Net64 {
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
pub enum Filter {
    Thru,
    Simple(f64, f64),
    Enveloped(Envelope, (f64, f64), f64),
}

pub struct SynthFilter {
    pub lowpass: Filter,
    pub highpass: Filter,
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
    pub fn new(synth: Box<dyn Synth>, lowpass: Filter, highpass: Filter) -> Self {
        Self {
            lowpass,
            highpass,
            synth,
        }
    }
    fn lowpassed(&self, synth: Net64) -> Net64 {
        match &self.lowpass {
            Filter::Thru => synth,
            Filter::Simple(freq, q) => synth >> lowpass_hz(*freq, *q),
            Filter::Enveloped(env, (min, max), q) => {
                (synth ^ (sink() | env.ranged_asdr(*min, *max) | sink())) >> lowpass_q(*q)
            }
        }
    }
    fn highpassed(&self, synth: Net64) -> Net64 {
        match &self.highpass {
            Filter::Thru => synth,
            Filter::Simple(freq, q) => synth >> highpass_hz(*freq, *q),
            Filter::Enveloped(env, (min, max), q) => {
                (synth ^ (sink() | env.ranged_asdr(*min, *max) | sink())) >> highpass_q(*q)
            }
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
            >> (multipass() & self.reverb_mix * reverb_stereo(self.reverb_size, self.reverb_time)))
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
    pub frequency: f64,
    pub amplitude: f64,
}

impl Synth for SynthVibrato {
    fn instantiate(&self) -> Net64 {
        (((1.0 + self.amplitude * sine_hz(self.frequency)) * pass()) | pass() | pass())
            >> self.synth.instantiate()
    }
    fn release_time(&self) -> f64 {
        self.synth.release_time()
    }
}

impl SynthVibrato {
    pub fn new(synth: Box<dyn Synth>, frequency: f64, amplitude: f64) -> Self {
        Self {
            synth,
            frequency,
            amplitude,
        }
    }
}

pub fn keys_synth(volume: f64) -> impl Synth {
    let synth = SimpleSynth::new(
        Envelope(0.05, 0.3, 0.6, 0.3),
        [0.25, 0.25, 0.25, 0.25],
        vec![(1.0, 0.6), (0.5, 0.3), (2.0, 0.1)],
    );

    let filter = Filter::Simple(2000.0, 0.5);
    let filtered_synth = SynthFilter::new(Box::new(synth), filter, Filter::Thru);
    SynthMaster::new(Box::new(filtered_synth), 10.0, 5.0, 0.2, 0.0, volume)
}
