#![allow(dead_code)]

use glium::Display;
use crate::shapes::Filling;

use crate::shapes::sphere::Sphere;

/// Helps to create a sphere.
pub struct SphereBuilder {
    radius: f32,
    color: [f32; 3],
    lats: usize,
    longs: usize,
}

impl SphereBuilder {
    /// Creates a new sphere builder.
    ///
    /// # Example
    /// ```no_run
    /// use rt::shapes::SphereBuilder;
    /// let sphere = SphereBuilder::new()
    ///  .radius(1.0)
    ///  .color([1.0, 0.0, 0.0])
    ///  .lats(10)
    ///  .longs(10)
    ///  .build(display);
    /// ```
    pub fn new() -> Self {
        SphereBuilder {
            radius: 1.0,
            filling: Filling::Color([1.0, 0.0, 0.0]),
            lats: 100,
            longs: 100,
        }
    }

    /// Sets the radius of the sphere.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the color of the sphere.
    pub fn color(mut self, color: [f32; 3]) -> Self {
        self.filling = Filling::Color(color);
        self
    }

    pub fn texture(mut self, texture: glium::texture::SrgbTexture2d) -> Self {
        self.filling = Filling::Texture(texture);
        self
    }

    /// Sets the number of latitudes.
    pub fn lats(mut self, lats: usize) -> Self {
        self.lats = lats;
        self
    }

    /// Sets the number of longitudes.
    pub fn longs(mut self, longs: usize) -> Self {
        self.longs = longs;
        self
    }

    /// Builds the sphere.
    pub fn build(self, display: &Display) -> Sphere {
        Sphere::new(display, self.radius, self.color, self.lats, self.longs)
    }
}