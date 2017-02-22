extern crate portaudio;

pub mod wave;
pub mod streamer;
pub mod trials;
pub mod notes;

pub use streamer::Streamer;
pub use wave::{Wave, Mix, SafeMix, Voice, VoiceBuilder};
pub use wave::shape::Noise;
pub use wave::mix::{SafeVoice, VoiceState, SafeState};


#[cfg(test)]
pub mod tests {
    #[test]
    fn it_works() {}
}
