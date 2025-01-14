mod renderer;
mod resmgr;

use error_stack::{Result, ResultExt};
use glutin_window::GlutinWindow as Window;
use graphics::clear;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{MouseCursorEvent, PressEvent, ReleaseEvent};
use renderer::Renderer;
use resmgr::ResMngr;
use crate::{event::{Event, MouseButton}, Scene};

/// Runtime error
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message
    fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

pub struct Runtime {
    window: Window,
    gl: GlGraphics,
    events: Events,
    pub resources: ResMngr,
}

impl Runtime {
    pub fn new(window_size: (u32, u32)) -> Result<Self, Error> {
        let window: Window = WindowSettings::new("GUI Demo", window_size)
            .graphics_api(OpenGL::V3_2)
            .build()
            .map_err(|_| Error::msg("Failed to init window"))?;
        let gl = GlGraphics::new(OpenGL::V3_2);
        let event_settings = EventSettings::new();
        let events = Events::new(event_settings);
        let resources = ResMngr::new();
        Ok(Self { window, gl, events, resources })
    }

    pub fn run(mut self, mut scene: Box<dyn Scene>) -> Result<(), Error> {
        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.gl.draw(args.viewport(), |c, g| {
                    clear([1.0; 4], g);
                    let mut renderer = Renderer { ctx: vec![c], g, res: &mut self.resources };
                    scene.draw(&mut renderer);
                });
            }

            let event = e.mouse_cursor_args().map_or_else(
                || {
                    if let Some(piston::Button::Mouse(args)) = e.press_args() {
                        match args {
                            piston::MouseButton::Left => Some(MouseButton::Left),
                            piston::MouseButton::Right => Some(MouseButton::Right),
                            piston::MouseButton::Middle => Some(MouseButton::Middle),
                            _ => None,
                        }
                        .map(Event::MousePress)
                    } else if let Some(piston::Button::Mouse(args)) = e.release_args() {
                        match args {
                            piston::MouseButton::Left => Some(MouseButton::Left),
                            piston::MouseButton::Right => Some(MouseButton::Right),
                            piston::MouseButton::Middle => Some(MouseButton::Middle),
                            _ => None,
                        }
                        .map(Event::MouseRelease)
                    } else {
                        None
                    }
                },
                |args|Some(Event::MouseMove(args[0], args[1]))
            );
            if let Some(e) = event {
                scene.handle_event(e).change_context(Error::msg("Scene failed to handle event"))?;
            }
        }
        Ok(())
    }
}
