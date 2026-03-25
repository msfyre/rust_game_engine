use std::{
    io::{Error, Stdout},
    time::Instant,
};

use tui::{Terminal, backend::CrosstermBackend};

pub struct Runtime {
    running: bool,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Runtime {
    pub fn init() -> Result<Runtime, Error> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        return Ok(Runtime {
            running: false,
            terminal,
        });
    }

    pub fn execute(&mut self) {
        let mut prev_frame = Instant::now();

        self.running = true;

        while self.running {
            let current_frame = Instant::now();
            let delta_time = current_frame.duration_since(prev_frame).as_secs_f32();

            self.update(delta_time);
            self.render(delta_time);

            prev_frame = current_frame;
        }
    }

    fn update(&self, delta_time: f32) {
        println!("Delta Time: {}", delta_time);
    }

    fn render(&self, delta_time: f32) {}
}
