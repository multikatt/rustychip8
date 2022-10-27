#![allow(dead_code)]

use chip8::Chip8;

pub mod chip8;

fn main() {
    let c8 = Chip8::new();
    c8.load_rom("testrom.ch8");
}
