use std::time::Instant;

use crate::types::runtime::{RuntimeEvent, RuntimeEventCallback, RuntimeEventTrigger};

pub struct Runtime {
    running: bool,
    runtime_events: Vec<RuntimeEvent>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            running: false,
            runtime_events: Vec::new(),
        }
    }

    pub fn subscribe_to_runtime(&mut self, event: RuntimeEvent) {
        self.runtime_events.push(event);
    }

    pub fn execute(&mut self) -> Result<(), sdl3::Error> {
        self.running = true;
        let mut prev_frame = Instant::now();

        println!("Runtime executed.");

        while self.running {
            let current_frame = Instant::now();
            let delta_time = current_frame.duration_since(prev_frame).as_secs_f32();

            self.update(delta_time)?;
            self.render(delta_time)?;

            prev_frame = current_frame;
        }

        return Ok(());
    }

    // runtime loop
    fn update(&mut self, delta_time: f32) -> Result<(), sdl3::Error> {
        for event in self.runtime_events.iter_mut() {
            match event.trigger {
                RuntimeEventTrigger::UPDATE => match &mut event.callback {
                    RuntimeEventCallback::Function(f) => f(delta_time),
                    RuntimeEventCallback::Method(f) => f(delta_time)?,
                },
                RuntimeEventTrigger::RENDER => {}
            }
        }
        return Ok(());
    }
    fn render(&mut self, delta_time: f32) -> Result<(), sdl3::Error> {
        for event in self.runtime_events.iter_mut() {
            match event.trigger {
                RuntimeEventTrigger::UPDATE => {}
                RuntimeEventTrigger::RENDER => match &mut event.callback {
                    RuntimeEventCallback::Function(f) => f(delta_time),
                    RuntimeEventCallback::Method(f) => f(delta_time)?,
                },
            }
        }
        return Ok(());
    }
}
