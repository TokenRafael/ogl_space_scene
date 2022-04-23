use std::f32::consts::PI;

use glium::{Frame, Surface};

/// Macros to create the matrices of the various transforms used in the program

/// Creates a matrix that translates the object by the given amount
#[macro_export]
macro_rules! translate {
    [$param: expr, x] => {
        [
            [1.0f32, 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [$param, 0., 0., 1.],
        ]
    };
    [$param: expr, y] => {
        [
            [1.0f32, 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., $param, 0., 1.],
        ]
    };
    [$param: expr, z] => {
        [
            [1.0f32, 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., $param, 1.],
        ]
    };
    [$x: expr, $y: expr, $z: expr] => {
        [
            [1.0f32, 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [$x, $y, $z, 1.],
        ]
    }
}

/// Creates a matrix that rotates the object by the given angle
#[macro_export]
macro_rules! rotate {
    [$param:expr, x] => {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, $param.cos(), $param.sin(), 0.0],
            [0.0, -$param.sin(), $param.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ]
    };
    [$param:expr, z] => {
        [
            [$param.cos(), $param.sin(), 0.0, 0.0],
            [-$param.sin(), $param.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ]
    };
    [$param:expr, y] => {
        [
            [$param.cos(), 0.0, -$param.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [$param.sin(), 0.0, $param.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0 as f32],
        ]
    };
    ($x: expr, $y: expr, $z: expr) => {
        [
            [$z.cos() * $y.cos(), $z.cos() * $y.sin() * $x.sin() - $z.sin() * $x.cos(), $z.cos() * $y.sin() * $x.cos() + $z.sin() * $x.sin(), 0.0],
            [$z.sin() * $y.cos(), $z.sin() * $y.sin() * $x.sin() + $z.cos() * $x.cos(), $z.sin() * $y.sin() * $x.cos() - $z.cos() * $x.sin(), 0.0],
            [-$y.sin(), $y.cos() * $x.sin(), $x.cos() * $y.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ]
    }
}


/// Creates a matrix that scales the object by the given amount
#[macro_export]
macro_rules! scale {
    [$x: expr] => {
        [
            [$x, 0., 0., 0.],
            [0., $x, 0., 0.],
            [0., 0., $x, 0.],
            [0., 0., 0., 1.0f32],
        ]
    };
}

/// Creates the identity matrix, used for convenience
#[macro_export]
macro_rules! identity {
    () => {
        [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.0f32],
        ]
    };
}

/// Function that generates the perspective matrix
pub fn perspective_matrix(target: &Frame) -> [[f32; 4]; 4] {
    let (width, height) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;

    const FOV: f32 = PI / 3.0;
    const ZFAR: f32 = 1024.0;
    const ZNEAR: f32 = 0.1;

    let f: f32 = 1.0 / (FOV / 2.0).tan();

    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (ZFAR + ZNEAR) / (ZFAR - ZNEAR), 1.0],
        [0.0, 0.0, -(2.0 * ZFAR * ZNEAR) / (ZFAR - ZNEAR), 0.0],
    ]
}
