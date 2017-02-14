extern crate mule_audio as ma;
use ma::mocks;

fn main() {
    println!("HELLO WORLD!");
    mocks::sine_chord().unwrap();
    mocks::saw_chord().unwrap();
}
