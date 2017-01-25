use wave::{Wave, Time};

#[derive(Clone)]
pub struct WaveChain<T: Wave, K: Wave> {
    first: Option<T>,
    second: K,
}

impl<T: Wave, K: Wave> WaveChain<T, K> {
    pub fn new(f: T, s: K) -> Self {
        WaveChain {
            first: Some(f),
            second: s,
        }
    }
    pub fn chain<N: Wave>(self, n: N) -> WaveChain<WaveChain<T, K>, N> {
        WaveChain {
            first: Some(self),
            second: n,
        }
    }
}

impl<T: Wave, K: Wave> Wave for WaveChain<T, K> {
    fn val(&mut self, time: Time) -> Option<f32> {
        let mut val = if let Some(ref mut first) = self.first {
            first.val(time)
        } else {
            return self.second.val(time);
        };
        if val.is_none() {
            self.first = None;
            val = self.second.val(time);
        }
        val
    }
}

#[derive(Clone)]
pub struct WaveMixer<T: Wave, K: Wave> {
    first_amp: f32,
    w1: T,
    w2: K,
}

impl<T: Wave, K: Wave> WaveMixer<T, K> {
    pub fn new(w1: T, first_amp: f32, w2: K) -> Self {
        WaveMixer {
            first_amp: first_amp,
            w1: w1,
            w2: w2,
        }
    }
}

impl<T: Wave, K: Wave> Wave for WaveMixer<T, K> {
    fn val(&mut self, time: Time) -> Option<f32> {
        self.w1
            .val(time)
            .and_then(|x| {
                self.w2
                    .val(time)
                    .and_then(|y| Some(self.first_amp * x + (1.0 - self.first_amp) * y))
            })
    }
}
