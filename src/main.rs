mod shapes;
mod event_handler;

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
use event_handler::EventHandler;

type Light = [f32; 3];

//Starts the window and the event loop
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

/// Main function
fn main() {
    let (event_loop, display) = match start_opengl("First", None) {
        (event_loop, Ok(display)) => (event_loop, display),
        (_, Err(e)) => panic!("Could not create window: {e}"),
    };

    //let moon_texture = load_tex!(display, "imgs/2k_venus_surface.jpg", jpeg);
    /// Loads the earth texture
    let earth_texture = load_tex!(display, "imgs/2k_earth_daymap.jpg", jpeg);

    /// Initializes the earth object
    let earth = shapes::sphere::SphereBuilder::new()
        .radius(1.0)
        .texture(earth_texture)
        .build(&display);

    /// Initializes the moon object
    let moon = shapes::sphere::SphereBuilder::new()
        .radius(0.1)
        // .texture(moon_texture)
        .color([0.5; 3])
        .build(&display);

    /// Initializes the saturn object
    let saturn = shapes::ring_planet::RingPlanet::new(
        &display,
        1.3,
        shapes::sphere::SphereBuilder::new()
            .radius(1.0)
            .color([0.0, 0.3, 0.7])
            .build(&display),
    );

    ///Initalizes the asteroid object
    let asteroid = shapes::cube::CubeBuilder::new()
        .size(0.5)
        .color([0.2; 3])
        .build(&display);

    /// Initializes the sky
    let mut sky = shapes::sky::Sky::new(&display);

    let star = shapes::star::Star::new(&display);

    /// Defining the draw parameters
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

    /// Render runtime
    let mut angle = (0..360)
        .map(|i| (i as f32).to_radians())
        .cycle();

    let mut size = (0..240) // frames
        .map(|i| (i - 120) as f32 * 0.3 / 240.0 + 0.4)
        .cycle();

    /// Initializes the event handler
    let mut event_handler = EventHandler::new(0.15, 0.4, PI, 0., 0., [0.0, 0.0, 1.0], [-2.0, 1.0, 1.0], [0.0, 1.0, 0.0]);

    event_loop.run(move |ev, _, cf| {
        let a = angle.next().unwrap();
        let s = size.next().unwrap();
        let mut target = display.draw();
        target.clear_color_and_depth((0., 0., 0., 1.), 1.);

        set_wait(cf, 16_666_667);

        /// Handles the events
        event_handler.handle_event(ev, cf);

        let perspective = matrices::perspective_matrix(&mut target);
        let EventHandler {
            grow,
            tilt,
            spin,
            translate_x,
            translate_y,
            direction,
            position,
            up
        } = event_handler;

        /// Draws the earth
        earth.draw(
            &mut target,
            &draw_params,
            Transform {
                rotate_self: [0.0, spin, 0.0],
                scale: 0.3,
                view: [direction, position, up],
                ..Default::default()
            },
        );

        /// Draws the moon
        moon.draw(
            &mut target,
            &draw_params,
            Transform {
                translation: [-0.8, 0.0, 0.0],
                rotate_self: [0.0, a, 0.0],
                rotation: [0.0, a, a.cos() * tilt],
                view: [direction, position, up],
                ..Default::default()
            },
        );

        /// Draws saturn
        saturn.draw(
            &mut target,
            &draw_params,
            Transform {
                translation: [-0.7, 0.7, 0.0],
                scale: grow,
                rotate_self: [0.0, -a, -0.4],
                view: [direction, position, up],
                ..Default::default()
            },
        );

        /// Draws the asteroid
        asteroid.draw(
            &mut target,
            &draw_params,
            Transform {
                translation: [0.5 + translate_x, 0.5 + translate_y, 0.5],
                rotate_self: [0.0, a, 0.2],
                scale: 0.25,
                view: [direction, position, up],
                ..Default::default()
            },
        );

        /// Draws the sky
        sky.draw(&mut target, &draw_params);

        /// Draws the star
        star.draw(&mut target, &draw_params, Transform {
            translation: [0.8, -0.5, 0.0],
            rotate_self: [0.0, 0.0, a/6.0],
            scale: 0.3,
            ..Default::default()
        });

        target.finish().unwrap();
    })
}

/// Defines the wait time for the next frame
fn set_wait(cf: &mut ControlFlow, nanos: u64) {
    let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(nanos);
    *cf = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
}

