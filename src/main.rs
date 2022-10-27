#![allow(dead_code)]

use chip8::Chip8;

pub mod chip8;

fn main() {
    let mut c8 = Chip8::new();
    c8.load_rom("roms/test.ch8");
}
