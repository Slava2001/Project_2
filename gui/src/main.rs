use error_stack::Result;
use glutin_window::GlutinWindow as Window;
use graphics::{clear, line, Context, DrawState, Rectangle, Transformed};
use gui::manager::input_event::{self, InputEvent};
use gui::manager::Manager;
use gui::renderer::vec2::Vec2f;
use gui::renderer::{color::Color, rect::Rect, Drawble, Renderer};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{Button, MouseCursorEvent, PressEvent, ReleaseEvent};

const WINDOW_H: f64 = 480.0;
const WINDOW_W: f64 = 480.0;

fn main() {
    if let Err(e) = run() {
        println!("{e:?}");
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct Error(String);
impl Error {
    fn msg<T: Into<String>>(msg: T) -> Self {
        Error(msg.into())
    }
}

struct PistonRenderer<'a> {
    g: &'a mut GlGraphics,
    ctx: Vec<Context>,
}

impl<'a> Renderer for PistonRenderer<'a> {
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
}

fn run() -> Result<(), Error> {
    let mut window: Window = WindowSettings::new("GUI Demo", [WINDOW_H, WINDOW_W])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .map_err(|_| Error::msg("Failed to init window"))?;
    let mut gl = GlGraphics::new(OpenGL::V3_2);
    let event_settings = EventSettings::new();
    let mut events = Events::new(event_settings);

    let mut gui = Manager::new(());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear([1.0; 4], g);
                let mut renderer = PistonRenderer { ctx: vec![c], g };
                gui.draw(&mut renderer);
            });
        }

        if let Some(args) = e.mouse_cursor_args() {
            gui.handle_event(InputEvent::MouseMove(args[0], args[1]));
        }
        if let Some(Button::Mouse(args)) = e.press_args() {
            let btn = match args {
                piston::MouseButton::Left => input_event::MouseButton::Left,
                piston::MouseButton::Right => input_event::MouseButton::Right,
                _ => input_event::MouseButton::Middle,
            };
            gui.handle_event(InputEvent::MousePress(btn));
        }
        if let Some(Button::Mouse(args)) = e.release_args() {
            let btn = match args {
                piston::MouseButton::Left => input_event::MouseButton::Left,
                piston::MouseButton::Right => input_event::MouseButton::Right,
                _ => input_event::MouseButton::Middle,
            };
            gui.handle_event(InputEvent::MouseRelease(btn));
        }
    }
    Ok(())
}
