mod engine;
mod types;

use std::{self, time::Duration};

use sdl3::pixels::Color;

use types::VectorUnsigned;

fn main() {
    const WINDOW_NAME: &'static str = "ENGINE";
    const WINDOW_SIZE: VectorUnsigned = VectorUnsigned { x: 250, y: 250 };
    const WINDOW_BASE_COLOR: Color = Color::RGB(255, 255, 255);

    let mut game_engine: engine::Engine = engine::Engine::init(WINDOW_NAME, WINDOW_SIZE);
    game_engine.window_canvas.set_draw_color(WINDOW_BASE_COLOR);
    game_engine.window_canvas.clear();
    game_engine.window_canvas.present();

    std::thread::sleep(Duration::from_secs(5));
}
