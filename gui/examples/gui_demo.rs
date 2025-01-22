//! GUI library usage example

use std::{cell::RefCell, rc::Rc};

use builder::{BuildFromCfg, Config};
use error_stack::{Result, ResultExt};
use gui::{
    manager::{widget::Widget, Manager},
    widget::{Builder, Button, Flag, Label, Panel},
};
use renderer::Drawable;
use runtime::Runtime;
use scene::{event::Event, Scene};

/// Window scale
const WINDOW_SCALE: u32 = 50;
/// Window hight
const WINDOW_H: u32 = 16 * WINDOW_SCALE;
/// Window width
const WINDOW_W: u32 = 9 * WINDOW_SCALE;

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

fn main() {
    if let Err(e) = run() {
        println!("{e:?}");
    }
}

fn run() -> Result<(), Error> {
    let runtime =
        Runtime::new((WINDOW_H, WINDOW_W)).change_context(Error::msg("Failed to init runtime"))?;
    let mut builder = scene::Builder::new();
    builder.reg_builder("main", MainScene::build);

    let scene_cfg =
        Config::new("./gui_demo.json").change_context(Error::msg("Failed to load scene config"))?;
    runtime.run(&builder, scene_cfg).change_context(Error::msg("Runtime error"))?;
    Ok(())
}

struct MainScene {
    gui: Manager,
    cursor_pos_label: Rc<RefCell<Label>>,
}

impl Scene for MainScene {
    fn handle_event(&mut self, e: scene::event::Event) -> Result<(), scene::Error> {
        if let Event::MouseMove(x, y) = e {
            self.cursor_pos_label.borrow_mut().set_text(format!("Cursor pos: ({x}, {y})"));
        }
        self.gui.handle_event(e).change_context(scene::Error::msg("Failed to update gui"))?;
        Ok(())
    }
}

impl Drawable for MainScene {
    fn draw(&self, renderer: &mut dyn renderer::Renderer) {
        self.gui.draw(renderer);
    }
}

impl BuildFromCfg<Box<dyn Scene>> for MainScene {
    fn build(
        mut cfg: Config,
        res: &mut dyn resources::Manger,
    ) -> Result<Box<dyn Scene>, builder::Error> {
        let gui_cfg = cfg
            .take::<Config>("gui")
            .change_context(builder::Error::msg("Failed to build scene GUI"))?;
        let gui = Manager::new(&Builder::default(), res, gui_cfg)
            .change_context(builder::Error::msg("Failed to init GUI manager"))?;

        let err = || builder::Error::msg("Failed to build scene, required widget not fount");
        gui.get_by_id_cast::<Panel>("middle_panel").change_context_lazy(err)?;
        let btn = gui.get_by_id_cast::<Button>("hello_button").change_context_lazy(err)?;
        btn.borrow_mut().click_cb(|button| {
            println!("Button \"{}\" clicked!", button.get_id());
        });
        let cursor_pos_label =
            gui.get_by_id_cast::<Label>("cursor_pos").change_context_lazy(err)?;

        let flag_state = gui.get_by_id_cast::<Label>("flag_state").change_context_lazy(err)?;
        let flag = gui.get_by_id_cast::<Flag>("hello_flag").change_context_lazy(err)?;
        flag.borrow_mut().change_state_cb(move |flag, state| {
            flag_state.borrow_mut().set_text(format!("Flag state: {}", state));
            println!("Flag \"{}\" change state: {}", flag.get_id(), state);
        });

        Ok(Box::new(Self { gui, cursor_pos_label }))
    }
}
