#![allow(dead_code)]

use chip8::chip8::Chip8;
use chip8::graphics::Graphics;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use std::env;
use std::time::Duration;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut c8 = Chip8::new();

    let mut filename = &String::from("roms/tank.ch8");

    let mut timer = Instant::now();

    if args.len() > 1 {
        filename = &args[1];
    }

    match c8.load_rom(&filename) {
        Ok(()) => {}
        Err(err) => panic!("{}", err),
    }

    let mut gfx = Graphics::new(true);
    gfx.clear();

    let mut event_pump = gfx.context.event_pump().unwrap();
    'running: loop {
        /*
         Chip-8 Key  Keyboard
         ----------  ---------
           1 2 3 C    1 2 3 4
           4 5 6 D    q w e r
           7 8 9 E    a s d f
           A 0 B F    z x c v
        */
        let ks = event_pump.keyboard_state();
        if ks.is_scancode_pressed(Scancode::X) {
            c8.key_pressed = Some(0x00)
        };
        if ks.is_scancode_pressed(Scancode::Num1) {
            c8.key_pressed = Some(0x01)
        };
        if ks.is_scancode_pressed(Scancode::Num2) {
            c8.key_pressed = Some(0x02)
        };
        if ks.is_scancode_pressed(Scancode::Num3) {
            c8.key_pressed = Some(0x03)
        };
        if ks.is_scancode_pressed(Scancode::Q) {
            c8.key_pressed = Some(0x04)
        };
        if ks.is_scancode_pressed(Scancode::W) {
            c8.key_pressed = Some(0x05)
        };
        if ks.is_scancode_pressed(Scancode::E) {
            c8.key_pressed = Some(0x06)
        };
        if ks.is_scancode_pressed(Scancode::A) {
            c8.key_pressed = Some(0x07)
        };
        if ks.is_scancode_pressed(Scancode::S) {
            c8.key_pressed = Some(0x08)
        };
        if ks.is_scancode_pressed(Scancode::D) {
            c8.key_pressed = Some(0x09)
        };
        if ks.is_scancode_pressed(Scancode::Z) {
            c8.key_pressed = Some(0x0A)
        };
        if ks.is_scancode_pressed(Scancode::C) {
            c8.key_pressed = Some(0x0B)
        };
        if ks.is_scancode_pressed(Scancode::Num4) {
            c8.key_pressed = Some(0x0C)
        };
        if ks.is_scancode_pressed(Scancode::R) {
            c8.key_pressed = Some(0x0D)
        };
        if ks.is_scancode_pressed(Scancode::F) {
            c8.key_pressed = Some(0x0E)
        };
        if ks.is_scancode_pressed(Scancode::V) {
            c8.key_pressed = Some(0x0F)
        };

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                Event::KeyUp { .. } => c8.key_pressed = None,
                _ => {}
            }
        }
        match c8.decode() {
            Ok(()) => {
                gfx.draw(&c8);

                gfx.canvas.present();
            }
            Err(()) => (),
        }
        ::std::thread::sleep(Duration::new(0, 100_000u32 / 60));
         if timer.elapsed() > Duration::new(0, 16_000_000) {
            if c8.delay_timer > 0x00 {
                c8.delay_timer -= 0x01;
            }
            timer = Instant::now();
        }
    }
}
