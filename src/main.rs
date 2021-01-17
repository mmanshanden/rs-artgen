use anneal::{Config, Anneal};
use minifb::{Window, WindowOptions};

mod canvas;
mod color;
mod primitive;
mod anneal;

use canvas::Canvas;

fn main() {
    let window_options = WindowOptions::default();
    let mut window = Window::new("Lines", 512, 512, window_options).unwrap();

    let canvas = Canvas::from_file("lune.png", 512, 512);

    let config = Config {
        min_x: 0,
        max_x: 511,
        min_y: 0,
        max_y: 511,
    };

    let mut anneal: Anneal<primitive::Line, color::Grey> = Anneal::new(3000, &config, 512, 512);

    window.limit_update_rate(None);

    while window.is_open() {
        for _ in 0..10 {
            anneal.iterate(&config, &canvas);
        }

        window.update_with_buffer(&anneal.canvas.to_u32_buffer(), 512, 512).unwrap();
    }
}
