use std::iter::{Cycle, Map};
use std::ops::Range;
use glium::{Display, DrawParameters, Frame, Program, Surface, VertexBuffer};
use glium::index::NoIndices;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::shapes::{map_range, StaticDrawble, Vertex};
use crate::{identity, translate};

pub struct Sky {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: NoIndices,
    pub program: Program,
    pub shooting_stars: VertexBuffer<Vertex>,
    pub shooting_indices: NoIndices,
    pub shooting_frame: Cycle<Range<i32>>,
}

const ANIMATION_FRAMES: i32 = 480;

impl Sky {
    pub fn new(display: &Display) -> Self {
        Sky::with_count(display, 100, 7)
    }

    pub fn with_count(display: &Display, star_count: u32, shooting_star_count: u32) -> Self {
        let stars = Self::gen_vertices(star_count);

        let vertices = VertexBuffer::new(display, &stars).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let program = glium::Program::from_source(display,
                                                  include_str!("sky.vert"),
                                                  include_str!("sky.frag"),
                                                  None).unwrap();

        let shooting_stars = Self::gen_shooting_stars(shooting_star_count);
        let shooting_stars = VertexBuffer::new(display, &shooting_stars).unwrap();
        let shooting_indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

        Sky {
            vertices,
            indices,
            program,
            shooting_stars,
            shooting_indices,
            shooting_frame: (0..ANIMATION_FRAMES).cycle(),
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

    fn gen_shooting_stars(shooting_star_count: u32) -> Vec<Vertex> {
        let mut rng = rand::thread_rng();
        let shooting_stars = Self::gen_rand_points_iter(shooting_star_count, rng)
            .map(|[x, y]| [
                    Vertex::new(x, y, -1.0)
                ,   Vertex::new(x - 0.5, y - 0.5, -1.0)
                ]
            )
            .flatten()
            .collect::<Vec<_>>();
        shooting_stars
    }

    fn gen_rand_points_iter(shooting_star_count: u32, mut rng: ThreadRng) -> impl Iterator <Item = [f32; 2]> {
        (0..shooting_star_count)
            .map(move |_| {
                let x = rng.gen_range(-100..100i32) as f32 / 100.0;
                let y = rng.gen_range(-100..100i32) as f32 / 100.0;
                [x, y]
            })
    }
}

impl StaticDrawble for Sky {
    fn draw(&mut self, target: &mut Frame, params: &DrawParameters) {
        let params = DrawParameters {
            depth: glium::Depth {
                range: (0.9, 1.0),

                ..params.depth.clone()
            },
            // line_width: Some(0.1),
            ..params.clone()
        };

        target.draw(
            &self.vertices,
            &self.indices,
            &self.program,
            &uniform! {
                translation: identity!()
            },
            &params
        ).unwrap();

        let translation_x = map_range(
            (0., ANIMATION_FRAMES as f32),
            (1.5, -1.5),
            self.shooting_frame.next().unwrap() as f32
        );
        let translation_y = map_range(
            (0., ANIMATION_FRAMES as f32),
            (2.5, -1.),
            self.shooting_frame.next().unwrap() as f32
        );
        let translation_matrix = translate!(translation_x, translation_y, 0.);

        target.draw(
            &self.shooting_stars,
            &self.shooting_indices,
            &self.program,
            &uniform! {
                translation: translation_matrix
            },
            &params
        ).unwrap();
    }
}