use glium::{Display, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer};
use glium::index::NoIndices;

pub use builder::CubeBuilder;

use crate::{Drawable, Transform};
use crate::shapes::Vertex;
use crate::translate;

mod builder;

pub struct Cube {
    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<u16>,
    color: [f32; 3],
    program: glium::Program,
    wireframe_indices: IndexBuffer<u16>,
    wireframe_program: glium::Program,
}

impl Cube {
    pub fn new(display: &Display, origin: [f32; 3], size: f32, color: [f32; 3]) -> Cube {
        let vertices = Cube::generate_vertices(origin, size);
        let (indices, wireframe_indices) = Cube::generate_indices();
        let (program, wireframe_program) = Cube::generate_program(display);


        Cube {
            vertices: VertexBuffer::new(display, &vertices).unwrap(),
            indices:
                IndexBuffer::new(
                    display,
                    glium::index::PrimitiveType::TrianglesList,
                    &indices
                ).unwrap(),
            color,
            program,
            wireframe_indices:
                IndexBuffer::new(
                    display,
                    glium::index::PrimitiveType::LinesList,
                    &wireframe_indices
                ).unwrap(),
            wireframe_program,
        }
    }

    fn generate_vertices(origin: [f32; 3], size: f32) -> [Vertex; 8] {
        let (right, left) = (origin[0] + size, origin[0] - size);
        let (top, bottom) = (origin[1] + size, origin[1] - size);
        let (front, back) = (origin[2] + size, origin[2] - size);
        [
            Vertex::new(right, top, front),
            Vertex::new(left, top, front),
            Vertex::new(left, bottom, front),
            Vertex::new(right, bottom, front),
            Vertex::new(right, top, back),
            Vertex::new(left, top, back),
            Vertex::new(left, bottom, back),
            Vertex::new(right, bottom, back),
        ]
    }

    fn generate_indices() -> ([u16; 36], [u16; 24]) {
        let cube_indices = [
            0u16, 1, 2, // front 1
            0, 2, 3, // front 2
            0, 3, 4, // right 1
            3, 4, 7, // right 2
            1, 2, 5, // left 1
            2, 5, 6, // left 2
            4, 5, 6, // back 1
            5, 6, 7, // back 2
            0, 1, 5, // top 1
            1, 4, 5, // top 2
            2, 3, 6, // bottom 1
            3, 6, 7, // bottom 2
        ];

        let wireframe_indices = [
            // front
            0u16, 1,
            1, 2,
            2, 3,
            3, 0,
            // back
            4, 5,
            5, 6,
            6, 7,
            7, 4,
            // sides
            0, 4,
            1, 5,
            2, 6,
            3, 7,
        ];

        (cube_indices, wireframe_indices)

    }

    fn generate_program(display: &Display) -> (Program, Program) {
        let program = glium::Program::from_source(
            display,
            include_str!("cube.vert"),
            include_str!("cube.frag"),
            None).unwrap();

        let wireframe_program = glium::Program::from_source(
            display,
            include_str!("cube.vert"),
            include_str!("wireframe.frag"),
            None).unwrap();

        (program, wireframe_program)
    }
}

impl Drawable for Cube {
    fn draw(&self,
            target: &mut Frame,
            params: &DrawParameters,
            transform: Transform,
    ) {
        let uniforms = uniform! {
            color: self.color,
            translation: transform.get_translation(),
            undo_translation: translate!(-transform.translation[0], -transform.translation[1], -transform.translation[2]),
            rotation: transform.get_rotation(),
            scale: transform.get_scaling(),
            self_rotation: transform.get_self_rotation(),
        };

        // Draw cube itself
        target.draw(
            &self.vertices,
            &self.indices,
            &self.program,
            &uniforms,
            &DrawParameters {
                line_width: Some(3.0),
                ..params.clone()
            },
        ).unwrap();

        // Draw cube wireframe
        target.draw(
            &self.vertices,
            &self.wireframe_indices,
            &self.wireframe_program,
            &uniforms,
            params,
        ).unwrap();
    }
}
