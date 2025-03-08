//! This is a simple implementation of the runtime, it is only needed to test components
//! dependent on it, in the future it will be rewritten using a low-level OpenGL API,
//! so now it is full of crutches and questionable code.

mod renderer;
mod resmgr;

use ::renderer::Drawable;
use builder::config::Config;
use error_stack::{ensure, Result, ResultExt};
use glutin_window::GlutinWindow as Window;
use graphics::clear;
use gui::widget::{Graph, Label, Slider};
use gui::{manager::Manager as GuiMngr, widget::Builder as GuiBuilder};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston::{Button, EventLoop, Key, Motion, UpdateEvent};
use renderer::Renderer;
use resmgr::ResMngr;
use resources::{FontId, TextureId};
use scene::event::{self, Event, KeyCode, MouseButton};
use scene::TimeTick;
use std::time::{Duration, Instant};

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
    /// Runtime gui.
    gui: GuiMngr,
    /// Runtime gui recourses.
    gui_res: ResMngr,
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
        let mut gui_res = ResMngr::new();
        {
            // load default font
            let id = FontId(gui_res.fonts.len());
            let mut settings = TextureSettings::new();
            settings.set_filter(opengl_graphics::Filter::Nearest);
            let cache = GlyphCache::from_bytes(
                include_bytes!("./ubuntu.mono.ttf"),
                (),
                TextureSettings::new(),
            )
            .map_err(|e| Error::msg(format!("Failed to load font: {e:?}")))?;
            gui_res.fonts.push(cache);
            gui_res.fonts_map.insert("default".into(), id);
        }
        {
            // load slider texture
            let id = TextureId(gui_res.textures.len());
            let mut settings = TextureSettings::new();
            settings.set_filter(opengl_graphics::Filter::Nearest);
            gui_res.textures.push(
                Texture::from_bytes(include_bytes!("./slider.png"), &settings)
                    .map_err(|e| Error::msg(format!("Failed to load texture: {e}")))?,
            );
            gui_res.textures_map.insert("slider_texture".into(), id);
        }
        let cfg = Config::from_json(include_str!("./gui_cfg.json"))
            .change_context(Error::msg("Failed to create runtime gui config"))?;
        let gui = GuiMngr::new(&GuiBuilder::default(), &mut gui_res, cfg)
            .change_context(Error::msg("Failed to load runtime gui"))?;
        Ok(Self { window, gl, gui, gui_res })
    }

    /// Run runtime cycle.
    ///
    /// # Errors
    /// Return error if some scene failed.
    pub fn run(mut self, scene_builder: &scene::Builder, scene_cfg: Config) -> Result<(), Error> {
        let root = self
            .gui
            .get_by_id("root")
            .ok_or_else(|| Error::msg("Failed to get runtime gui root"))?;
        let fps_graph = self
            .gui
            .get_by_id_cast::<Graph>("fps_graph")
            .change_context(Error::msg("Failed to get runtime fps graph"))?;
        let fps_label = self
            .gui
            .get_by_id_cast::<Label>("fps_label")
            .change_context(Error::msg("Failed to get runtime fps label"))?;
        let tps_label = self
            .gui
            .get_by_id_cast::<Label>("tps_label")
            .change_context(Error::msg("Failed to get runtime tps label"))?;
        let tps_slider = self
            .gui
            .get_by_id_cast::<Slider>("tps_slider")
            .change_context(Error::msg("Failed to get runtime tps slider"))?;

        let mut events = Events::new(EventSettings::new());
        events.bench_mode(true);
        events.max_fps(100);
        let mut state = State { next_scene: None, res: ResMngr::new(), exit: false };
        let mut scene = scene_builder
            .build(scene_cfg, &mut state.res)
            .change_context(Error::msg("Failed to create first scene"))?;

        let mut fps_counter = 0;
        let mut fps_timer = Instant::now();
        let mut tick_per_sec = tps_slider.borrow().get_value();

        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.gl.draw(args.viewport(), |c, g| {
                    clear([1.0; 4], g);
                    let mut renderer = Renderer { ctx: vec![c], g, res: &mut state.res };
                    scene.draw(&mut renderer);
                    let mut renderer = Renderer { ctx: vec![c], g, res: &mut self.gui_res };
                    self.gui.draw(&mut renderer);
                });

                fps_counter += 1;
                if fps_timer.elapsed() >= Duration::from_secs_f32(0.1) {
                    let fps = f64::from(fps_counter) / fps_timer.elapsed().as_secs_f64();
                    fps_graph.borrow_mut().push(fps);
                    fps_label.borrow_mut().set_text(&format!("fps: {}", fps.round()));
                    fps_counter = 0;
                    fps_timer = Instant::now();
                }
            }

            if let Some(e) = e.update_args() {
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                let dt = (e.dt * tick_per_sec).round() as TimeTick;
                scene
                    .handle_event(event::Event::TimeTick(dt), &mut state)
                    .change_context(Error::msg("Scene failed to handle update event"))?;
            }

            let event = convert_event(e);
            if let Some(e) = event {
                if matches!(e, Event::KeyPress(KeyCode::F1)) {
                    let is_visible = root.borrow().is_visible();
                    root.borrow_mut().set_visible_flag(!is_visible);
                }
                scene
                    .handle_event(e.clone(), &mut state)
                    .change_context(Error::msg("Scene failed to handle event"))?;
                self.gui
                    .handle_event(e)
                    .change_context(Error::msg("Failed to update runtime gui"))?;
                tick_per_sec = tps_slider.borrow().get_value().round();
                tps_label.borrow_mut().set_text(&format!("TPS: {tick_per_sec}"));
            }

            if let Some(cfg) = state.next_scene.take() {
                scene = scene_builder
                    .build(cfg, &mut state.res)
                    .change_context(Error::msg("Failed to load next scene"))?;
            }

            if state.exit {
                break;
            }
        }
        Ok(())
    }
}

