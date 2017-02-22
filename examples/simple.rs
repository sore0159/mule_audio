extern crate mule_audio as ma;
extern crate portaudio as pa;

pub mod support;
use support::{Voice, VoiceBuilder, notes, play_voices};

fn main() {
    println!("Simple voice trials!");
    play_voices(trial1_voices()).unwrap();
    play_voices(trial2_voices()).unwrap();
}

pub fn trial1_voices() -> Vec<Voice> {
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

    let mut v1 = VoiceBuilder::sine(440.0);
    v1 = play_note(v1, note_dur).silence(note_dur * 2.0);
    v1 = play_note(v1, note_dur * 2.0);
    let mut v2 = VoiceBuilder::sine(550.0).silence(note_dur);
    v2 = play_note(v2, note_dur).silence(note_dur);
    v2 = play_note(v2, note_dur * 2.0);
    let mut v3 = VoiceBuilder::sine(660.0).silence(note_dur * 2.0);
    v3 = play_note(v3, note_dur);
    v3 = play_note(v3, note_dur * 2.0);

    let mut v1b = VoiceBuilder::sine(439.0);
    v1b = play_note(v1b, note_dur).silence(note_dur * 2.0);
    v1b = play_note(v1b, note_dur * 2.0);
    let mut v2b = VoiceBuilder::sine(549.0).silence(note_dur);
    v2b = play_note(v2b, note_dur).silence(note_dur);
    v2b = play_note(v2b, note_dur * 2.0);
    let mut v3b = VoiceBuilder::sine(661.0).silence(note_dur * 2.0);
    v3b = play_note(v3b, note_dur);
    v3b = play_note(v3b, note_dur * 2.0);
    vec![v1.into(), v2.into(), v3.into(), v1b.into(), v2b.into(), v3b.into()]
}


pub fn trial2_voices() -> Vec<Voice> {
    let vol: f64 = 0.95;
    // let echo_vol = vol * 0.5;
    // let note_dur: f64 = 1.0;

    let v1 = VoiceBuilder::sine(notes::C5).linear_amp(0.1, vol).hold(3.0).fade(0.1);
    let v2 = VoiceBuilder::sine(notes::F4).linear_amp(0.1, vol).hold(3.0).fade(0.1);
    let v3 = VoiceBuilder::sine(notes::E3).linear_amp(0.1, vol).hold(3.0).fade(0.1);
    vec![v1.into(), v2.into(), v3.into()]
}
