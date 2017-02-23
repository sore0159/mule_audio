
use wave::voices::Voice;
use wave::shape::{Shape, Waver, Noise};

#[derive(Clone)]
pub struct VoiceBuilder {
    pub cur_shape: Shape,
    pub cur_fq: f64,
    pub cur_amp: f64,

    pub cur_noise: Noise,
    pub data: Vec<Waver>,
}

impl From<VoiceBuilder> for Voice {
    fn from(mut item: VoiceBuilder) -> Voice {
        if item.cur_noise.dur() > 0.0 {
            item.data.push(Waver::new_noise(item.cur_noise));
        }
        item.data.into()
    }
}

impl VoiceBuilder {
    pub fn new(shape: Shape, fq: f64) -> Self {
        VoiceBuilder {
            cur_shape: shape,
            cur_fq: fq,
            cur_amp: 0.0,
            cur_noise: Noise::new(shape, fq),
            data: Vec::new(),
        }
    }
    pub fn next_noise(&mut self, shape: Shape, fq: f64) {
        if self.cur_noise.dur() > 0.0 {
            self.data.push(Waver::new_noise(self.cur_noise.clone()));
        }
        self.cur_shape = shape;
        self.cur_noise = Noise::new(shape, fq);
        self.cur_fq = fq;
    }
    pub fn sine(fq: f64) -> Self {
        VoiceBuilder::new(Shape::Sine, fq)
    }
    pub fn saw(fq: f64) -> Self {
        VoiceBuilder::new(Shape::Saw, fq)
    }
    pub fn square(fq: f64) -> Self {
        VoiceBuilder::new(Shape::Square, fq)
    }
    pub fn set_shape(mut self, shape: Shape) -> Self {
        let fq = self.cur_fq;
        self.next_noise(shape, fq);
        self
    }
    pub fn set_sine(mut self) -> Self {
        let fq = self.cur_fq;
        self.next_noise(Shape::Sine, fq);
        self
    }
    pub fn set_square(mut self) -> Self {
        let fq = self.cur_fq;
        self.next_noise(Shape::Square, fq);
        self
    }
    pub fn set_saw(mut self) -> Self {
        let fq = self.cur_fq;
        self.next_noise(Shape::Saw, fq);
        self
    }
    pub fn hold(mut self, mut dur: f64) -> Self {
        if let Some(&(amp, fq, t)) = self.cur_noise.stats.last() {
            self.cur_fq = fq;
            self.cur_amp = amp;
            dur += t;
        }
        self.cur_noise.push_stats(self.cur_amp, self.cur_fq, dur);
        self
    }
    pub fn silence(mut self, dur: f64) -> Self {
        let fq = self.cur_fq;
        let shape = self.cur_shape;
        self.next_noise(shape, fq);
        self.data.push(Waver::new_silence(dur));
        self
    }
    pub fn linear_amp(mut self, mut dur: f64, end_amp: f64) -> Self {
        dur += self.cur_noise.dur();
        let fq = self.cur_fq;
        self.cur_amp = end_amp;
        self.cur_noise.push_stats(end_amp, fq, dur);
        self
    }
    pub fn linear_fq(mut self, mut dur: f64, end_fq: f64) -> Self {
        dur += self.cur_noise.dur();
        let amp = self.cur_amp;
        self.cur_fq = end_fq;
        self.cur_noise.push_stats(amp, end_fq, dur);
        self
    }
    pub fn linear_both(mut self, mut dur: f64, end_amp: f64, end_fq: f64) -> Self {
        dur += self.cur_noise.dur();
        self.cur_fq = end_fq;
        self.cur_amp = end_amp;
        self.cur_noise.push_stats(end_amp, end_fq, dur);
        self
    }
    pub fn fade(self, dur: f64) -> Self {
        self.linear_amp(dur, 0.0)
    }
    pub fn spike(self, dur: f64, spike_amp: f64) -> Self {
        let cur = self.cur_amp;
        self.linear_amp(dur * 0.5, spike_amp).linear_amp(dur * 0.5, cur)
    }
}
