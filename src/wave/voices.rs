use wave::shape::Waver;
use wave::{Wave, Time};

#[derive(Clone)]
pub struct Voice(Vec<Waver>);

impl From<Vec<Waver>> for Voice {
    fn from(mut item: Vec<Waver>) -> Voice {
        item.reverse();
        Voice(item)
    }
}

impl Wave for Voice {
    fn val(&mut self, dt: Time) -> Option<f32> {
        if let Some(w) = self.0.last_mut() {
            let v = w.val(dt);
            if v.is_some() {
                return v;
            }
        } else {
            return None;
        }
        self.0.pop();
        self.val(dt)
    }
}


#[derive(Clone)]
pub struct Mix(Vec<Voice>);

impl From<Vec<Voice>> for Mix {
    fn from(item: Vec<Voice>) -> Mix {
        Mix(item)
    }
}

impl Wave for Mix {
    fn val(&mut self, dt: Time) -> Option<f32> {
        let mut sum = 0.0;
        let mut any = false;
        for w in &mut self.0 {
            if let Some(x) = w.val(dt) {
                any = true;
                sum += x;
            }
        }
        if any {
            Some(sum / self.0.len() as f32)
        } else {
            None
        }
    }
}
