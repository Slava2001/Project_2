//! Simple runtime implementation.

mod renderer;
mod resmgr;

use builder::Config;
use error_stack::{ensure, Result, ResultExt};
use glutin_window::GlutinWindow as Window;
use graphics::clear;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{MouseCursorEvent, PressEvent, ReleaseEvent};
use renderer::Renderer;
use resmgr::ResMngr;
use scene::event::{Event, MouseButton};

/// Runtime error.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message.
    fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

/// Simple runtime implementation.
pub struct Runtime {
    /// Window.
    window: Window,
    /// OpenGL data.
    gl: GlGraphics,
}

impl Runtime {
    /// Creates new runtime instance.
    ///
    /// # Errors
    /// Return error if failed to init window.
    pub fn new(window_name: &str, window_size: (u32, u32)) -> Result<Self, Error> {
        let window: Window = WindowSettings::new(window_name, window_size)
            .graphics_api(OpenGL::V3_2)
            .build()
            .map_err(|_| Error::msg("Failed to init window"))?;
        let gl = GlGraphics::new(OpenGL::V3_2);
        Ok(Self { window, gl })
    }

    /// Run runtime cycle.
    ///
    /// # Errors
    /// Return error if some scene failed.
    pub fn run(mut self, scene_builder: &scene::Builder, scene_cfg: Config) -> Result<(), Error> {
        let mut events = Events::new(EventSettings::new());
        let mut state = State { next_scene: None, res: ResMngr::new() };
        let mut scene = scene_builder
            .build(scene_cfg, &mut state.res)
            .change_context(Error::msg("Failed to create first scene"))?;

        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.gl.draw(args.viewport(), |c, g| {
                    clear([1.0; 4], g);
                    let mut renderer = Renderer { ctx: vec![c], g, res: &mut state.res };
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
                |args| Some(Event::MouseMove(args[0], args[1])),
            );
            if let Some(e) = event {
                scene
                    .handle_event(e, &mut state)
                    .change_context(Error::msg("Scene failed to handle event"))?;
            }
            if let Some(cfg) = state.next_scene.take() {
                scene = scene_builder
                    .build(cfg, &mut state.res)
                    .change_context(Error::msg("Failed to load next scene"))?;
            }
        }
        Ok(())
    }
}

/// Scene state.
struct State {
    /// Next scene config.
    next_scene: Option<Config>,

    /// Resource manager.
    res: ResMngr,
}

impl scene::State for State {
    fn load_next_scene(&mut self, cfg: Config) -> Result<(), scene::Error> {
        ensure!(self.next_scene.is_none(), scene::Error::msg("Next scene already specified"));
        self.next_scene = Some(cfg);
        Ok(())
    }

    fn get_resources_manager(&mut self) -> &dyn resources::Manger {
        &mut self.res
    }
}
