use std::f32::consts::PI;
use glium::{Display, DrawParameters, Frame, Surface, VertexBuffer};
use crate::{DynDrawble, shapes, StaticDrawble, Transform};
use crate::shapes::sphere::{Sphere, SphereBuilder};
use crate::shapes::Vertex;
use crate::translate;

pub struct RingPlanet {
    planet: Sphere,
    ring: VertexBuffer<Vertex>,
    ring_program: glium::Program,
}

impl RingPlanet {
    pub fn new(display: &glium::Display, ring_radius: f32, sphere: Sphere) -> RingPlanet {
        assert!(sphere.radius() < ring_radius);
        let ring_points = (0..360)
            .map(|i| (i as f32).to_radians())
            .map(|angle| {
                let x = angle.cos() * ring_radius;
                let z = angle.sin() * ring_radius;
                Vertex {
                    position: [x, 0.0, z],
                    tex_coords: [0.0, 0.0],
                }
            })
            .collect::<Vec<_>>();
        let ring = VertexBuffer::new(display, &ring_points).unwrap();
        let ring_program = glium::Program::from_source(display,
            include_str!("../sphere/sphere.vert"),
            include_str!("../sphere/sphere.frag"),
            None).unwrap();

        RingPlanet {
            planet: sphere,
            ring,
            ring_program,
        }
    }
}

impl DynDrawble for RingPlanet {
    fn draw(&self, target: &mut Frame, params: &DrawParameters, transform: Transform) {
        let uniforms = uniform! {
                translation: transform.get_translation(),
                undo_translation: translate!(-transform.translation[0], -transform.translation[1], -transform.translation[2]),
                scale: transform.get_scaling(),
                rotation: transform.get_rotation(),
                self_rotation: transform.get_self_rotation(),
                color: [1.0f32, 1.0, 1.0],
            };
        self.planet.draw(target, params, transform);
        target.draw(
            &self.ring,
            glium::index::NoIndices(glium::index::PrimitiveType::LineLoop),
            &self.ring_program,
            &uniforms,
            params
        ).unwrap();
    }
}