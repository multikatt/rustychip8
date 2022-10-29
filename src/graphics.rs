use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::Sdl;

use crate::chip8::Chip8;

pub struct Graphics {
    pub canvas: Canvas<sdl2::video::Window>,
    pub context: Sdl,
    pixel_size: u32,
}

impl Graphics {
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let pixel_size = 10;

        let window = video_subsystem
            .window("Chip8 Emulator", 64 * pixel_size, 32 * pixel_size)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        Self { canvas, context, pixel_size }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn draw(&mut self, c8: &Chip8) {
        self.canvas.set_draw_color(Color::RGB(255, 128, 0));
        let mut i: u16 = 0;
        for p in &c8.display {
            if *p {
                let pos = c8.get_xy_from_pixel(i);
                let posx = pos.0 as u32;
                let posy = pos.1 as u32;

                let new_rect = Rect::new(
                    (posx * self.pixel_size) as i32,
                    (posy * self.pixel_size) as i32,
                    self.pixel_size,
                    self.pixel_size,
                );
                self.canvas.fill_rect(new_rect).unwrap();
            }
            i += 1;
        }

        self.canvas.present();
    }
}
