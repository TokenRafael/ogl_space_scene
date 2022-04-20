use glium::{Display, DrawParameters, Frame, Program, Surface, VertexBuffer};
use glium::index::NoIndices;
use rand::Rng;
use crate::shapes::{StaticDrawble, Vertex};

pub struct Sky {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: NoIndices,
    pub program: Program,
}

impl Sky {
    pub fn new(display: &Display) -> Self {
        Sky::with_count(display, 100)
    }

    pub fn with_count(display: &Display, star_count: u32) -> Self {
        let stars = Self::gen_vertices(star_count);

        let vertices = VertexBuffer::new(display, &stars).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let program = glium::Program::from_source(display,
            include_str!("sky.vert"),
            include_str!("sky.frag"),
            None).unwrap();

        Sky {
            vertices,
            indices,
            program,
        }
    }

    fn gen_vertices(star_count: u32) -> Vec<Vertex> {
        let mut rng = rand::thread_rng();
        let stars = (0..star_count)
            .map(|_| {
                let x = rng.gen_range(-100..100i32) as f32 / 100.0;
                let y = rng.gen_range(-100..100i32) as f32 / 100.0;
                [x, y]
            })
            .map(|[x, y]| Vertex::new(x, y, -1.0))
            .collect::<Vec<_>>();
        stars
    }
}

impl StaticDrawble for Sky {
    fn draw(&self, target: &mut Frame, params: &DrawParameters) {
        let params = DrawParameters {
            depth: glium::Depth {
                range: (0.9, 1.0),
                ..params.depth.clone()
            },
            ..params.clone()
        };

        target.draw(
            &self.vertices,
            &self.indices,
            &self.program,
            &uniform! {},
            &params
        ).unwrap();
    }
}