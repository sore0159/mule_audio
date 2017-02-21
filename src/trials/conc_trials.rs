use portaudio as pa;
use streamer::Streamer;
use wave::{Shape, SafeMix, VoiceBuilder, VoiceState, SafeVoice};
use wave::shape::Noise;
use notes;
use std::{time, thread};

pub fn c_trial1() -> Result<(), pa::Error> {
    let mut streamer = Streamer::new()?;
    let (mix, v_send, mod_send, done_send) = SafeMix::new(6);
    let v1 = VoiceBuilder::sine(notes::A4).linear_amp(0.1, 0.5).hold(2.0).fade(0.1);
    let v2 = VoiceBuilder::sine(notes::CS4).linear_amp(0.1, 0.5).hold(2.0).fade(0.1);
    let mut stop_noise = Noise::new(Shape::Sine, notes::A4);
    stop_noise.push_stats(0.0, notes::A4, 0.05);
    streamer.set_stream(mix)?;
    streamer.start()?;
    println!("Starting stream");
    thread::sleep(time::Duration::from_millis(500));
    let safe_v: SafeVoice = v1.into();
    let v_state = safe_v.1.clone();
    println!("Sending voice");
    v_send.send(safe_v).unwrap();
    let mut tic = 0;
    loop {
        if let Ok(r) = v_state.read() {
            match *r {
                VoiceState::Active(x) => {
                    tic += 1;
                    println!("VoiceActive: id {}", x);
                    if tic == 4 {
                        println!("Starting voice 2!");
                        v_send.send(v2.clone().into()).unwrap();
                    } else if tic == 8 {
                        println!("Sending switch signal for id {}", x);
                        mod_send.send((stop_noise.clone(), x)).unwrap();
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
        }
        thread::sleep(time::Duration::from_millis(100));
    }
    thread::sleep(time::Duration::from_millis(10000));
    println!("Sending done signal!");
    done_send.send(()).unwrap();
    Ok(())
}
