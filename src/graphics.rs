use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::Sdl;

pub struct Graphics {
    pub canvas: Canvas<sdl2::video::Window>,
    pub context: Sdl,
}

impl Graphics {
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let window =
            video_subsystem.window("Chip8 Emulator", 800, 600).position_centered().build().unwrap();

        let canvas = window.into_canvas().build().unwrap();
        Self { canvas, context }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }
}
