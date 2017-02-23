extern crate mule_audio as ma;
extern crate portaudio as pa;

pub mod support;
use support::{Voice, VoiceBuilder, notes, play_voices};
use ma::wave::Waver;

fn main() {
    println!("Simple echo trials!");
    play_voices(echo_voices()).unwrap();
}

// not really sounding echo-ey yet
pub fn echo_voices() -> Vec<Voice> {
    let vol: f64 = 0.45;
    let note_dur: f64 = 1.0;
    let grow_dur = 0.2;
    let fade_dur = 0.1;
    let play_note = |v: VoiceBuilder, dur: f64| {
        v.linear_amp(grow_dur * 0.75, 2.0 * vol)
            .linear_amp(grow_dur * 0.25, vol)
            .hold(dur - grow_dur - fade_dur)
            .fade(fade_dur)
    };

    let mut v1 = VoiceBuilder::sine(notes::C4);
    v1 = play_note(v1, note_dur).silence(note_dur * 2.0);
    v1 = play_note(v1, note_dur * 2.0);
    v1 = v1.silence(1.0);
    let mut voices = vec![v1.clone().into()];
    for i in 0..4 {
        let mut v2: Voice = v1.clone().into();
        if i % 2 == 0 {
            v2.scale_amp(0.25);
        } else {
            v2.scale_amp(0.5);
        }

        v2.data.insert(0, Waver::new_silence(0.5 * i as f64));
        voices.push(v2);
    }
    voices
}
