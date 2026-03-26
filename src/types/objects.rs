use std::{cell::RefCell, rc::Rc};

use sdl3::{
    pixels::Color,
    render::{Canvas, FPoint},
    video::Window,
};

use crate::{
    sdl::renderer::{Renderer, RendererAction},
    types::vectors::VectorFloat,
};

pub struct Square {
    pub position: VectorFloat,
    pub rotation: f32,
    pub side_length: u32,
    pub color_outline: Color,
}

impl Square {
    pub fn new(side_length: u32, color_outline: Color) -> Self {
        return Self {
            position: VectorFloat { x: 0.0, y: 0.0 },
            rotation: 0.0,
            side_length,
            color_outline,
        };
    }

    pub fn render(&mut self, window_canvas: Box<&mut Canvas<Window>>) -> Result<(), sdl3::Error> {
        let points = [
            FPoint {
                x: self.position.x
                    + ((self.side_length as f32) * (self.rotation + 45.0).to_radians().cos()),
                y: self.position.y
                    + ((self.side_length as f32) * (self.rotation + 45.0).to_radians().sin()),
            },
            FPoint {
                x: self.position.x
                    + ((self.side_length as f32) * (self.rotation + 135.0).to_radians().cos()),
                y: self.position.y
                    + ((self.side_length as f32) * (self.rotation + 135.0).to_radians().sin()),
            },
            FPoint {
                x: self.position.x
                    + ((self.side_length as f32) * (self.rotation + 225.0).to_radians().cos()),
                y: self.position.y
                    + ((self.side_length as f32) * (self.rotation + 225.0).to_radians().sin()),
            },
            FPoint {
                x: self.position.x
                    + ((self.side_length as f32) * (self.rotation + 315.0).to_radians().cos()),
                y: self.position.y
                    + ((self.side_length as f32) * (self.rotation + 315.0).to_radians().sin()),
            },
        ];

        window_canvas.set_draw_color(self.color_outline);

        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];

            window_canvas.draw_line(p1, p2)?;
        }

        return Ok(());
    }
}
