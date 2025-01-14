//! GUI library usage example

use std::{cell::RefCell, rc::Rc};

use config::{Config, File};
use error_stack::{Result, ResultExt};
use gui::{
    manager::{
        widget::{builder::Builder, Widget},
        Manager,
    },
    widget::{Button, Flag, Label, Panel},
};
use renderer::Drawable;
use scene::{event::Event, runtime::Runtime, Scene};

/// Window hight
const WINDOW_H: u32 = 480;
/// Window width
const WINDOW_W: u32 = 480;

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
    let mut runtime =
        Runtime::new((WINDOW_H, WINDOW_W)).change_context(Error::msg("Failed to init runtime"))?;
    let scene = MainScene::new(&mut runtime.resources)
        .change_context(Error::msg("Failed to init main scene"))?;
    runtime.run(Box::new(scene)).change_context(Error::msg("Runtime error"))?;
    Ok(())
}

struct MainScene {
    gui: Manager,
    cursor_pos_lable: Rc<RefCell<Label>>,
}

fn get_gui_el<T: 'static>(gui: &gui::manager::Manager, id: &str) -> Result<Rc<RefCell<T>>, Error> {
    Ok(gui
        .get_by_id(id)
        .ok_or_else(|| Error::msg(format!("Required GUI element \"{id}\" not found")))?
        .try_cast::<T>()
        .ok_or(Error::msg(format!("GUI element \"{id}\" has unexpected type")))?)
}

impl MainScene {
    fn new(res: &mut dyn resources::Manger) -> Result<Self, Error> {
        let gui_cfg = Config::builder()
            .add_source(File::with_name("./gui_demo.json"))
            .build()
            .change_context(Error::msg("Failed to build GUI config"))?
            .try_deserialize::<config::Map<String, config::Value>>()
            .change_context(Error::msg("Failed to deserialize GUI config as table"))?;
        let gui = Manager::new(&Builder::default(), res, gui_cfg)
            .change_context(Error::msg("Failed to init GUI manager"))?;
        get_gui_el::<Panel>(&gui, "middle_panel")?;
        get_gui_el::<Button>(&gui, "hello_button")?.borrow_mut().click_cb(|button| {
            println!("Button \"{}\" clicked!", button.get_id());
        });
        get_gui_el::<Label>(&gui, "debug_label_1")?.borrow_mut().set_text("Debug text:");
        let cursor_pos_lable = get_gui_el::<Label>(&gui, "debug_label_2")?;
        let debug_label_3 = get_gui_el::<Label>(&gui, "debug_label_3")?;
        get_gui_el::<Flag>(&gui, "hello_flag")?.borrow_mut().change_state_cb(move |flag, state| {
            debug_label_3.borrow_mut().set_text(format!("Flag state: {}", state));
            println!("Flag \"{}\" change state: {}", flag.get_id(), state);
        });
        Ok(Self { gui, cursor_pos_lable })
    }
}

impl Scene for MainScene {
    fn handle_event(&mut self, e: scene::event::Event) -> Result<(), scene::Error> {
        if let Event::MouseMove(x, y) = e {
            self.cursor_pos_lable.borrow_mut().set_text(format!("Cursor pos: ({x}, {y})"));
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
