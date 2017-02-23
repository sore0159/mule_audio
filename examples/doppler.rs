extern crate mule_audio as ma;
extern crate portaudio as pa;

pub mod support;
use support::{Voice, VoiceBuilder, notes, play_voices};

fn main() {
    println!("Simple doppler trials!");
    play_voices(doppler_voices()).unwrap();
    play_voices(multi_doppler_voices()).unwrap();
}

// Also doesn't sound quite right...
pub fn doppler_voices() -> Vec<Voice> {
    let v1 = VoiceBuilder::sine(notes::CS3)
        .linear_amp(0.1, 0.5)
        .linear_fq(3.0, notes::C3)
        .linear_fq(3.0, notes::CS3)
        .linear_fq(3.0, notes::C3)
        .linear_fq(3.0, notes::CS3)
        .linear_fq(3.0, notes::C3)
        .fade(0.1);
    let v: Voice = v1.into();
    for w in &v.data {
        use ma::wave::shape::Behavior;
        if let Behavior::Noise(ref n) = w.behavior {
            println!("STATS:{:?}", n.stats);
        }
    }
    vec![v]
}

// Still not good!
pub fn multi_doppler_voices() -> Vec<Voice> {
    let mut voices: Vec<Voice> = vec![];
    for (i, n) in [notes::A3, notes::GS3, notes::G3, notes::FS3, notes::F3, notes::E3,
                   notes::DS3, notes::D3, notes::CS3, notes::C3]
        .into_iter()
        .enumerate() {
        let v1 = VoiceBuilder::sine(*n)
            .silence(i as f64 * 0.75)
            .linear_amp(0.1, 1.0)
            .hold(3.0)
            .fade(0.1);
        voices.push(v1.into());
    }
    voices
}
