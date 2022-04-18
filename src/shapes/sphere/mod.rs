#[allow(mixed_script_confusables)]

use glium::{Display, Surface, VertexBuffer};
use glium::IndexBuffer;
use glium::texture::*;

use crate::shapes::{Drawable, Transform, Vertex};

mod builder;
pub use self::builder::SphereBuilder;

/// Abstraction of a sphere.
/// Creates a sphere with a given radius
/// and a given number of subdivisions
/// both in the x and y directions.
pub struct Sphere {
    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<u16>,
    color: [f32; 3],
    program: glium::Program,
}

impl Sphere {
    /// Creates a new sphere.
    ///
    /// # Example
    /// ```no_run
    /// use rt::shapes::Sphere;
    /// let sphere = Sphere::new(1.0, [1.0, 0.0, 0.0], 10, 10);
    /// ```
    pub fn new(display: &Display, radius: f32, color: [f32; 3], lats: usize, longs: usize) -> Self {
        let mut vertices = Vec::with_capacity(lats * longs);
        let mut indices = Vec::with_capacity(lats * longs * 6);

        let lat_step = std::f32::consts::PI * 2.0 / lats as f32;
        let long_step = std::f32::consts::PI * 2.0 / longs as f32;

        for lat in 0..lats {
            let θ = lat_step * lat as f32;
            let y = radius * θ.sin();
            let h = radius * θ.cos();

            // Do not duplicate the poles
            if lat == 0 || lat == lats - 1 {
                vertices.push(Vertex::new(0.0, y, 0.0));
                continue;
            }

            for long in 0..longs {
                let φ = long_step * long as f32;
                let x = h * φ.sin();
                let z = h * φ.cos();


                vertices.push(Vertex::new(x, y, z));

                // Make indices list
                if lat != lats - 1 && long != longs - 1 {

                    let long = long as u16;
                    let longs = longs as u16;
                    let lat = lat as u16;

                    indices.append( &mut vec![
                        long + lat * longs,
                        (long + 1) + lat * longs,
                        long + (lat + 1) * longs
                    ]);
                    indices.append( &mut vec![
                        (long + 1) + lat * longs,
                        (long + 1) + (lat + 1) * longs,
                        long + (lat + 1) * longs
                    ]);
                }
            }
        }

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices[..]).unwrap();
        let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices[..]).unwrap();

        Sphere {
            vertices: vertex_buffer,
            indices: index_buffer,
            color,
            program: glium::Program::from_source(
                display,
                include_str!("sphere.vert"),
                include_str!("sphere.frag"),
                None,
            ).unwrap(),
        }
    }

    pub fn get_vertices(&self) -> &VertexBuffer<Vertex> {
        &self.vertices
    }

    pub fn get_program(&self) -> &glium::Program {
        &self.program
    }

    pub fn get_color(&self) -> [f32; 3] {
        self.color
    }
}

impl Drawable for Sphere {
    /// Draws the sphere.
    fn draw(&self, target: &mut glium::Frame, params: &glium::DrawParameters, transform: Transform, texture: &glium::texture::srgb_texture2d::SrgbTexture2d) {
        let uniforms = uniform! {
                color: self.color,
                translation: transform.get_translation(),
                scale: transform.get_scaling(),
                rotation: transform.get_rotation(),
                self_rotation: transform.get_self_rotation(),
                tex: texture
            };

        // println!("translation: {:?}\n#########\nscale: {:?}\n#########\nrotation: {:?}\n#########\nself_rotation: {:?}", transform.get_translation(), transform.get_scaling(), transform.get_rotation(), transform.get_self_rotation());

        target.draw(
            &self.vertices,
            &self.indices,
            &self.program,
            &uniforms,
            &params,
        ).unwrap();

        for (name, value) in self.program.uniforms() {
            println!("{:?}: {:?}", name, value);
        };
    }
}