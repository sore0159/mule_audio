use std;
use std::iter::Cycle;
use std::ops::Range;

use portaudio as pa;
use wave::{Wave, Time};

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const SAMPLE_RATE_I: usize = 44_100;
const FRAMES_PER_BUFFER: u32 = 64;
pub const DT: f64 = 1.0 / SAMPLE_RATE;

pub type Stream = pa::Stream<pa::NonBlocking, pa::Output<f32>>;
pub struct Streamer {
    pub pa: pa::PortAudio,
    pub current_stream: Option<Stream>,
}

impl Streamer {
    pub fn new() -> Result<Self, pa::Error> {
        let pa = pa::PortAudio::new()?;
        Ok(Streamer {
            pa: pa,
            current_stream: None,
        })
    }
    pub fn start(&mut self) -> Result<(), pa::Error> {
        if let Some(ref mut x) = self.current_stream {
            return x.start();
        }
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), pa::Error> {
        if let Some(ref mut x) = self.current_stream {
            return x.stop();
        }
        Ok(())
    }
    pub fn close(&mut self) -> Result<(), pa::Error> {
        let mut r = Ok(());
        if let Some(ref mut x) = self.current_stream {
            r = x.close();
        }
        self.current_stream = None;
        r
    }
    pub fn play_till_done(&mut self) -> Result<(), pa::Error> {
        self.start()?;
        self.till_done()?;
        self.close()
    }
    pub fn play_for(&mut self, millis: u64) -> Result<(), pa::Error> {
        self.start()?;
        std::thread::sleep(std::time::Duration::from_millis(millis));
        self.close()
    }

    pub fn is_active(&self) -> Result<bool, pa::Error> {
        if let Some(ref x) = self.current_stream {
            x.is_active()
        } else {
            Ok(false)
        }
    }
    pub fn time(&self) -> f64 {
        if let Some(ref x) = self.current_stream {
            x.time() as f64
        } else {
            0.0
        }
    }
    pub fn till_done(&self) -> Result<(), pa::Error> {
        while self.is_active()? {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        Ok(())
    }

    pub fn set_stream<T: 'static + Wave>(&mut self, mut data: T) -> Result<(), pa::Error> {
        let mut settings = try!(self.pa
            .default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));
        // we won't output out of range samples so don't bother clipping them.
        settings.flags = pa::stream_flags::CLIP_OFF;

        // This routine will be called by the PortAudio engine when audio is needed. It may called at
        // interrupt level on some machines so don't do anything that could mess up the system like
        // dynamic resource allocation or IO.
        let mut timer = Timer::new();
        let mut done = false;
        let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
            let mut idx = 0;
            for _ in 0..frames {
                if !done {
                    match data.val(timer.tic()) {
                        Some(x) => {
                            buffer[idx] = x;
                            buffer[idx + 1] = x;
                            idx += 2;
                        }
                        None => done = true,
                    }
                }
                if done {
                    buffer[idx] = 0.0;
                    buffer[idx + 1] = 0.0;
                    idx += 2;
                }

            }
            if done { pa::Complete } else { pa::Continue }
        };

        let stream = self.pa.open_non_blocking_stream(settings, callback)?;
        self.current_stream = Some(stream);
        Ok(())
    }
}


pub struct Timer {
    i: Cycle<Range<usize>>,
    seconds: f64,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            seconds: -1.0,
            i: (0..SAMPLE_RATE_I).cycle(),
        }
    }
    pub fn tic(&mut self) -> Time {
        let x_maybe = self.i.next();
        if let Some(x) = x_maybe {
            if x == 0 {
                self.seconds += 1.0;
            }
            (x as f64 / SAMPLE_RATE) + self.seconds
        } else {
            0.0
        }
    }
}
