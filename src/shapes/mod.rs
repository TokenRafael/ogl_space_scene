use crate::{rotate, scale, translate};

pub mod sphere;
pub mod matrices;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn new(p0: f32, p1: f32, p2: f32) -> Self {
        Vertex {
            position: [p0, p1, p2],
        }
    }
}

implement_vertex!(Vertex, position);

pub trait Drawable {
    fn draw(&self, target: &mut glium::Frame, params: &glium::DrawParameters, transform: Transform);
}

pub struct Transform {
    /// Translate in [x, y, z]
    pub translation: [f32; 3],
    /// Rotate in [x, y, z]
    pub rotation: [f32; 3],
    /// Rotate object around itself in [x, y, z]
    pub rotate_self: [f32; 3],
    /// Scale in s
    pub scale: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            rotate_self: [0.0, 0.0, 0.0],
            scale: 1.0,
        }
    }
}

impl Transform {
    pub fn get_translation(&self) -> [[f32; 4]; 4] {
        translate!(self.translation[0], self.translation[1], self.translation[2])
    }

    pub fn get_rotation(&self) -> [[f32; 4]; 4] {
        rotate!(self.rotation[0], self.rotation[1], self.rotation[2])
    }

    pub fn get_scaling(&self) -> [[f32; 4]; 4] {
        scale!(self.scale)
    }

    pub fn get_self_rotation(&self) -> [[f32; 4]; 4] {
        rotate!(self.rotate_self[0], self.rotate_self[1], self.rotate_self[2])
    }
}
