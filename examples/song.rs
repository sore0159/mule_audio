extern crate mule_audio as ma;
extern crate portaudio as pa;

pub mod support;
use support::{Voice, VoiceBuilder, notes, play_voices};

fn main() {
    println!("Mary had a little lamb...");
    play_voices(song_voices()).unwrap();
}

pub fn song_voices() -> Vec<Voice> {
    use notes::*;
    use ma::wave::Shape;
    let song = [E4, D4, C4, D4, E4, E4, E4, D4, D4, D4, E4, G4, G4, E4, D4, C4, D4, E4, E4, E4,
                E4, D4, D4, E4, D4, C4];
    let vol: f64 = 0.45;
    let note_dur: f64 = 0.66;
    let silence_dur: f64 = note_dur * 0.85;
    let grow_dur = 0.1;
    let fade_dur = 0.1;
    let play_note = |mut v: VoiceBuilder, n: f64| if n < 0.0 {
        v.silence(silence_dur - n)
    } else {
        v = v.silence(silence_dur);
        v.next_noise(Shape::Sine, n);
        v.linear_amp(0.75 * grow_dur, 1.5 * vol)
        .linear_amp(0.25 * grow_dur, vol)
        //.linear_amp(grow_dur, vol)
            .hold(note_dur - grow_dur - fade_dur)
            .fade(fade_dur)
    };

    let mut v1 = VoiceBuilder::sine(song[0]);
    let mut v2 = VoiceBuilder::sine(song[1]).silence(silence_dur);
    for (i, n) in song.into_iter().enumerate() {
        if i % 2 == 0 {
            v1 = play_note(v1, *n);
        } else {
            v2 = play_note(v2, *n);
        }
        match i {
            6 | 9 | 12 => {
                v1 = v1.silence(silence_dur * 1.0);
                v2 = v2.silence(silence_dur * 1.0);
            }
            19 | 26 => {
                v1 = v1.silence(silence_dur * 0.25);
                v2 = v2.silence(silence_dur * 0.25);
            }
            _ => {}
        }
    }
    vec![v1.into(), v2.into()]
}

/*
pub fn wierd_song_voices() -> Vec<Voice> {
    use notes::*;
    let song = [E4, D4, C4, D4, E4, E4, E4, D4, D4, D4, E4, G4, G4, E4, D4, C4, D4, E4, E4, E4,
                E4, D4, D4, E4, D4, C4];
    let vol: f64 = 0.45;
    let note_dur: f64 = 0.850;
    let silence_dur: f64 = note_dur * 0.85;
    let grow_dur = 0.1;
    let fade_dur = 0.1;
    let play_note = |mut v: VoiceBuilder, n: f64| if n < 0.0 {
        v.silence(silence_dur - n)
    } else {
        v.cur_fq = n;
        v.linear_amp(grow_dur, vol)
            .hold(note_dur - grow_dur - fade_dur)
            .fade(fade_dur)
            .silence(silence_dur)
    };

    let mut v1 = VoiceBuilder::sine(song[0]);
    let mut v2 = VoiceBuilder::sine(song[1]).silence(silence_dur);
    for (i, n) in song.into_iter().enumerate() {
        if i % 2 == 0 {
            v1 = play_note(v1, *n);
        } else {
            v2 = play_note(v2, *n);
        }
        match i {
            6 | 12 => {
                v1 = v1.silence(silence_dur * 0.5);
                v2 = v2.silence(silence_dur * 0.5);
            }
            9 | 19 => {
                v1 = v1.silence(silence_dur * 0.25);
                v2 = v2.silence(silence_dur * 0.25);
            }
            _ => {}
        }
    }
    vec![v1.into(), v2.into()]
}
*/
