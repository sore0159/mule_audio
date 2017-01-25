use super::*;

#[derive(Clone)]
pub struct Func {
    pub timer: TimeAdjust,
    pub shape: Shape,
    pub amp: Flow,
    pub fq: Flow,
}

impl Func {
    pub fn new(shape: Shape, amp: Flow, fq: Flow) -> Self {
        Func {
            timer: TimeAdjust::new(),
            shape: shape,
            amp: amp,
            fq: fq,
        }
    }
}

impl Wave for Func {
    fn val(&mut self, t: Time) -> Option<f32> {
        let dt = self.timer.dt(t);
        let (amp_maybe, fq_maybe) = (self.amp.val(dt), self.fq.val(dt));
        if let Some(amp) = amp_maybe {
            if let Some(fq) = fq_maybe {
                return Some(self.shape.val(amp, fq, dt));
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct Silence {
    pub timer: TimeAdjust,
    pub duration: Time,
}

impl Silence {
    pub fn new(dur: Time) -> Self {
        Silence {
            timer: TimeAdjust::new(),
            duration: dur,
        }
    }
}


impl Wave for Silence {
    fn val(&mut self, t: Time) -> Option<f32> {
        let dt = self.timer.dt(t);
        if dt < 0.0 || dt > self.duration {
            None
        } else {
            Some(0.0)
        }
    }
}

#[derive(Clone)]
pub enum Flow {
    Hold(f64, Option<Time>),
    Linear(f64, f64, Time),
}

impl Flow {
    pub fn new_hold(val: f64, dur: Option<Time>) -> Flow {
        Flow::Hold(val, dur)
    }
    pub fn new_linear(start: f64, stop: f64, dur: Time) -> Flow {
        Flow::Linear(start, (stop - start) / dur, dur)
    }
    pub fn val(&self, t: Time) -> Option<f64> {
        match self {
            &Flow::Hold(x, dur_maybe) => {
                if let Some(dur) = dur_maybe {
                    if t >= 0.0 && t <= dur { Some(x) } else { None }
                } else {
                    Some(x)
                }
            }
            &Flow::Linear(start, slope, dur) => {
                if t >= 0.0 && t <= dur {
                    Some(start + slope * t)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum Shape {
    Sine,
    Saw,
    Square,
}
impl Shape {
    pub fn val(&self, amp: Amp, fq: Frequency, time: Time) -> f32 {
        match self {
            &Shape::Sine => (amp * (time * fq * TAU).sin()) as f32,
            &Shape::Saw => (2.0 * amp * ((0.5 + time) * fq).fract() - 1.0) as f32,
            &Shape::Square => {
                if (time * fq).fract() < 0.5 {
                    amp as f32
                } else {
                    -1.0 * amp as f32
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct TimeAdjust(Option<Time>);

impl TimeAdjust {
    pub fn new() -> Self {
        TimeAdjust(None)
    }
    pub fn dt(&mut self, t: Time) -> Time {
        if let Some(start) = self.0 {
            return t - start;
        }
        self.0 = Some(t);
        0.0
    }
}
