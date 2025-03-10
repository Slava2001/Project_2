//! Main menu scene.

use std::{cell::RefCell, rc::Rc};

use builder::{config::Config, BuildFromCfg};
use error_stack::ResultExt;
use gui::{
    manager::{widget::Widget, Manager as GuiManager},
    widget::{Builder as GuiBuilder, Button, Flag, Graph, Textbox},
};
use renderer::Drawable;
use scene::{event::Event, Scene};

/// Main menu scene.
pub struct MainMenu {
    /// Main menu GUI.
    gui: GuiManager,
    /// Is need to load next scene.
    next_scene: Rc<RefCell<bool>>,
    /// Next scene config.
    cfg: Config,
    /// Graph for cursor x.
    cursor_x: Rc<RefCell<Graph>>,
    /// Graph for cursor y.
    cursor_y: Rc<RefCell<Graph>>,
}

impl Scene for MainMenu {
    fn handle_event(
        &mut self,
        e: scene::event::Event,
        state: &mut dyn scene::State,
    ) -> error_stack::Result<(), scene::Error> {
        self.gui.handle_event(e.clone()).change_context(scene::Error::msg("Gui failed"))?;
        if let Event::MouseMove(x, y) = e {
            self.cursor_x.borrow_mut().push(x);
            self.cursor_y.borrow_mut().push(y);
        }
        if *self.next_scene.borrow() {
            state
                .load_next_scene(
                    self.cfg
                        .take::<Config>("next_scene_cfg")
                        .change_context(scene::Error::msg("Next scene config not found"))?,
                )
                .change_context(scene::Error::msg("Failed to request load next scene"))?;
        }
        Ok(())
    }
}

impl Drawable for MainMenu {
    fn draw(&self, renderer: &mut dyn renderer::Renderer) {
        self.gui.draw(renderer);
    }
}

impl BuildFromCfg<Box<dyn Scene>> for MainMenu {
    fn build(
        mut cfg: Config,
        res: &mut dyn resources::Manager,
    ) -> error_stack::Result<Box<dyn Scene>, builder::Error> {
        let gui_cfg = cfg
            .take::<Config>("gui")
            .change_context(builder::Error::msg("Failed to build scene GUI"))?;
        let gui = GuiManager::new(&GuiBuilder::default(), res, gui_cfg)
            .change_context(builder::Error::msg("Failed to init GUI manager"))?;
        let next_scene = Rc::new(RefCell::new(false));
        let next_scene_clone = next_scene.clone();
        gui.get_by_id_cast::<Button>("change_scene")
            .change_context(builder::Error::msg("Failed to find change scene button"))?
            .borrow_mut()
            .click_cb(move |_| *next_scene_clone.borrow_mut() = true);
        let cursor_x = gui
            .get_by_id_cast::<Graph>("cursor_x")
            .change_context(builder::Error::msg("Failed to find graph for cursor x"))?;
        let cursor_y = gui
            .get_by_id_cast::<Graph>("cursor_y")
            .change_context(builder::Error::msg("Failed to find graph for cursor y"))?;
        {
            let cursor_x = cursor_x.clone();
            let cursor_y = cursor_y.clone();
            let text_1 = gui
                .get_by_id_cast::<Textbox>("input_txt_1")
                .change_context(builder::Error::msg("Failed to find textbox 1"))?;
            gui.get_by_id_cast::<Flag>("hello_flag")
                .change_context(builder::Error::msg("Failed to find change scene button"))?
                .borrow_mut()
                .change_state_cb(move |_, state| {
                    cursor_x.borrow_mut().set_visible_flag(state);
                    cursor_y.borrow_mut().set_visible_flag(state);
                    text_1.borrow_mut().set_visible_flag(state);
                });
        }
        Ok(Box::new(Self { gui, next_scene, cfg, cursor_x, cursor_y }))
    }
}
