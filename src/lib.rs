extern crate portaudio;

pub mod wave;
pub mod streamer;
pub mod trials;
pub mod notes;

pub use streamer::Streamer;
pub use wave::{Wave, Mix, VoiceBuilder};


#[cfg(test)]
pub mod tests {
    #[test]
    fn it_works() {}
}
