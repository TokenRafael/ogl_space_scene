use std::iter::Map;
use std::ops::Range;
use glium::texture;
use crate::{rotate, scale, translate};
use crate::matrices::view_matrix;

pub mod sphere;
pub mod cube;
pub mod matrices;
pub mod sky;
pub mod ring_planet;
pub mod triangle;
pub mod star;

/// Enum that decides the filling for a given shape, be it a RGB color or a texture.
#[derive(Debug)]
pub enum Filling {
    Color([f32; 3]),
    Texture(texture::SrgbTexture2d),
}

impl Filling {
    pub fn get_color(&self) -> Option<[f32; 3]> {
        if let Filling::Color(color) = self {
            Some(*color)
        } else {
            None
        }
    }

    pub fn get_texture(&self) -> Option<&texture::SrgbTexture2d> {
        if let Filling::Texture(texture) = self {
            Some(texture)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2]
}

impl Vertex {
    pub fn new(p0: f32, p1: f32, p2: f32) -> Self {

        /// Maps the (x, y, z) coordinates to the (u, v) coordinates of the sphere.
        let u = - (0.5 + (p0.atan2(p2) / (std::f32::consts::PI * 2.0)));
        let v = 0.5 + p1.asin() / std::f32::consts::PI;

        Vertex {
            position: [p0, p1, p2],
            tex_coords: [u, v]
        }
    }
}

implement_vertex!(Vertex, position, tex_coords);

/// Function that normalizes a range [a, b] to a given range [c, d].
pub fn map_range((from_start, from_end): (f32, f32), (to_start, to_end): (f32, f32), value: f32) -> f32 {
    let from_range = from_end - from_start;
    let to_range = to_end - to_start;
    let from_value = value - from_start;
    let to_value = from_value * to_range / from_range + to_start;
    to_value
}

/// Interface for moving drawable objects.
pub trait DynDrawble {
    fn draw(&self, target: &mut glium::Frame, params: &glium::DrawParameters, transform: Transform);
}

/// Interface for static drawable objects.
pub trait StaticDrawble {
    fn draw(&mut self, target: &mut glium::Frame, params: &glium::DrawParameters);
}

/// Struct that holds the transform parameters of a drawable object.
#[derive(Clone)]
pub struct Transform {
    /// Translate in [x, y, z]
    pub translation: [f32; 3],
    /// Rotate in [x, y, z]
    pub rotation: [f32; 3],
    /// Rotate object around itself in [x, y, z]
    pub rotate_self: [f32; 3],
    /// Scale in s
    pub scale: f32,
    /// View in [direction, position, up]
    pub view: [[f32; 3]; 3],
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            rotate_self: [0.0, 0.0, 0.0],
            scale: 1.0,
            view: [[0.0, 0.0, 1.0], [-2.0, 1.0, 1.0], [0.0, 1.0, 0.0]],
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

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        view_matrix(&self.view[0], &self.view[1], &self.view[2])
    }
}

/// Macro that loads a texture from a file and returns a texture.
#[macro_export]
macro_rules! load_tex {
    ($display: ident, $path: expr, jpeg) => {
        {
            let image = image::load(std::io::Cursor::new(&include_bytes!($path)),
                            image::ImageFormat::Jpeg).unwrap().to_rgba8();

            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

            let texture = glium::texture::SrgbTexture2d::new(&$display, image).unwrap();

            texture
        }
    };
    ($display: ident, $path: expr, png) => {
        {
            let image = image::load(std::io::Cursor::new(&include_bytes!($path)),
                            image::ImageFormat::Png).unwrap().to_rgba8();

            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

            let texture = glium::texture::SrgbTexture2d::new(&$display, image).unwrap();

            texture
        }
    };
}
