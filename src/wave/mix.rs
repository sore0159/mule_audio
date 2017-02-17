
use wave::{Wave, Time, Voice, VoiceBuilder};

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


use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{RwLock, Arc};
use std::thread;

pub struct SafeMix {
    capacity: f32,
    data: Vec<Option<SafeVoice>>,
    voice_rc: Receiver<SafeVoice>,
    state_modder: Sender<(Arc<RwLock<VoiceState>>, VoiceState)>,
    terminator: Receiver<()>,
}
pub enum VoiceState {
    Pending,
    Failed,
    Active(u64),
    Done,
}


pub struct SafeVoice(Voice, pub Arc<RwLock<VoiceState>>);

impl From<Voice> for SafeVoice {
    fn from(item: Voice) -> SafeVoice {
        SafeVoice(item, Arc::new(RwLock::new(VoiceState::Pending)))
    }
}
impl From<VoiceBuilder> for SafeVoice {
    fn from(item: VoiceBuilder) -> SafeVoice {
        SafeVoice(item.into(), Arc::new(RwLock::new(VoiceState::Pending)))
    }
}

impl SafeMix {
    pub fn new(capacity: usize) -> (Self, Sender<SafeVoice>, Sender<()>) {
        let mut v = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            v.push(None);
        }
        let (voice_sd, voice_rc) = channel();
        let (done_sd, done_rc) = channel();
        let (state_sd, state_rc) = channel::<(Arc<RwLock<VoiceState>>, VoiceState)>();
        thread::spawn(move || {
            let mut id_count: u64 = 0;
            loop {
                if let Ok((data, state)) = state_rc.recv() {
                    if let Ok(mut w) = data.write() {
                        match state {
                            VoiceState::Active(_) => {
                                id_count += 1;
                                *w = VoiceState::Active(id_count);
                            }
                            _ => *w = state,
                        };
                    }
                } else {
                    return;
                }
            }
        });
        (SafeMix {
             capacity: capacity as f32,
             voice_rc: voice_rc,
             data: v,
             state_modder: state_sd,
             terminator: done_rc,
         },
         voice_sd,
         done_sd)
    }
}

impl Wave for SafeMix {
    fn val(&mut self, dt: Time) -> Option<f32> {
        let mut sum = 0.0;
        let mut needs_id: Option<Arc<RwLock<VoiceState>>> = None;
        if let Ok(_) = self.terminator.try_recv() {
            return None;
        }
        let mut new_voice = if let Ok(v) = self.voice_rc.try_recv() {
            Some(v)
        } else {
            None
        };
        for vd_maybe in &mut self.data {
            let mut flag = false;
            if let &mut Some(SafeVoice(ref mut v, _)) = vd_maybe {
                if let Some(x) = v.val(dt) {
                    sum += x;
                } else {
                    flag = true;
                }
            } else {
                if let Some(new_vd) = new_voice.take() {
                    needs_id = Some(new_vd.1.clone());
                    *vd_maybe = Some(new_vd);
                } else {
                    continue;
                }
            }
            if flag {
                if let Some(vd) = vd_maybe.take() {
                    self.state_modder.send((vd.1, VoiceState::Done)).unwrap();
                }
            }
        }
        if let Some(vs) = needs_id {
            self.state_modder.send((vs, VoiceState::Active(0))).unwrap();
        } else if let Some(new_vd) = new_voice {
            self.state_modder.send((new_vd.1, VoiceState::Failed)).unwrap();
        }
        Some(sum / self.capacity)
    }
}
