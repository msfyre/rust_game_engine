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
    window.set_draw_color(Color::WHITE);
    window.clear();

    println!("Render action called!");

    return Ok(());
}

fn main() -> Result<(), sdl3::Error> {
    let mut renderer = Renderer::init("i hate rust callbacks", VectorUnsigned { x: 250, y: 200 });
    let mut runtime = Runtime::new();

    renderer.render_actions.push(test_render);
    renderer.begin_render(&mut runtime);
    runtime.execute()?;
    return Ok(());
}
