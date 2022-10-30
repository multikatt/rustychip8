#![allow(dead_code)]

use chip8::graphics::Graphics;
use chip8::chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    let mut c8 = Chip8::new();
    match c8.load_rom("roms/IBMLogo.ch8") {
        Ok(()) => {}
        Err(err) => panic!("{}", err),
    }

    let mut gfx = Graphics::new(true);
    gfx.clear();

    let mut event_pump = gfx.context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                _ => {}
            }
        }
        c8.decode();
        gfx.draw(&c8);

        gfx.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
