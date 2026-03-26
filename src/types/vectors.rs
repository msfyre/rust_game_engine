pub trait VectorMethod {
    fn magnitude(&self) -> f32;
    fn normalize(&self) -> Self;
}

pub struct VectorUnsigned {
    pub x: u32,
    pub y: u32,
}

pub struct VectorFloat {
    pub x: f32,
    pub y: f32,
}

impl VectorMethod for VectorUnsigned {
    fn magnitude(&self) -> f32 {
        return ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt();
    }
    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        return Self {
            x: (self.x / magnitude as u32),
            y: (self.y / magnitude as u32),
        };
    }
}

impl VectorMethod for VectorFloat {
    fn magnitude(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        return Self {
            x: (self.x / magnitude),
            y: (self.y / magnitude),
        };
    }
}
