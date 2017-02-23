use wave::shape::{Noise, Waver};
use wave::{Wave, Time};

#[derive(Clone)]
pub struct Voice {
    pub current: usize,
    pub data: Vec<Waver>,
}

impl Voice {
    pub fn swap(&mut self, t: Time, new_noise: Noise) {
        if let Some(w) = self.data.get_mut(self.current) {
            w.swap(t, new_noise)
        }
        self.data.truncate(self.current + 1)
    }
    pub fn scale_amp(&mut self, s: f64) {
        for w in &mut self.data {
            w.scale_amp(s)
        }
    }
}
impl From<Vec<Waver>> for Voice {
    fn from(item: Vec<Waver>) -> Voice {
        Voice {
            current: 0,
            data: item,
        }
    }
}

impl Wave for Voice {
    fn val(&mut self, dt: Time) -> Option<f32> {
        if let Some(w) = self.data.get_mut(self.current) {
            let v = w.val(dt);
            if v.is_some() {
                return v;
            } else {
                self.current += 1;
            }
        } else {
            return None;
        }
        self.val(dt)
    }
}
