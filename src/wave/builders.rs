
use wave::Time;
use wave::voices::Voice;
use wave::shape::{Shape, Waver, Noise, Flow};

pub struct VoiceBuilder {
    pub cur_shape: Option<Shape>,
    pub data: Vec<Waver>,
}

impl From<VoiceBuilder> for Voice {
    fn from(item: VoiceBuilder) -> Voice {
        item.data.into()
    }
}

impl VoiceBuilder {
    pub fn new() -> Self {
        VoiceBuilder {
            cur_shape: None,
            data: Vec::new(),
        }
    }
    pub fn sine(mut self) -> Self {
        self.cur_shape = Some(Shape::Sine);
        self
    }
    pub fn silence(mut self) -> Self {
        self.cur_shape = None;
        self
    }
    pub fn square(mut self) -> Self {
        self.cur_shape = Some(Shape::Square);
        self
    }
    pub fn saw(mut self) -> Self {
        self.cur_shape = Some(Shape::Saw);
        self
    }
    pub fn then(mut self,
                dur: Time,
                s_amp: f64,
                e_amp: Option<f64>,
                s_fq: f64,
                e_fq: Option<f64>)
                -> Self {
        let shape = if let Some(s) = self.cur_shape {
            s
        } else {
            self.data.push(Waver::new_silence(dur));
            return self;
        };
        let amp_flow = if let Some(x) = e_amp {
            Flow::new_linear(s_amp, x, dur)
        } else {
            Flow::new_hold(s_amp, Some(dur))
        };
        let fq_flow = if let Some(x) = e_fq {
            Flow::new_linear(s_fq, x, dur)
        } else {
            Flow::new_hold(s_fq, Some(dur))
        };
        let n = Noise::new(shape, amp_flow, fq_flow);
        self.data.push(Waver::new_noise(n));
        self
    }
}
