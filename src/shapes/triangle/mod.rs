mod builder;

use glium::{Display, DrawParameters, Frame, Program, Surface};
use glium::index::NoIndices;
use glium::VertexBuffer;
use crate::{DynDrawble, Transform};
use crate::shapes::Vertex;

pub use builder::TriangleBuilder;

pub struct Triangle {
    vertices: VertexBuffer<Vertex>,
    indices: NoIndices,
    program: Program,
    color: [f32; 3],
}

impl Triangle {
    pub fn new(display: &Display, points: &[[f32; 2]; 3], color: [f32; 3]) -> Self {
        let vertices = Self::generate_vertices(points);
        let vertices = VertexBuffer::new(display, &vertices[..]).unwrap();

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let program = glium::Program::from_source(display,
            include_str!("../sphere/sphere.vert"),
            include_str!("../sphere/sphere.frag"),
            None).unwrap();

        Triangle {
            vertices,
            indices,
            program,
            color,
        }

    }

    fn generate_vertices(points: &[[f32; 2]; 3]) -> Vec<Vertex> {
        let vertices: Vec<Vertex> = points.to_vec().iter()
            .map(|p| Vertex::new(p[0], p[1], 0.0f32))
            .collect();
        vertices
    }
}

impl DynDrawble for Triangle {
    fn draw(&self, target: &mut Frame, params: &DrawParameters, transform: Transform) {
        let uniforms = uniform! {
            color: self.color,
            translation: transform.get_translation(),
            rotation: transform.get_rotation(),
            scale: transform.get_scaling(),
            self_rotation: transform.get_self_rotation(),
            view: transform.get_view(),
        };

        target.draw(
            &self.vertices,
            &self.indices,
            &self.program,
            &uniforms,
            params
        ).unwrap();
    }
}