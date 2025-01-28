//! Main menu scene.

use std::{cell::RefCell, rc::Rc};

use builder::{BuildFromCfg, Config};
use error_stack::ResultExt;
use gui::{
    manager::Manager as GuiManager,
    widget::{Builder as GuiBuilder, Button},
};
use renderer::Drawable;
use scene::Scene;

/// Main menu scene.
pub struct MainMenu {
    /// Main menu GUI.
    gui: GuiManager,

    /// Is need to load next scene.
    next_scene: Rc<RefCell<bool>>,

    /// Next scene config.
    cfg: Config,
}

impl Scene for MainMenu {
    fn handle_event(
        &mut self,
        e: scene::event::Event,
        state: &mut dyn scene::State,
    ) -> error_stack::Result<(), scene::Error> {
        self.gui.handle_event(e).change_context(scene::Error::msg("Gui failed"))?;
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
        mut cfg: builder::Config,
        res: &mut dyn resources::Manger,
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
        Ok(Box::new(Self { gui, next_scene, cfg }))
    }
}
