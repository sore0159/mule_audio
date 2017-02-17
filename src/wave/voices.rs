use wave::shape::Waver;
use wave::{Wave, Time};

#[derive(Clone)]
pub struct Voice {
    current: usize,
    data: Vec<Waver>,
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
