use pa;
use std::{time, thread};

pub use ma::{Mix, Streamer, notes};
pub use ma::{VoiceBuilder, Voice};
pub use ma::{Noise, SafeMix, VoiceState, SafeVoice};

pub fn play_voices(vs: Vec<Voice>) -> Result<(), pa::Error> {
    let mut streamer = Streamer::new()?;
    let mut mix = Mix::new(vs.len());
    for v in vs {
        mix.add_voice(v);
    }
    streamer.set_stream(mix)?;
    streamer.play_till_done()?;
    Ok(())
}

// Have to return the Streamer otherwise it gets dropped and
// playback stops.  Hold on to the result in a _stream var or something
pub fn play_safemixer(m: SafeMix) -> Result<Streamer, pa::Error> {
    let mut streamer = Streamer::new()?;
    streamer.set_stream(m)?;
    streamer.start()?;
    Ok(streamer)
}

pub fn sleep(dur: u64) {
    thread::sleep(time::Duration::from_millis(dur));
}
