use glium::Display;
use crate::shapes::triangle::Triangle;

pub struct TriangleBuilder {
    points: [[f32; 2]; 3],
    color: [f32; 3],
}

impl TriangleBuilder {
    pub fn new() -> Self {
        let points = (0..360)
            .step_by(120)
            .take(3)
            .map(|i| {
                let x = (i as f32).to_radians().sin();
                let y = (i as f32).to_radians().cos();
                [x, y]
            })
            .collect::<Vec<[f32;2]>>();
        Self {
            points: [points[0], points[1], points[2]],
            color: [0.0, 1.0, 0.0],
        }
    }

    pub fn points(&mut self, points: [[f32; 2]; 3]) -> &mut Self {
        self.points = points;
        self
    }

    pub fn color(&mut self, color: [f32; 3]) -> &mut Self {
        self.color = color;
        self
    }

    pub fn build(&self, display: &Display) -> Triangle {
        Triangle::new(display, &self.points, self.color)
    }
}