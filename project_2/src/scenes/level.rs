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
use scene::{
    event::{Event, KeyCode},
    Scene,
};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
#[allow(clippy::missing_docs_in_private_items)]
enum PlayerState {
    IdleR,
    IdleL,
    WalkR,
    WalkL,
    AttackR,
    AttackL,
    AttackWalkR,
    AttackWalkL,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
#[allow(clippy::missing_docs_in_private_items)]
enum PlayerEvent {
    WalkR,
    WalkL,
    WalkREnd,
    WalkLEnd,
    Attack,
    AttackEnd,
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

/// Convert scene event to player event.
const fn to_player_event(e: &Event) -> Option<PlayerEvent> {
    match e {
        Event::KeyPress(key_code) => match key_code {
            KeyCode::ArrowRight => Some(PlayerEvent::WalkR),
            KeyCode::ArrowLeft => Some(PlayerEvent::WalkL),
            KeyCode::Tab => Some(PlayerEvent::Attack),
            _ => None,
        },
        Event::KeyRelease(key_code) => match key_code {
            KeyCode::ArrowRight => Some(PlayerEvent::WalkREnd),
            KeyCode::ArrowLeft => Some(PlayerEvent::WalkLEnd),
            KeyCode::Tab => Some(PlayerEvent::AttackEnd),
            _ => None,
        },
        _ => None,
    }
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
        if let Some(e) = to_player_event(&e) {
            self.player_anim
                .handle_event(e)
                .change_context(scene::Error::msg("Failed to handle animation event"))?;
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
                WalkL: "walk_l",
                AttackR: "attack_r",
                AttackL: "attack_l",
                AttackWalkR: "attack_walk_r",
                AttackWalkL: "attack_walk_l"
            Transient_map:
                IdleR:
                    WalkR     => WalkR,
                    WalkL     => WalkL,
                    Attack    => AttackR,
                    AnimFin   => IdleR;
                IdleL:
                    WalkR     => WalkR,
                    WalkL     => WalkL,
                    Attack    => AttackL,
                    AnimFin   => IdleL;
                WalkR:
                    WalkL     => WalkL,
                    WalkREnd  => IdleR,
                    Attack    => AttackWalkR,
                    AnimFin   => WalkR;
                WalkL:
                    WalkR     => WalkR,
                    WalkLEnd  => IdleL,
                    Attack    => AttackWalkL,
                    AnimFin   => WalkL;
                AttackR:
                    WalkR     => AttackWalkR,
                    WalkL     => AttackWalkL,
                    AttackEnd => IdleR,
                    AnimFin   => AttackR;
                AttackL:
                    WalkR     => AttackWalkR,
                    WalkL     => AttackWalkL,
                    AttackEnd => IdleL,
                    AnimFin   => AttackL;
                AttackWalkR:
                    WalkL     => AttackWalkL,
                    WalkREnd  => AttackR,
                    AttackEnd => WalkR,
                    AnimFin   => AttackWalkR;
                AttackWalkL:
                    WalkR     => AttackWalkR,
                    WalkLEnd  => AttackL,
                    AttackEnd => WalkL,
                    AnimFin   => AttackWalkL
        );
        let anim_cfg = cfg
            .take("player_anim")
            .change_context(builder::Error::msg("Failed to init player config"))?;
        let player_anim = Animator::new(animator_cfg, anim_cfg, res)
            .change_context(builder::Error::msg("Failed to create new player"))?;

        Ok(Box::new(Self { gui, menu_scene, cfg, player_anim }))
    }
}
