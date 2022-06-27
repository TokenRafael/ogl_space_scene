use glium::{Display, DrawParameters, Frame, Program, Surface, VertexBuffer};
use glium::index::NoIndices;
use crate::{DynDrawble, Transform};
use crate::shapes::Vertex;
use crate::shapes::triangle::{Triangle, TriangleBuilder};

pub struct Star {
    triangles: Vec<Triangle>,
    pub shine_vertices: VertexBuffer<Vertex>,
    pub shine_index: NoIndices,
    pub shine_program: Program,
}

impl Star {
    pub fn new(display: &Display) -> Self {
        let triangles = (0..2).map(|_| TriangleBuilder::new().color([1.0; 3]).build(display)).collect();

        let shine_points = (0..360)
            .step_by(60)
            .map(|angle| (angle as f32).to_radians())
            .map(|angle| {
                let x = (angle.cos());
                let y = (angle.sin());
                let x1 = (angle.cos() * 1.2);
                let y1 = (angle.sin() * 1.2);
                [
                    Vertex::new(x, y, 0.0),
                    Vertex::new(x1, y1, 0.0),
                ]
            })
            .flatten()
            .collect::<Vec<Vertex>>();

        let shine_vertices = glium::VertexBuffer::new(display, &shine_points).unwrap();

        let shine_index = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

        let shine_program = glium::Program::from_source(
            display,
            include_str!("../sphere/sphere.vert"),
            include_str!("../sky/sky.frag"),
            None
        ).unwrap();

        Star {
            triangles,
            shine_vertices,
            shine_index,
            shine_program,
        }
    }
}

impl DynDrawble for Star {
    fn draw(&self, target: &mut Frame, params: &DrawParameters, transform: Transform) {
        let uniforms = uniform! {
            translation: transform.get_translation(),
            rotation: transform.get_rotation(),
            scale: transform.get_scaling(),
            self_rotation: transform.get_self_rotation(),
            view: transform.get_view()
        };

        let rotate_self = transform.rotate_self;
        let second_triangle_transform = Transform {
            rotate_self: [rotate_self[0], rotate_self[1], (rotate_self[2] + 60f32.to_radians())],
            ..transform.clone()
        };

        self.triangles[0].draw(target, params, transform);
        self.triangles[1].draw(target, params, second_triangle_transform);

        target.draw(
            &self.shine_vertices,
            &self.shine_index,
            &self.shine_program,
            &uniforms,
            params
        ).unwrap();
    }
}