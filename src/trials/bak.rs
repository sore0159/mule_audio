use portaudio;
use streamer::Streamer;
use wave::{Wave, sine, saw, square, silence};

pub fn sine_chord() -> Result<(), portaudio::Error> {
    let mut s = Streamer::new().unwrap();
    let amp = 0.2;
    let w1 = sine(440.0).amp(amp * 0.33).dur(1.0);
    let m1 = sine(440.0).amp(amp).dur(1.0);
    let w2 = sine(550.0).amp(amp).dur(1.0);
    let m2 = sine(550.0).amp(amp).dur(1.0);
    let w3 = sine(660.0).amp(amp).dur(1.0);
    let m3 = sine(660.0).amp(amp).dur(1.0);
    let mix = m1.clone().mix(0.5, m2.clone()).mix(0.66, m3.clone());

    let sl = silence(1.0);
    s.set_stream(w1.clone()
            .chain(w2.clone())
            .chain(w3.clone())
            .chain(mix)
            .chain(sl.clone()))?;
    s.play_till_done()
}

pub fn saw_chord() -> Result<(), portaudio::Error> {
    let mut s = Streamer::new().unwrap();
    let _s = square(440.0).amp(0.2).dur(0.15);
    let sq4 = saw(440.0).amp(0.2).dur(0.15);
    let sq4m = saw(440.0).amp(0.2).dur(0.15);
    let sq5 = saw(550.0).amp(0.2).dur(0.15);
    let sq5m = saw(550.0).amp(0.2).dur(0.15);
    let sq6 = saw(660.0).amp(0.2).dur(0.15);
    let sq6m = saw(660.0).amp(0.2).dur(0.15);
    let sl = silence(0.1);
    let mix = sq4m.mix(0.5, sq5m).mix(0.66, sq6m);
    // let up = square(440.0).linear_amp(0.0, 1.0).dur(0.25);

    s.set_stream(sq4.chain(sl.clone())
            .chain(sq5)
            .chain(sl.clone())
            .chain(sq6)
            .chain(sl.clone())
            .chain(mix))?;
    s.play_till_done()
}


pub fn square_chord() -> Result<(), portaudio::Error> {
    let mut s = Streamer::new().unwrap();
    let _s = square(440.0).amp(0.3).dur(0.15);
    let sq4 = square(440.0).amp(0.3).dur(0.15);
    let sq4m = square(440.0).amp(1.0).dur(0.15);
    let sq5 = square(550.0).amp(0.3).dur(0.15);
    let sq5m = square(550.0).amp(1.0).dur(0.15);
    let sq6 = square(660.0).amp(0.3).dur(0.15);
    let sq6m = square(660.0).amp(1.0).dur(0.15);
    let sl = silence(0.1);
    let mix = sq4m.mix(0.5, sq5m).mix(0.66, sq6m);
    // let up = square(440.0).linear_amp(0.0, 1.0).dur(0.25);

    s.set_stream(sq4.chain(sl.clone())
            .chain(sq5)
            .chain(sl.clone())
            .chain(sq6)
            .chain(sl.clone())
            .chain(mix))?;
    s.play_till_done()
}
