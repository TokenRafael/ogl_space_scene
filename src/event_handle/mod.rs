use glium::glutin::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::ControlFlow;
use crate::{glutin, KeyboardInput};

/// Struct that stores the parameters used in the transformation of the keyboard input
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
    pub fn new(grow: f32, tilt: f32, spin: f32, translate_x: f32, translate_y: f32) -> Self
    {
        MatrixParams{grow, tilt, spin, translate_x, translate_y}
    }
}

/// Function that handles the keyboard input
pub fn event_handle(ev: Event<()>, cf: &mut ControlFlow,
                    MatrixParams{
                        ref mut grow,
                        ref mut tilt,
                        ref mut spin,
                        ref mut translate_x,
                        ref mut translate_y
                    }: &mut MatrixParams) {
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
                /// If the key is pressed, the value is changed
                if let state = ElementState::Pressed {
                    /// Parses the pressed key and changes the value
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
