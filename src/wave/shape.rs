use super::*;

#[derive(Clone)]
pub struct Waver {
    pub timer: TimeAdjust,
    pub behavior: Behavior,
}

impl Waver {
    pub fn new_silence(dur: Time) -> Waver {
        Waver {
            timer: TimeAdjust::new(),
            behavior: Behavior::Silence(Silence::new(dur)),
        }
    }
    pub fn new_noise(noise: Noise) -> Waver {
        Waver {
            timer: TimeAdjust::new(),
            behavior: Behavior::Noise(noise),
        }
    }
    pub fn modify(&mut self, t: Time, new_noise: Noise) {

        let dt = self.timer.dt(t);
    }
}

impl Wave for Waver {
    fn val(&mut self, t: Time) -> Option<f32> {
        let dt = self.timer.dt(t);
        match self.behavior {
            Behavior::Noise(ref mut x) => x.val(dt),
            Behavior::Silence(ref mut x) => x.val(dt),
        }
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone)]
pub enum Behavior {
    Noise(Noise),
    Silence(Silence),
}

#[derive(Clone)]
pub struct Noise {
    pub shape: Shape,
    pub stats: Vec<(f64, f64, f64)>, // amp, fq, t_end
    pub current_stats: usize,
}

impl Noise {
    pub fn new(shape: Shape, fq: f64) -> Self {
        Noise {
            shape: shape,
            stats: vec![(0.0, fq, 0.0)],
            current_stats: 0,
        }
    }
    pub fn sine(fq: f64) -> Self {
        Noise::new(Shape::Sine, fq)
    }
    pub fn square(fq: f64) -> Self {
        Noise::new(Shape::Square, fq)
    }
    pub fn saw(fq: f64) -> Self {
        Noise::new(Shape::Saw, fq)
    }
    pub fn push_stats(&mut self, amp: f64, fq: f64, dur: f64) {
        self.stats.push((amp, fq, dur));
    }
    pub fn reset(&mut self) {
        self.current_stats = 0;
    }
    pub fn dur(&self) -> f64 {
        if let Some(d) = self.stats.last() {
            d.2
        } else {
            0.0
        }
    }
    // This will be called mid-play, so new_stats must pre-allocate the first
    // spot, and the second spot's time must be transition time.
    pub fn swap(&mut self, now: Time, mut new_noise: Noise) {
        if let Some(stats) = self.stats.get(self.current_stats) {
            if new_stats.len() > 0 {
                new_stats[0] = stats.clone();
                new_stats[0].2 -= now;
            }
        } else if new_stats.len() > 1 {
            new_stats[0] = new_stats[1].clone();
        }
        for stat in &mut new_stats {
            stat.2 += now;
        }
        self.stats = new_stats;
    }
}

impl Wave for Noise {
    fn val(&mut self, dt: Time) -> Option<f32> {
        let (amp, fq) = if let Some(&(amp1, fq1, dur1)) = self.stats.get(self.current_stats) {
            if let Some(&(amp2, fq2, dur2)) = self.stats.get(self.current_stats.wrapping_add(1)) {
                if dt > dur2 {
                    self.current_stats += 1;
                    return self.val(dt);
                }
                let prog = (dt - dur1) / (dur2 - dur1);
                let amp = amp1 + (amp2 - amp1) * prog;
                let fq = fq1 + (fq2 - fq1) * prog;
                (amp, fq)
            } else {
                return None;
            }
        } else {
            return None;
        };
        match self.shape {
            x @ Shape::Sine => Some(x.val(amp, fq, dt)),
            x @ Shape::Square => Some(x.val(amp, fq, dt)),
            x @ Shape::Saw => Some(x.val(amp, fq, dt)),
        }
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Silence {
    pub duration: Time,
}

impl Silence {
    pub fn new(dur: Time) -> Self {
        Silence { duration: dur }
    }
}


impl Wave for Silence {
    fn val(&mut self, dt: Time) -> Option<f32> {
        if dt < 0.0 || dt > self.duration {
            None
        } else {
            Some(0.0)
        }
    }
}
