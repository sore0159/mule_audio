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
    pub fn swap(&mut self, t: Time, mut new_noise: Noise) {
        let dt = self.timer.dt(t);
        if let Behavior::Noise(ref mut x) = self.behavior {
            x.swap(dt, new_noise);
            return;
        }
        if new_noise.stats.len() > 1 {
            let mut st = new_noise.stats[1].clone();
            st.0 = 0.0;
            st.2 = 0.0;
            new_noise.stats[0] = st
        }
        for stat in &mut new_noise.stats {
            stat.2 += dt;
        }
        self.behavior = Behavior::Noise(new_noise);
    }
    pub fn scale_amp(&mut self, s: f64) {
        if let Behavior::Noise(ref mut n) = self.behavior {
            n.scale_amp(s);
        }
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

#[derive(Debug, Clone)]
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
        if let Some(snapshot) = self.calc_stats(now) {
            if new_noise.stats.len() > 0 {
                new_noise.stats[0] = (snapshot.0, snapshot.1, 0.0);
            }
        } else if new_noise.stats.len() > 1 {
            new_noise.stats[0] = new_noise.stats[1].clone();
            new_noise.stats[0].2 = 0.0;
        }
        for stat in &mut new_noise.stats {
            stat.2 += now;
        }
        *self = new_noise;
    }
    fn calc_stats(&mut self, dt: Time) -> Option<(f64, f64)> {
        if let Some(&(amp1, fq1, dur1)) = self.stats.get(self.current_stats) {
            if let Some(&(amp2, fq2, dur2)) = self.stats.get(self.current_stats.wrapping_add(1)) {
                if dt > dur2 {
                    self.current_stats += 1;
                    return self.calc_stats(dt);
                }
                let prog = (dt - dur1) / (dur2 - dur1);
                let amp = amp1 + (amp2 - amp1) * prog;
                let fq = fq1 + (fq2 - fq1) * prog;
                Some((amp, fq))
            } else {
                None
            }
        } else {
            None
        }

    }
    fn scale_amp(&mut self, s: f64) {
        for stat in &mut self.stats {
            stat.0 *= s;
        }
    }
}

impl Wave for Noise {
    fn val(&mut self, dt: Time) -> Option<f32> {
        if let Some((amp, fq)) = self.calc_stats(dt) {
            match self.shape {
                x @ Shape::Sine => Some(x.val(amp, fq, dt)),
                x @ Shape::Square => Some(x.val(amp, fq, dt)),
                x @ Shape::Saw => Some(x.val(amp, fq, dt)),
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Sine,
    Saw,
    Square,
}
impl Shape {
    pub fn val(&self, amp: Amp, fq: Frequency, time: Time) -> f32 {
        //println!("USING AMP {}, FQ {}", amp, fq);
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
