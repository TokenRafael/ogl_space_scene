use glium::Display;
use crate::shapes::cube::Cube;

pub struct CubeBuilder {
    origin: [f32; 3],
    size: f32,
    color: [f32; 3],
}

impl CubeBuilder {
    pub fn new() -> Self {
        CubeBuilder {
            origin: [0.0; 3],
            size: 1.0,
            color: [1.0, 1.0, 1.0],
        }
    }

    pub fn origin(mut self, origin: [f32; 3]) -> Self {
        self.origin = origin;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: [f32; 3]) -> Self {
        self.color = color;
        self
    }

    pub fn build(self, display: &Display) -> Cube {
        Cube::new(display, self.origin, self.size, self.color)
    }
}