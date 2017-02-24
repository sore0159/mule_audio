extern crate mule_audio as ma;
extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

use ma::{Voice, notes, VoiceBuilder, SafeMix, Streamer};
use ma::wave::Shape;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut streamer = Streamer::new().unwrap();
    let (mix, v_send, _mod_send, done_send) = SafeMix::new(15);
    let mut shape: Shape = Shape::Sine;
    let play_note = |shape: Shape, note: f64| {
        let v1: Voice = VoiceBuilder::new(shape, note)
            .linear_amp(0.05, 0.5)
            .linear_amp(0.05, 0.25)
            .hold(0.2)
            .fade(0.05)
            .into();
        v_send.send(v1.into()).unwrap();
    };
    streamer.set_stream(mix).unwrap();
    streamer.start().unwrap();

    write!(stdout,
           "{}{}q to exit. Type stuff, use alt, and so on.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
        .unwrap();
    stdout.flush().unwrap();
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
                play_note(shape, notes::C4);
            }
            Key::Char('o') => {
                play_note(shape, notes::CS4);
            }
            Key::Char('e') => {
                play_note(shape, notes::D4);
            }
            Key::Char('u') => {
                play_note(shape, notes::DS4);
            }
            Key::Char('i') => {
                play_note(shape, notes::E4);
            }
            Key::Char('d') => {
                play_note(shape, notes::F4);
            }
            Key::Char('h') => {
                play_note(shape, notes::FS4);
            }
            Key::Char('t') => {
                play_note(shape, notes::G4);
            }
            Key::Char('n') => {
                play_note(shape, notes::GS4);
            }
            Key::Char('\'') => {
                shape = Shape::Sine;
                println!("Sine wave!");
            }
            Key::Char(',') => {
                shape = Shape::Square;
                println!("Square wave!");
            }
            Key::Char('.') => {
                shape = Shape::Triangle;
                println!("Triangle wave!");
            }
            Key::Char('p') => {
                shape = Shape::Saw;
                println!("Saw wave!");
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
