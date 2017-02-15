use super::shape::*;
use super::Time;

#[derive(Clone)]
pub struct FuncBuilder {
    shape: Shape,
    fq_data: f64,
    amp_data: f64,
    amp_maybe: Option<(f64, f64)>,
    fq_maybe: Option<(f64, f64)>,
}

impl FuncBuilder {
    pub fn amp(self, amp: f64) -> Self {
        FuncBuilder { amp_data: amp, ..self }
    }
    pub fn linear_amp(self, start: f64, end: f64) -> Self {
        FuncBuilder { amp_maybe: Some((start, end)), ..self }
    }
    pub fn linear_fq(self, start: f64, end: f64) -> Self {
        FuncBuilder { fq_maybe: Some((start, end)), ..self }
    }
    pub fn dur(self, dur: Time) -> Func {
        let amp_data: Flow;
        let fq_data: Flow;
        if let Some((start, end)) = self.amp_maybe {
            amp_data = Flow::new_linear(start, end, dur);
        } else {
            amp_data = Flow::new_hold(self.amp_data, Some(dur));
        }
        if let Some((start, end)) = self.fq_maybe {
            fq_data = Flow::new_linear(start, end, dur);
        } else {
            fq_data = Flow::new_hold(self.fq_data, Some(dur));
        }
        Func::new(self.shape, amp_data, fq_data)
    }
    pub fn forever(self) -> Func {
        let amp_data = Flow::new_hold(self.amp_data, None);
        let fq_data = Flow::new_hold(self.fq_data, None);
        Func::new(self.shape, amp_data, fq_data)

    }
}

pub fn sine(fq: f64) -> FuncBuilder {
    FuncBuilder {
        shape: Shape::Sine,
        amp_data: 1.0,
        fq_data: fq,
        amp_maybe: None,
        fq_maybe: None,
    }
}

pub fn saw(fq: f64) -> FuncBuilder {
    FuncBuilder {
        shape: Shape::Saw,
        amp_data: 1.0,
        fq_data: fq,
        amp_maybe: None,
        fq_maybe: None,
    }
}

pub fn square(fq: f64) -> FuncBuilder {
    FuncBuilder {
        shape: Shape::Square,
        amp_data: 1.0,
        fq_data: fq,
        amp_maybe: None,
        fq_maybe: None,
    }
}
pub fn silence(dur: Time) -> Silence {
    Silence::new(dur)
}
