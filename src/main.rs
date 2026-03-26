use sdl3::{
    pixels::Color,
    render::{Canvas, FPoint},
    video::Window,
};

use crate::{runtime::Runtime, sdl::renderer::Renderer, types::vectors::VectorUnsigned};

mod runtime;
mod sdl;
mod types;

fn draw_debug_info(window_canvas: Box<&mut Canvas<Window>>, dt: f32) -> Result<(), sdl3::Error> {
    let window_size = window_canvas.output_size().unwrap();
    let fps_string = format!("FPS: {}", 1.0 / dt);
    let size_string = format!("Size: {}x{}", window_size.0, window_size.1);
    window_canvas.set_draw_color(Color::WHITE);
    window_canvas.draw_debug_text(
        &fps_string,
        FPoint {
            x: 5 as f32,
            y: 5 as f32,
        },
    )?;
    window_canvas.draw_debug_text(
        &size_string,
        FPoint {
            x: 5 as f32,
            y: 13 as f32,
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

    renderer.render_actions.push(draw_debug_info);
    renderer.begin_render(&mut runtime);
    runtime.execute()?;
    return Ok(());
}
