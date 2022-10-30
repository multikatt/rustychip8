use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::Sdl;

use crate::chip8::Chip8;


pub struct Graphics {
    pub canvas: Canvas<sdl2::video::Window>,
    pub context: Sdl,
    pixel_size: u32,
    debug_grid: bool,
}

impl Graphics {
    pub fn new(debug_grid: bool) -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let pixel_size = 10;

        let window = video_subsystem
            .window("Chip8 Emulator", 64 * pixel_size, 32 * pixel_size)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        Self { canvas, context, pixel_size, debug_grid }
    }

    fn default() -> Self {
        Self::new(false)
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 150, 150));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn draw(&mut self, c8: &Chip8) {
        self.canvas.set_draw_color(Color::RGB(255, 128, 0));

        for (i, p) in c8.display.iter().enumerate() {
            if *p {
                self.canvas.set_draw_color(Color::RGB(255, 128, 0));
            } else {
                self.canvas.set_draw_color(Color::RGB(0, 128, 128));
            }
            let pos = c8.get_xy_from_pixel(i as u16);
            let posx = pos.0 as u32;
            let posy = pos.1 as u32;

            let psize = {
                if self.debug_grid {
                    self.pixel_size - 1
                } else {
                    self.pixel_size
                }
            };

            let new_rect = Rect::new(
                (posx * self.pixel_size) as i32,
                (posy * self.pixel_size) as i32,
                psize,
                psize,
            );
            self.canvas.fill_rect(new_rect).unwrap();
        }

        self.canvas.present();
    }
}
