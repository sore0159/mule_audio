use portaudio as pa;
use streamer::Streamer;
use wave::{Mix, VoiceBuilder};

pub fn trial1() -> Result<(), pa::Error> {
    let mut streamer = Streamer::new()?;
    let mut mix = Mix::new(6);
    let vol: f64 = 0.45;
    let note_dur: f64 = 1.0;
    let grow_dur = 0.1;
    let fade_dur = 0.1;
    let play_note = |v: VoiceBuilder, dur: f64| {
        v.linear_amp(grow_dur * 0.75, 2.0 * vol)
            .linear_amp(grow_dur * 0.25, vol)
            .hold(dur - grow_dur - fade_dur)
            .fade(fade_dur)
        // v.linear_amp(grow_dur, vol).hold(dur - grow_dur - fade_dur).fade(fade_dur)
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

    let mut v1b = VoiceBuilder::sine(430.0);
    v1b = play_note(v1b, note_dur).silence(note_dur * 2.0);
    v1b = play_note(v1b, note_dur * 2.0);
    let mut v2b = VoiceBuilder::sine(545.0).silence(note_dur);
    v2b = play_note(v2b, note_dur).silence(note_dur);
    v2b = play_note(v2b, note_dur * 2.0);
    let mut v3b = VoiceBuilder::sine(665.0).silence(note_dur * 2.0);
    v3b = play_note(v3b, note_dur);
    v3b = play_note(v3b, note_dur * 2.0);

    mix.add_voice(v1.into());
    mix.add_voice(v2.into());
    mix.add_voice(v3.into());
    mix.add_voice(v1b.into());
    mix.add_voice(v2b.into());
    mix.add_voice(v3b.into());

    streamer.set_stream(mix)?;
    streamer.play_till_done()
}
