mod shapes;

#[macro_use]
extern crate glium;

extern crate image;

use crate::glutin::event::KeyboardInput;
use crate::glutin::event_loop::ControlFlow;
use crate::shapes::{DynDrawble, StaticDrawble, Transform};
use glium::backend::glutin::DisplayCreationError;
use glium::glutin::event::Event;
use glium::glutin::event_loop::EventLoop;
use glium::texture::*;
use glium::{glutin, Display, Surface};
use glutin::event::ElementState;
use glutin::event::VirtualKeyCode;
use glutin::event::WindowEvent;
use shapes::matrices;
use std::any::Any;
use std::f32::consts::PI;

type Light = [f32; 3];

fn start_opengl(
    title: &str,
    mut size: Option<(u32, u32)>,
) -> (EventLoop<()>, Result<Display, DisplayCreationError>) {
    let size = size.get_or_insert((400, 400));

    let event_loop = glutin::event_loop::EventLoop::new();

    let window = glutin::window::WindowBuilder::new()
        .with_title(title)
        .with_inner_size(glutin::dpi::LogicalSize::new(size.0, size.1));

    let context = glutin::ContextBuilder::new().with_depth_buffer(24);

    let display = glium::Display::new(window, context, &event_loop);
    (event_loop, display)
}

fn main() {
    let (event_loop, display) = match start_opengl("First", None) {
        (event_loop, Ok(display)) => (event_loop, display),
        (_, Err(e)) => panic!("Could not create window: {e}"),
    };

    let moon_texture = load_tex!(display, "imgs/2k_venus_surface.jpg", jpeg);
    let earth_texture = load_tex!(display, "imgs/2k_earth_daymap.jpg", jpeg);

    let earth = shapes::sphere::SphereBuilder::new()
        .radius(1.0)
        .texture(earth_texture)
        .build(&display);

    let moon = shapes::sphere::SphereBuilder::new()
        .radius(0.1)
        // .texture(moon_texture)
        .color([0.5; 3])
        .build(&display);

    let saturn = shapes::ring_planet::RingPlanet::new(
        &display,
        1.3,
        shapes::sphere::SphereBuilder::new()
            .radius(1.0)
            .color([0.0, 0.3, 0.7])
            .build(&display),
    );

    let asteroid = shapes::cube::CubeBuilder::new()
        .size(0.5)
        .color([0.2; 3])
        .build(&display);

    let mut sky = shapes::sky::Sky::new(&display);

    let triangle = shapes::triangle::TriangleBuilder::new().build(&display);

    let draw_params = glium::draw_parameters::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            range: (0.0, 0.9),
            ..Default::default()
        },
        // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    // Render runtime
    let mut angle = (0..360)
        .map(|i| (i as f32).to_radians())
        .cycle();

    let mut size = (0..240) // frames
        .map(|i| (i - 120) as f32 * 0.3 / 240.0 + 0.4)
        .cycle();

    let mut mat_params = MatrixParams::new(0.15, 0.4, PI, 0., 0.);

    event_loop.run(move |ev, _, cf| {
        let a = angle.next().unwrap();
        let s = size.next().unwrap();
        let mut target = display.draw();
        target.clear_color_and_depth((0., 0., 0., 1.), 1.);

        set_wait(cf, 16_666_667);

        event_handle(ev, cf,  &mut mat_params);

        let perspective = matrices::perspective_matrix(&mut target);
        let MatrixParams{grow, tilt, spin, translate_x, translate_y} = mat_params;

        earth.draw(
            &mut target,
            &draw_params,
            Transform {
                rotate_self: [0.0, spin, 0.0],
                scale: 0.3,
                ..Default::default()
            },
        );

        moon.draw(
            &mut target,
            &draw_params,
            Transform {
                translation: [-0.8, 0.0, 0.0],
                rotate_self: [0.0, a, 0.0],
                rotation: [0.0, a, a.cos() * tilt],
                ..Default::default()
            },
        );

        saturn.draw(
            &mut target,
            &draw_params,
            Transform {
                translation: [-0.7, 0.7, 0.0],
                scale: grow,
                rotate_self: [0.0, -a, -0.4],
                ..Default::default()
            },
        );

        asteroid.draw(
            &mut target,
            &draw_params,
            Transform {
                translation: [0.5 + translate_x, 0.5 + translate_y, 0.5],
                rotate_self: [0.0, a, 0.2],
                scale: 0.25,
                ..Default::default()
            },
        );

        sky.draw(&mut target, &draw_params);

        triangle.draw(&mut target, &draw_params, Transform {
            // translation: [0.0, 0.0, -1.0],
            ..Default::default()
        });

        target.finish().unwrap();
    })
}

fn set_wait(cf: &mut ControlFlow, nanos: u64) {
    let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(nanos);
    *cf = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
}

pub struct MatrixParams
{
    pub grow: f32,
    pub tilt: f32,
    pub spin: f32,
    pub translate_x: f32,
    pub translate_y: f32,
}

impl MatrixParams
{
    fn new(grow: f32, tilt: f32, spin: f32, translate_x: f32, translate_y: f32) -> Self
    {
        MatrixParams{grow, tilt, spin, translate_x, translate_y}
    }
}

fn event_handle(ev: Event<()>, cf: &mut ControlFlow, MatrixParams{ref mut grow, ref mut tilt, ref mut spin, ref mut translate_x, ref mut translate_y}: &mut MatrixParams) {
    match ev {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => {
                let KeyboardInput {
                    state,
                    virtual_keycode,
                    ..
                } = input;
                let virtual_keycode = if let Some(code) = virtual_keycode {
                    code
                } else {
                    return;
                };
                const STEP: f32 = 0.05;
                if let state = ElementState::Pressed {
                    match virtual_keycode {
                        VirtualKeyCode::W => *grow += STEP,
                        VirtualKeyCode::A => *tilt -= STEP,
                        VirtualKeyCode::S => *grow -= STEP,
                        VirtualKeyCode::D => *tilt += STEP,
                        VirtualKeyCode::J => *spin += STEP,
                        VirtualKeyCode::K => *spin -= STEP,
                        VirtualKeyCode::Right => *translate_x += STEP,
                        VirtualKeyCode::Left => *translate_x -= STEP,
                        VirtualKeyCode::Up => *translate_y += STEP,
                        VirtualKeyCode::Down => *translate_y -= STEP,
                        _ => (),
                    }
                }

                if *grow < 0.02 {
                    *grow = 0.01;
                } else if *grow > 1.0 {
                    *grow = 1.0;
                }

                if *tilt > 1.0 {
                   *tilt = 1.0;
                } else if *tilt < -1.0 {
                    *tilt = -1.0;
                }

            }

            glutin::event::WindowEvent::CloseRequested => {
                *cf = glutin::event_loop::ControlFlow::Exit;
                return;
            }
            _ => return,
        },
        glutin::event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => (),
            glutin::event::StartCause::Init => (),
            _ => return,
        },
        _ => return,
    }
}
