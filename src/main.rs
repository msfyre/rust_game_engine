mod engine;
mod types;

use types::VectorUnsigned;

fn main() {
    const WINDOW_NAME: &'static str = "ENGINE";
    const WINDOW_SIZE: VectorUnsigned = VectorUnsigned { x: 250, y: 250 };
    const REFRESH_RATE: u64 = 256;

    let mut game_engine: engine::Engine = engine::Engine::init(WINDOW_NAME, WINDOW_SIZE);
    game_engine.run(REFRESH_RATE);
}
