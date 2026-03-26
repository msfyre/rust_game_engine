use sdl3::{
    Sdl, VideoSubsystem, event::Event, keyboard::Keycode, pixels::Color, render::Canvas,
    video::Window,
};

use crate::{
    runtime::Runtime,
    types::{
        runtime::{RuntimeEvent, RuntimeEventCallback, RuntimeEventTrigger},
        vectors::VectorUnsigned,
    },
};

pub struct Renderer {
    pub context: Sdl,
    pub video_system: VideoSubsystem,
    pub window_canvas: Canvas<Window>,
    pub window_size: VectorUnsigned,
    pub render_actions: Vec<fn(Box<&mut Canvas<Window>>, f32) -> Result<(), sdl3::Error>>,
}

impl Renderer {
    pub fn init(process_name: &str, process_size: VectorUnsigned) -> Self {
        let context = sdl3::init().unwrap();
        let video_system = context.video().unwrap();

        let window = video_system
            .window(process_name, process_size.x, process_size.y)
            .resizable()
            .build()
            .unwrap();
        let window_canvas = window.into_canvas();

        return Self {
            context,
            video_system,
            window_canvas,
            window_size: process_size,
            render_actions: Vec::new(),
        };
    }

    pub fn begin_render(&mut self, runtime: &mut Runtime) {
        let self_ref = self as *mut Self;

        runtime.subscribe_to_runtime(RuntimeEvent {
            trigger: RuntimeEventTrigger::RENDER,
            callback: RuntimeEventCallback::Method(Box::new(move |dt| unsafe {
                (*self_ref).render_window(dt)
            })),
        });
        runtime.subscribe_to_runtime(RuntimeEvent {
            trigger: RuntimeEventTrigger::UPDATE,
            callback: RuntimeEventCallback::MethodWithMutableRuntimeReference(Box::new(
                move |_dt, runtime| unsafe { (*self_ref).poll_sdl_events(runtime) },
            )),
        });
    }

    pub fn poll_sdl_events(&self, runtime: &mut Runtime) -> Result<(), sdl3::Error> {
        let mut events = self.context.event_pump().unwrap();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    runtime.stop();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    runtime.stop();
                }
                _ => {}
            }
        }
        return Ok(());
    }

    fn render_window(&mut self, dt: f32) -> Result<(), sdl3::Error> {
        self.window_canvas.set_draw_color(Color::BLACK);
        self.window_canvas.clear();

        for action in self.render_actions.iter() {
            action(Box::new(&mut self.window_canvas), dt)?;
        }

        self.window_canvas.present();
        self.window_size = VectorUnsigned {
            x: self.window_canvas.output_size().unwrap().0,
            y: self.window_canvas.output_size().unwrap().1,
        };

        return Ok(());
    }
}
