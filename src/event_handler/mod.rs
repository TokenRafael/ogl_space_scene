use glium::glutin::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::ControlFlow;
use crate::{glutin, KeyboardInput};

/// Struct that handles the events of the window.
pub struct EventHandler {
    pub grow: f32,
    pub tilt: f32,
    pub spin: f32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub direction: [f32; 3],
    pub position: [f32; 3],
    pub up: [f32; 3],
}

impl EventHandler {
    pub fn new(grow: f32, tilt: f32, spin: f32, translate_x: f32, translate_y: f32, direction: [f32; 3], position: [f32; 3], up: [f32; 3]) -> Self {
        EventHandler {grow, tilt, spin, translate_x, translate_y, direction, position, up}
    }

    /// Method that handles the keyboard input
    pub fn handle_event(&mut self, ev: Event<()>, cf: &mut ControlFlow) {

        let EventHandler {
            ref mut grow,
            ref mut tilt,
            ref mut spin,
            ref mut translate_x,
            ref mut translate_y,
            ref mut direction,
            ref mut position,
            ref mut up,
        } = self;

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
                            VirtualKeyCode::Numpad8 => (*position)[1] -= STEP,
                            VirtualKeyCode::Numpad2 => (*position)[1] += STEP,
                            VirtualKeyCode::Numpad4 => (*position)[0] += STEP,
                            VirtualKeyCode::Numpad6 => (*position)[0] -= STEP,
                            VirtualKeyCode::Numpad5 => (*position)[2] += STEP,
                            VirtualKeyCode::NumpadSubtract => (*position)[2] -= STEP,
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
}

