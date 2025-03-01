//! Game level scene.

use std::{cell::RefCell, path::PathBuf, rc::Rc};

use anim::anim::Anim;
use builder::{config::Config, BuildFromCfg};
use error_stack::ResultExt;
use gui::{
    manager::Manager as GuiManager,
    widget::{Builder as GuiBuilder, Button},
};
use renderer::Drawable;
use resources::TextureId;
use scene::{event::Event, Scene};

/// Game level scene.
pub struct Level {
    /// Level scene GUI.
    gui: GuiManager,
    /// Is need to return to main menu.
    menu_scene: Rc<RefCell<bool>>,
    /// Main menu config.
    cfg: Config,
    /// Player texture.
    player_texture: TextureId,
    /// Player animation.
    player_anim: Anim,
}

impl Scene for Level {
    fn handle_event(
        &mut self,
        e: Event,
        state: &mut dyn scene::State,
    ) -> error_stack::Result<(), scene::Error> {
        if let Event::TimeTick(dt) = e {
            self.player_anim.update(dt);
        }
        self.gui.handle_event(e).change_context(scene::Error::msg("Gui failed"))?;

        if *self.menu_scene.borrow() {
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

impl Drawable for Level {
    fn draw(&self, renderer: &mut dyn renderer::Renderer) {
        self.gui.draw(renderer);
        renderer.draw_img(&[100.0; 4].into(), self.player_texture, self.player_anim.get_rect());
    }
}

impl BuildFromCfg<Box<dyn Scene>> for Level {
    fn build(
        mut cfg: Config,
        res: &mut dyn resources::Manager,
    ) -> error_stack::Result<Box<dyn Scene>, builder::Error> {
        let gui_cfg = cfg
            .take::<Config>("gui")
            .change_context(builder::Error::msg("Failed to build scene GUI"))?;
        let gui = GuiManager::new(&GuiBuilder::default(), res, gui_cfg)
            .change_context(builder::Error::msg("Failed to init GUI manager"))?;
        let menu_scene = Rc::new(RefCell::new(false));
        let menu_scene_clone = menu_scene.clone();
        gui.get_by_id_cast::<Button>("change_scene")
            .change_context(builder::Error::msg("Failed to find change scene button"))?
            .borrow_mut()
            .click_cb(move |_| *menu_scene_clone.borrow_mut() = true);

        let path = cfg
            .take::<PathBuf>("player_anim_texture")
            .change_context(builder::Error::msg("Failed to init player texture"))?;
        res.load("texture", "player_anim_texture", &path)
            .change_context(builder::Error::msg("Failed to load player texture"))?;
        let player_texture = res
            .get_texture("player_anim_texture")
            .change_context(builder::Error::msg("Failed to find player texture"))?;
        let player_anim_cfg = cfg
            .take("player_anim")
            .change_context(builder::Error::msg("Failed to init player animation config"))?;
        let player_anim = Anim::new(player_anim_cfg)
            .change_context(builder::Error::msg("Failed to build player animation"))?;
        Ok(Box::new(Self { gui, menu_scene, cfg, player_texture, player_anim }))
    }
}
