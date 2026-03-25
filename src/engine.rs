use sdl3::{self, Sdl, VideoSubsystem, render::Canvas, video::Window};

use crate::types::VectorUnsigned;

pub struct Engine {
    video_system: VideoSubsystem,
    pub window_canvas: Canvas<Window>,
}

impl Engine {
    pub fn init(window_name: &'static str, window_size: VectorUnsigned) -> Engine {
        let context: Sdl = sdl3::init().unwrap();

        let video_system = context.video().unwrap();

        let window = video_system
            .window(window_name, window_size.x, window_size.y)
            .position_centered()
            .build()
            .unwrap();

        let mut window_canvas = window.into_canvas();

        return Engine {
            video_system,
            window_canvas,
        };
    }
}
