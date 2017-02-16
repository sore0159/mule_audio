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


#[derive(Clone)]
pub struct Mix {
    id_count: u64,
    capacity: f32,
    data: Vec<Option<Voice>>,
    ids: Vec<Option<u64>>,
}

impl Mix {
    pub fn new(capacity: usize) -> Self {
        Mix {
            id_count: 0,
            capacity: capacity as f32,
            data: vec![None;capacity],
            ids: vec![None;capacity],
        }
    }
    pub fn add_voice(&mut self, voice: Voice) -> Option<u64> {
        for (w_maybe, id) in self.data.iter_mut().zip(self.ids.iter_mut()) {
            if w_maybe.is_some() {
                continue;
            }
            self.id_count = self.id_count.wrapping_add(1);
            *w_maybe = Some(voice);
            *id = Some(self.id_count);
            return Some(self.id_count);
        }
        None
    }
    pub fn stop_voice(&mut self, id: u64) -> Option<Voice> {
        for (w_maybe, id_maybe) in self.data.iter_mut().zip(self.ids.iter_mut()) {
            if w_maybe.is_none() {
                continue;
            }
            if let &mut Some(test_id) = id_maybe {
                if test_id != id {
                    continue;
                }
            } else {
                continue;
            }
            *id_maybe = None;
            return w_maybe.take();
        }
        None
    }
}

impl Wave for Mix {
    fn val(&mut self, dt: Time) -> Option<f32> {
        let mut sum = 0.0;
        let mut any = false;
        for (w_maybe, id) in self.data.iter_mut().zip(self.ids.iter_mut()) {
            let mut flag = false;
            if let &mut Some(ref mut w) = w_maybe {
                if let Some(x) = w.val(dt) {
                    any = true;
                    sum += x;
                } else {
                    flag = true;
                }
            }
            if flag {
                *w_maybe = None;
                *id = None;
            }
        }
        if any { Some(sum / self.capacity) } else { None }
    }
}
