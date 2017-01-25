use std;

mod tools;
mod makers;
pub mod shape;

use self::tools::{WaveChain, WaveMixer};
pub use self::shape::{Func, Silence, Flow, Shape};
pub use self::makers::{sine, square, saw, silence};

pub type Frequency = f64;
pub type Amp = f64;
pub type Time = f64;

const TAU: f64 = std::f64::consts::PI * 2.0;

pub trait Wave: Sized {
    fn val(&mut self, time: Time) -> Option<f32>;
    fn chain<N: Wave>(self, n: N) -> WaveChain<Self, N> {
        WaveChain::new(self, n)
    }
    fn mix<N: Wave>(self, first_amp: f32, n: N) -> WaveMixer<Self, N> {
        WaveMixer::new(self, first_amp, n)
    }
}
