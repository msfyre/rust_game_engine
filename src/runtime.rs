use std::time::Instant;

use crate::types::runtime::{RuntimeEvent, RuntimeEventCallback, RuntimeEventTrigger};

pub struct Runtime {
    refresh_rate: f32,
    running: bool,
    runtime_events: Vec<RuntimeEvent>,
}

impl Runtime {
    pub fn new(refresh_rate: f32) -> Self {
        Self {
            refresh_rate,
            running: false,
            runtime_events: Vec::new(),
        }
    }

    pub fn subscribe_to_runtime(&mut self, event: RuntimeEvent) {
        self.runtime_events.push(event);
    }

    pub fn execute(&mut self) -> Result<(), sdl3::Error> {
        let target_frame_time = 1000.0 / self.refresh_rate;
        self.running = true;
        let mut prev_frame = Instant::now();

        println!("Runtime executed.");

        while self.running {
            let current_frame = Instant::now();
            let delta_time = current_frame.duration_since(prev_frame).as_secs_f32();

            self.update(delta_time)?;
            self.render(delta_time)?;

            if delta_time < target_frame_time {
                std::thread::sleep(std::time::Duration::from_secs_f32(
                    target_frame_time - delta_time,
                ));
            }

            prev_frame = current_frame;
        }

        return Ok(());
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    // runtime loop
    fn update(&mut self, delta_time: f32) -> Result<(), sdl3::Error> {
        let mut events = std::mem::take(&mut self.runtime_events);

        for event in events.iter_mut() {
            match event.trigger {
                RuntimeEventTrigger::UPDATE => match &mut event.callback {
                    RuntimeEventCallback::Function(f) => f(delta_time)?,
                    RuntimeEventCallback::Method(f) => f(delta_time)?,
                    RuntimeEventCallback::MethodWithRuntimeReference(f) => f(delta_time, &self)?,
                    RuntimeEventCallback::MethodWithMutableRuntimeReference(f) => {
                        f(delta_time, self)?
                    }
                },
                RuntimeEventTrigger::RENDER => {}
            }
        }

        self.runtime_events = events;

        return Ok(());
    }
    fn render(&mut self, delta_time: f32) -> Result<(), sdl3::Error> {
        let mut events = std::mem::take(&mut self.runtime_events);

        for event in events.iter_mut() {
            match event.trigger {
                RuntimeEventTrigger::UPDATE => {}
                RuntimeEventTrigger::RENDER => match &mut event.callback {
                    RuntimeEventCallback::Function(f) => f(delta_time)?,
                    RuntimeEventCallback::Method(f) => f(delta_time)?,
                    RuntimeEventCallback::MethodWithRuntimeReference(f) => f(delta_time, &self)?,
                    RuntimeEventCallback::MethodWithMutableRuntimeReference(f) => {
                        f(delta_time, self)?
                    }
                },
            }
        }

        self.runtime_events = events;

        return Ok(());
    }
}
