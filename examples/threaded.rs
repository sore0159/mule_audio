extern crate mule_audio as ma;
extern crate portaudio as pa;

pub mod support;
use support::{notes, play_safemixer, sleep};
use support::{VoiceBuilder, Noise};
use support::{SafeVoice, VoiceState, SafeMix};

use std::error::Error;

fn main() {
    println!("Threaded voice trials!");
    c_trial1().unwrap();
}

pub fn c_trial1() -> Result<(), Box<Error>> {
    let (mix, v_send, mod_send, done_send) = SafeMix::new(6);
    let v1 = VoiceBuilder::sine(notes::A4).linear_amp(0.1, 0.5).hold(2.0).fade(0.1);
    let v2 = VoiceBuilder::sine(notes::CS4).linear_amp(0.1, 0.5).hold(2.0).fade(0.1);
    let mut stop_noise = Noise::sine(notes::A4);
    stop_noise.push_stats(0.0, notes::A4, 0.05);

    println!("Starting stream");
    let _stream = play_safemixer(mix)?;

    sleep(500);
    let safe_v: SafeVoice = v1.into();
    let v_state = safe_v.1.clone();
    println!("Sending voice");
    v_send.send(safe_v)?;
    let mut tic = 0;
    loop {
        if let Ok(r) = v_state.0.read() {
            match *r {
                VoiceState::Active(x) => {
                    tic += 1;
                    println!("VoiceActive: id {}", x);
                    if tic == 4 {
                        println!("Starting voice 2!");
                        v_send.send(v2.clone().into())?;
                    } else if tic == 8 {
                        println!("Sending switch signal for id {}", x);
                        mod_send.send((stop_noise.clone(), x))?;
                        // stop_send.send(x).unwrap();
                    }
                }
                VoiceState::Failed => {
                    println!("VoiceState failed!");
                    break;
                }
                VoiceState::Done => {
                    println!("VoiceState Done!");
                    break;
                }
                _ => {}
            };
            sleep(100)
        }
    }
    sleep(10000);
    println!("Sending done signal!");
    done_send.send(())?;
    Ok(())
}
