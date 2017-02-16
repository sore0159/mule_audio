use std;

pub mod shape;
pub mod voices;
pub mod builders;

pub use self::builders::VoiceBuilder;
pub use self::shape::{Waver, Silence, Shape};
pub use self::voices::{Voice, Mix};

pub type Frequency = f64;
pub type Amp = f64;
pub type Time = f64;

const TAU: f64 = std::f64::consts::PI * 2.0;

pub trait Wave: Sized {
    fn val(&mut self, time: Time) -> Option<f32>;
}
