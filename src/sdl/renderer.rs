use sdl3::{VideoSubsystem, render::Canvas, video::Window};

use crate::{
    runtime::Runtime,
    types::{
        runtime::{RuntimeEvent, RuntimeEventCallback, RuntimeEventTrigger},
        vectors::VectorUnsigned,
    },
};

pub struct Renderer {
    pub video_system: VideoSubsystem,
    pub window_canvas: Canvas<Window>,
    pub render_actions: Vec<fn(Box<&mut Canvas<Window>>, f32) -> Result<(), sdl3::Error>>,
}

impl Renderer {
    pub fn init(process_name: &str, process_size: VectorUnsigned) -> Self {
        let context = sdl3::init().unwrap();
        let video_system = context.video().unwrap();

        let window = video_system
            .window(process_name, process_size.x, process_size.y)
            .position_centered()
            .build()
            .unwrap();
        let window_canvas = window.into_canvas();

        return Self {
            video_system,
            window_canvas,
            render_actions: Vec::new(),
        };
    }

    pub fn begin_render(&mut self, runtime: &mut Runtime) {
        let self_ref = self as *mut Self;
        let method_boxed = Box::new(move |dt| unsafe { (*self_ref).render_window(dt) });
        let event = RuntimeEvent {
            trigger: RuntimeEventTrigger::RENDER,
            callback: RuntimeEventCallback::Method(method_boxed),
        };

        runtime.subscribe_to_runtime(event);
    }

    fn render_window(&mut self, dt: f32) -> Result<(), sdl3::Error> {
        for action in self.render_actions.iter() {
            action(Box::new(&mut self.window_canvas), dt)?;
        }

        self.window_canvas.present();

        return Ok(());
    }
}