/// Convert piston event to scene event.
fn convert_event(event: piston::Event) -> Option<Event> {
    match event {
        piston::Event::Input(input, _) => match input {
            piston::Input::Button(arg) => match arg.button {
                piston::Button::Keyboard(_) => match arg.button {
                    Button::Keyboard(Key::Escape) => Some(KeyCode::Escape),
                    Button::Keyboard(Key::Backspace) => Some(KeyCode::Backspace),
                    Button::Keyboard(Key::Tab) => Some(KeyCode::Tab),
                    Button::Keyboard(Key::F1) => Some(KeyCode::F1),
                    Button::Keyboard(Key::Return) => Some(KeyCode::Enter),
                    Button::Keyboard(Key::Up) => Some(KeyCode::ArrowUp),
                    Button::Keyboard(Key::Down) => Some(KeyCode::ArrowDown),
                    Button::Keyboard(Key::Right) => Some(KeyCode::ArrowRight),
                    Button::Keyboard(Key::Left) => Some(KeyCode::ArrowLeft),
                    Button::Keyboard(Key::Home) => Some(KeyCode::Home),
                    Button::Keyboard(Key::End) => Some(KeyCode::End),
                    Button::Keyboard(Key::Delete) => Some(KeyCode::Delete),
                    _ => None,
                }
                .map(|k| match arg.state {
                    piston::ButtonState::Press => Event::KeyPress(k),
                    piston::ButtonState::Release => Event::KeyRelease(k),
                }),
                piston::Button::Mouse(mouse_button) => match (mouse_button, arg.state) {
                    (piston::MouseButton::Left, piston::ButtonState::Press) => {
                        Some(Event::MousePress(MouseButton::Left))
                    }
                    (piston::MouseButton::Right, piston::ButtonState::Press) => {
                        Some(Event::MousePress(MouseButton::Right))
                    }
                    (piston::MouseButton::Middle, piston::ButtonState::Press) => {
                        Some(Event::MousePress(MouseButton::Middle))
                    }
                    (piston::MouseButton::Left, piston::ButtonState::Release) => {
                        Some(Event::MouseRelease(MouseButton::Left))
                    }
                    (piston::MouseButton::Right, piston::ButtonState::Release) => {
                        Some(Event::MouseRelease(MouseButton::Right))
                    }
                    (piston::MouseButton::Middle, piston::ButtonState::Release) => {
                        Some(Event::MouseRelease(MouseButton::Middle))
                    }
                    _ => None,
                },
                _ => None,
            },
            piston::Input::Move(Motion::MouseCursor(pos)) => Some(Event::MouseMove(pos.into())),
            piston::Input::Text(txt) => Some(Event::TextInput(txt)),
            piston::Input::Resize(arg) => Some(Event::Resize(
                (f64::from(arg.draw_size[0]), f64::from(arg.draw_size[1])).into(),
            )),
            _ => None,
        },
        _ => None,
    }
}

/// Scene state.
struct State {
    /// Next scene config.
    next_scene: Option<Config>,
    /// Resource manager.
    res: ResMngr,
    /// Terminate request.
    exit: bool,
}

impl scene::State for State {
    fn load_next_scene(&mut self, cfg: Config) -> Result<(), scene::Error> {
        ensure!(self.next_scene.is_none(), scene::Error::msg("Next scene already specified"));
        self.next_scene = Some(cfg);
        Ok(())
    }

    fn get_resources_manager(&mut self) -> &dyn resources::Manager {
        &mut self.res
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
