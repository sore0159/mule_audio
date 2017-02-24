extern crate mule_audio as ma;
extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

use ma::{Voice, notes, VoiceState, VoiceBuilder, Noise, SafeVoice, SafeMix, SafeState, Streamer};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut streamer = Streamer::new().unwrap();
    let (mix, v_send, mod_send, done_send) = SafeMix::new(4);
    let note1 = notes::A4;
    let note2 = notes::CS5;
    let note3 = notes::E5;
    let note4 = notes::CS4;
    let low_vol = 0.25;
    let high_vol = 0.5;
    let v1: Voice = VoiceBuilder::sine(note1)
        .linear_amp(0.1, high_vol)
        .linear_amp(0.1, low_vol)
        .hold(5.0)
        .fade(0.1)
        .into();
    let v2: Voice = VoiceBuilder::sine(note2)
        .linear_amp(0.1, high_vol)
        .linear_amp(0.1, low_vol)
        .hold(5.0)
        .fade(0.1)
        .into();
    let v3: Voice = VoiceBuilder::sine(note3).linear_amp(0.1, low_vol).hold(5.0).fade(0.1).into();
    let v4: Voice = VoiceBuilder::sine(note4).linear_amp(0.1, high_vol).hold(5.0).fade(0.1).into();
    let mut v1_stop = Noise::sine(note1);
    v1_stop.push_stats(0.0, note1, 0.20);
    let mut v2_stop = Noise::sine(note2);
    v2_stop.push_stats(0.0, note2, 0.20);
    let mut v3_stop = Noise::sine(note3);
    v3_stop.push_stats(0.0, note3, 0.05);
    let mut v4_stop = Noise::sine(note4);
    v4_stop.push_stats(0.0, note4, 0.05);
    streamer.set_stream(mix).unwrap();
    streamer.start().unwrap();

    let (mut a_maybe, mut o_maybe, mut e_maybe, mut u_maybe): (Option<SafeState>,
                                                               Option<SafeState>,
                                                               Option<SafeState>,
                                                               Option<SafeState>) =
        (None, None, None, None);

    write!(stdout,
           "{}{}q to exit. Type stuff, use alt, and so on.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
        .unwrap();
    stdout.flush().unwrap();
    let check_voice =
        |key: char, mut id_maybe: &mut Option<SafeState>, start_v: &Voice, stop_v: &mut Noise| {
            print!("Checking voice for {}", key);
            let mut start_flag = true;
            let mut stop_id: Option<u64> = None;
            if let &mut Some(ref mut ss_a) = id_maybe {
                print!("  voice active: attempting state read");
                if let Ok(r) = ss_a.0.read() {
                    match *r {
                        VoiceState::Active(x) => {
                            stop_id = Some(x);
                            start_flag = false;
                        }
                        VoiceState::Failed => {
                            print!("  VoiceState failed!");
                        }
                        VoiceState::Done => {
                            print!("  VoiceState Done!");
                        }
                        VoiceState::Pending => {
                            print!("  VoiceState Pending!");
                            start_flag = false;
                        }
                    };
                } else {
                    print!("  read failed!");
                    start_flag = false;
                }
            }
            if let Some(id) = stop_id {
                print!("  Id {} found: Stopping voice!", id);
                mod_send.send((stop_v.clone(), id)).unwrap();
            } else if start_flag {
                print!("  Starting new voice!");
                let safe_v: SafeVoice = start_v.clone().into();
                *id_maybe = Some(safe_v.1.clone());
                v_send.send(safe_v).unwrap();
            }
        };

    for c in stdin.keys() {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1, 1),
               termion::clear::CurrentLine)
            .unwrap();

        match c.unwrap() {
            Key::Char('q') => {
                done_send.send(()).unwrap();
                break;
            }
            Key::Char('a') => {
                check_voice('a', &mut a_maybe, &v1, &mut v1_stop);
            }
            Key::Char('o') => {
                check_voice('o', &mut o_maybe, &v2, &mut v2_stop);
            }
            Key::Char('e') => {
                check_voice('e', &mut e_maybe, &v3, &mut v3_stop);
            }
            Key::Char('u') => {
                check_voice('u', &mut u_maybe, &v4, &mut v4_stop);
            }
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        }
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
