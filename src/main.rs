use sdl3::{
    pixels::Color,
    render::{Canvas, FPoint},
    video::Window,
};

use crate::{runtime::Runtime, sdl::renderer::Renderer, types::vectors::VectorUnsigned};

mod runtime;
mod sdl;
mod types;

fn test_render(window: Box<&mut Canvas<Window>>, dt: f32) -> Result<(), sdl3::Error> {
    let formatted = format!("FPS: {}", 1.0 / dt);
    window.set_draw_color(Color::WHITE);
    window.draw_debug_text(
        &formatted,
        FPoint {
            x: 0 as f32,
            y: 0 as f32,
        },
    )?;

    return Ok(());
}

fn main() -> Result<(), sdl3::Error> {
    const REFRESH_RATE: f32 = 60.0;

    let mut renderer = Renderer::init("i hate rust callbacks", VectorUnsigned { x: 250, y: 200 });
    let mut runtime = Runtime::new(REFRESH_RATE);

    renderer.render_actions.push(test_render);
    renderer.begin_render(&mut runtime);
    runtime.execute()?;
    return Ok(());
}
