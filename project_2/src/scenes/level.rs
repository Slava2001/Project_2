//! Game level scene.

use std::{cell::RefCell, rc::Rc};

use anim::{make_animator_cfg, Animator};
use builder::{config::Config, BuildFromCfg};
use error_stack::ResultExt;
use gui::{
    manager::Manager as GuiManager,
    widget::{Builder as GuiBuilder, Button},
};
use renderer::Drawable;
use scene::{event::Event, Scene};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum PlayerState {
    IdleR,
    IdleL,
    WalkR,
    WalkL,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum PlayerEvent {
    WalkR,
    WalkL,
    Stop,
    AnimFin,
}

/// Game level scene.
pub struct Level {
    /// Level scene GUI.
    gui: GuiManager,
    /// Is need to return to main menu.
    menu_scene: Rc<RefCell<bool>>,
    /// Main menu config.
    cfg: Config,
    /// Player animation.
    player_anim: Animator<PlayerState, PlayerEvent>,
}

impl Scene for Level {
    fn handle_event(
        &mut self,
        e: Event,
        state: &mut dyn scene::State,
    ) -> error_stack::Result<(), scene::Error> {
        if let Event::TimeTick(dt) = e {
            self.player_anim.update(dt).change_context(scene::Error::msg("player_anim failed"))?;
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
        self.player_anim.draw(renderer);
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

        let animator_cfg = make_animator_cfg!(
            State_enum: PlayerState,
            Event_enum: PlayerEvent,
            Init_state: IdleR,
            Anim_fin_event: AnimFin,
            Anim_map:
                IdleR: "idle_r",
                IdleL: "idle_l",
                WalkR: "walk_r",
                WalkL: "walk_l"
            Transient_map:
                IdleR:
                    WalkL => WalkL,
                    WalkR => WalkR,
                    AnimFin => IdleR;
                IdleL:
                    WalkR => WalkR,
                    WalkL => WalkL;
                WalkR:
                    WalkL => WalkL,
                    AnimFin => WalkL,
                    Stop  => IdleR;
                WalkL:
                    WalkR => WalkR,
                    AnimFin => WalkR,
                    Stop  => IdleL
        );
        let anim_cfg = cfg.take("player_anim").change_context(builder::Error::msg(""))?;
        let player_anim =
            Animator::new(animator_cfg, anim_cfg, res).change_context(builder::Error::msg(""))?;
        Ok(Box::new(Self { gui, menu_scene, cfg, player_anim }))
    }
}
