//! GUI library usage example

use std::collections::HashMap;
use std::path::Path;

use config::{Config, File};
use error_stack::{bail, Result, ResultExt};
use glutin_window::GlutinWindow as Window;
use graphics::{clear, line, Context, DrawState, Image, Rectangle, Transformed};
use gui::manager::input_event::{self, InputEvent};
use gui::manager::widget::builder::Builder;
use gui::manager::Manager;
use gui::renderer::vec2::Vec2f;
use gui::renderer::Drawable;
use gui::renderer::{color::Color, rect::Rect, Renderer};
use gui::resources::{self, Manger, TextureId};
use gui::widget::Panel;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{Button, MouseCursorEvent, PressEvent, ReleaseEvent};

/// Window hight
const WINDOW_H: f64 = 480.0;
/// Window width
const WINDOW_W: f64 = 480.0;

fn main() {
    if let Err(e) = run() {
        println!("{e:?}");
    }
}

/// Simple error
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct Error(String);
impl Error {
    /// Make error from message
    fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

struct PistonResMngr {
    textures: Vec<Texture>,
    textures_map: HashMap<String, TextureId>,
}
impl PistonResMngr {
    pub fn new() -> Self {
        Self { textures: Vec::new(), textures_map: HashMap::new() }
    }
}

impl Manger for PistonResMngr {
    fn load(&mut self, kind: &str, name: &str, path: &str) -> Result<TextureId, resources::Error> {
        match kind {
            "texture" => {
                let id = TextureId(self.textures.len());
                let mut settings = TextureSettings::new();
                settings.set_filter(opengl_graphics::Filter::Nearest);
                self.textures.push(
                    Texture::from_path(Path::new(path), &settings).map_err(|e| {
                        resources::Error::msg(format!("Failed to load texture: {e}"))
                    })?,
                );
                self.textures_map.insert(name.into(), id);
                Ok(id)
            }
            _ => bail!(resources::Error::msg(format!(
                "Failed to load recourse: unexpected resource \
                type: \"{kind}\", name: \"{name}\", path: \"{path}\""
            ))),
        }
    }

    fn get_texture(&self, name: &str) -> Result<TextureId, resources::Error> {
        Ok(*self
            .textures_map
            .get(name)
            .ok_or(resources::Error::msg(format!("Failed to find texture: \"{name}\"")))?)
    }
}

/// Simple implementation of renderer
struct PistonRenderer<'a> {
    /// Gl graphics
    g: &'a mut GlGraphics,
    /// Contexts stack
    ctx: Vec<Context>,
    /// Resources
    res: &'a PistonResMngr,
}

impl Renderer for PistonRenderer<'_> {
    fn draw_rect(&mut self, rect: &Rect<f64>, color: &Color) {
        Rectangle::new(color.into()).draw(
            [rect.x, rect.y, rect.h, rect.w],
            &DrawState::default(),
            self.ctx.last().unwrap().transform,
            self.g,
        );
    }

    fn push_state(&mut self) {
        self.ctx.push(*self.ctx.last().unwrap());
    }

    fn pop_state(&mut self) {
        self.ctx.pop();
    }

    fn translate(&mut self, x: f64, y: f64) {
        let state = self.ctx.pop().unwrap().trans(x, y);
        self.ctx.push(state);
    }

    fn draw_line(&mut self, from: Vec2f, to: Vec2f, color: &Color) {
        line(
            color.into(),
            1.0,
            [from.x, from.y, to.x, to.y],
            self.ctx.last().unwrap().transform,
            self.g,
        );
    }

    fn draw_img(
        &mut self,
        rect: &Rect<f64>,
        texture: gui::resources::TextureId,
        texture_rect: &Rect<f64>,
    ) {
        Image::new()
            .rect([rect.x, rect.y, rect.h, rect.w])
            .src_rect(Into::<[f64; 4]>::into([
                texture_rect.x,
                texture_rect.y,
                texture_rect.h,
                texture_rect.w,
            ]))
            .draw(
                &self.res.textures[texture.0],
                &DrawState::default(),
                self.ctx.last().unwrap().transform,
                self.g,
            );
    }
}

/// main function
fn run() -> Result<(), Error> {
    let mut window: Window = WindowSettings::new("GUI Demo", [WINDOW_H, WINDOW_W])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .map_err(|_| Error::msg("Failed to init window"))?;
    let mut gl = GlGraphics::new(OpenGL::V3_2);
    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);

    let mut resources = PistonResMngr::new();

    let gui_cfg = Config::builder()
        .add_source(File::with_name("./gui_demo.json"))
        .build()
        .change_context(Error::msg("Failed to build GUI config"))?
        .try_deserialize::<config::Map<String, config::Value>>()
        .change_context(Error::msg("Failed to deserialize GUI config as table"))?;
    let mut gui = Manager::new(&Builder::default(), &mut resources, gui_cfg)
        .change_context(Error::msg("Failed to init GUI manager"))?;

    gui.get_by_id("middle_panel")
        .ok_or_else(|| Error::msg("Required GUI element \"middle_panel\" not found"))?
        .try_cast::<Panel>()
        .ok_or(Error::msg(
            "GUI element \"middle_panel\" has unexpected type. Expected: \"panel\"",
        ))?;

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear([1.0; 4], g);
                let mut renderer = PistonRenderer { ctx: vec![c], g, res: &resources };
                gui.draw(&mut renderer);
            });
        }

        let event = e.mouse_cursor_args().map_or_else(
            || {
                if let Some(Button::Mouse(args)) = e.press_args() {
                    match args {
                        piston::MouseButton::Left => Some(input_event::MouseButton::Left),
                        piston::MouseButton::Right => Some(input_event::MouseButton::Right),
                        piston::MouseButton::Middle => Some(input_event::MouseButton::Middle),
                        _ => None,
                    }
                    .map(InputEvent::MousePress)
                } else if let Some(Button::Mouse(args)) = e.release_args() {
                    match args {
                        piston::MouseButton::Left => Some(input_event::MouseButton::Left),
                        piston::MouseButton::Right => Some(input_event::MouseButton::Right),
                        piston::MouseButton::Middle => Some(input_event::MouseButton::Middle),
                        _ => None,
                    }
                    .map(InputEvent::MouseRelease)
                } else {
                    None
                }
            },
            |args| Some(InputEvent::MouseMove(args[0], args[1])),
        );
        if let Some(e) = event {
            gui.handle_event(e).change_context(Error::msg("GUI failed to handle event"))?;
        }
    }
    Ok(())
}
