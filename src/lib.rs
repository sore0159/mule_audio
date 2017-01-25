extern crate portaudio;

pub mod wave;
pub mod player;
pub mod streamer;


#[cfg(test)]
pub mod tests {
    #[test]
    fn it_works() {}
    #[test]
    fn a_sharp() {
        super::a_sharp()
    }
}

pub fn a_sharp() -> Result<(), portaudio::Error> {
    use streamer::Streamer;
    use wave::{Wave, sine, square, silence};
    let mut s = Streamer::new().unwrap();
    let wv = sine(440.0).dur(1.0);
    let wv2 = sine(550.0).dur(1.0);
    let wv3 = sine(660.0).dur(1.0);

    let mix = wv.clone().mix(0.5, wv2.clone()).mix(0.66, wv3.clone());
    let sl = silence(1.0);
    s.set_stream(wv.clone()
            .chain(wv2.clone())
            .chain(wv3.clone())
            .chain(mix)
            .chain(sl.clone()))?;

    //
    s.play_till_done()?;
    let sq = square(440.0).amp(0.5).dur(0.15);
    let sq2 = square(550.0).amp(0.5).dur(0.15);
    let sq3 = square(660.0).amp(0.5).dur(0.15);
    let sl = silence(0.1);
    let mix = sq.clone().mix(0.5, sq2.clone()).mix(0.66, sq3.clone());
    // let down = square(440.0).linear_amp(1.0, 0.0).dur(0.25);
    // let up = square(440.0).linear_amp(0.0, 1.0).dur(0.25);

    s.set_stream(sq.chain(sl.clone())
            .chain(sq2)
            .chain(sl.clone())
            .chain(sq3)
            .chain(sl.clone())
            .chain(mix))?;
    s.play_till_done()
}
