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
    let window_size = window.output_size().unwrap();
    let fps_string = format!("FPS: {}", 1.0 / dt);
    let size_string = format!("Size: {}x{}", window_size.0, window_size.1);
    window.set_draw_color(Color::WHITE);
    window.draw_debug_text(
        &fps_string,
        FPoint {
            x: 0 as f32,
            y: 0 as f32,
        },
    )?;
    window.draw_debug_text(
        &size_string,
        FPoint {
            x: 0 as f32,
            y: 8 as f32,
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
