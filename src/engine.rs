use std::time::{Duration, Instant};

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::{self, AudioSubsystem, EventPump, Sdl, VideoSubsystem, render::Canvas, video::Window};

use crate::types::VectorUnsigned;

pub struct Engine {
    pub is_running: bool,
    pub video_system: VideoSubsystem,
    pub audio_system: AudioSubsystem,
    pub window_canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

impl Engine {
    pub fn init(window_name: &'static str, window_size: VectorUnsigned) -> Engine {
        let context: Sdl = sdl3::init().unwrap();

        let event_pump = context.event_pump().unwrap();
        let video_system = context.video().unwrap();
        let audio_system = context.audio().unwrap();

        let window = video_system
            .window(window_name, window_size.x, window_size.y)
            .position_centered()
            .build()
            .unwrap();

        let window_canvas = window.into_canvas();

        return Engine {
            is_running: false,
            event_pump,
            video_system,
            audio_system,
            window_canvas,
        };
    }

    pub fn run(&mut self, refresh_rate: u64) {
        let frame_target_ms = 1000 / refresh_rate;
        self.is_running = true;
        let mut prev_frame = Instant::now();

        while self.is_running {
            let frame_start = Instant::now();
            let delta_time = frame_start.duration_since(prev_frame).as_secs_f32();
            prev_frame = frame_start;

            self.poll_sdl_events();
            self.update(delta_time);
            self.render(delta_time);

            let elapsed = frame_start.elapsed();
            if elapsed < Duration::from_millis(frame_target_ms) {
                std::thread::sleep(Duration::from_millis(frame_target_ms) - elapsed);
            }
        }
    }

    fn poll_sdl_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { timestamp } => {
                    self.is_running = false;
                    println!("Quitted at: {}", timestamp);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,
                _ => {}
            }
        }
    }

    // TODO: Create async events for each of these runtime functions
    // @msfyre 03/25/2026

    fn update(&self, delta_time: f32) {
        println!("{}", delta_time)
    }

    fn render(&mut self, delta_time: f32) {
        self.window_canvas.present();
        println!("FPS: {}", (1 as f32) / delta_time)
    }
}
