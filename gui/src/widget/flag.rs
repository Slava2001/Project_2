//! Flag.
//!
//! Flag widget, that change state by click.

use crate::manager::{
    widget::{
        event::{Event, MouseButton},
        Error, WRef, Widget,
    },
    State,
};
use builder::{self, config::Config, BuildFromCfg};
use error_stack::{Result, ResultExt};
use renderer::{Drawable, Renderer};
use resources::TextureId;
use std::{cell::RefCell, rc::Weak};
use utils::{rect::Rectf, vec2::Vec2f};

use super::Base;

/// Change state callback. Args: flag and it is state.
type FlagCb = dyn FnMut(&mut Flag, bool);

/// Flag widget.
pub struct Flag {
    /// Base widget.
    base: Base,
    /// Background texture.
    texture: TextureId,
    /// Background texture rectangle on on state.
    texture_rect_on: Rectf,
    /// Background texture rectangle on off state.
    texture_rect_off: Rectf,
    /// Background texture rectangle on hovered and on state.
    texture_rect_hovered_on: Rectf,
    /// Background texture rectangle on hovered and off state.
    texture_rect_hovered_off: Rectf,
    /// Is widget hovered.
    hovered: bool,
    /// Flag state.
    state: bool,
    /// Change state cb.
    cb: Option<Box<FlagCb>>,
}

impl Flag {
    /// Set change state callback. This callback will be called when flag change state.
    pub fn change_state_cb<F: 'static + FnMut(&mut Self, bool)>(&mut self, cb: F) {
        self.cb = Some(Box::new(cb));
    }
}

impl Widget for Flag {
    fn handle_event(
        &mut self,
        self_rc: WRef,
        event: Event,
        state: &mut State,
    ) -> Result<(), Error> {
        match event {
            Event::MousePress(mouse_button) => {
                if matches!(mouse_button, MouseButton::Left) && state.get_caught().is_none() {
                    self.set_position(self.get_global_position());
                    self.get_parent()
                        .map(|p| p.upgrade().map(|p| p.borrow_mut().erase_widget(&self_rc)));
                    state.catch_self(self, self_rc)?;
                }
            }
            Event::MouseRelease(mouse_button) => {
                if matches!(mouse_button, MouseButton::Left) && state.is_caught(self_rc.clone()) {
                    if self.check_bounds(state.mouse) {
                        self.state = !self.state;
                        if let Some(mut cb) = self.cb.take() {
                            cb(self, self.state);
                            self.cb = Some(cb);
                        }
                    }
                    state.uncatch(self, self_rc.clone())?;
                    self.get_parent().map(|p| {
                        p.upgrade().map(|p| {
                            p.clone().borrow_mut().add_widget(p.into(), self, self_rc);
                        })
                    });
                    self.hovered = self.check_bounds(state.mouse);
                    self.set_global_position(self.get_position());
                }
            }
            Event::MouseEnter => self.hovered = true,
            Event::MouseLeave => self.hovered = state.get_caught() == Some(self_rc),
            Event::MouseMove
            | Event::TextInput(_)
            | Event::Caught
            | Event::Released
            | Event::Focused
            | Event::Unfocused
            | Event::KeyPress(_)
            | Event::KeyRelease(_) => {}
        }
        Ok(())
    }

    fn get_hovered(&self, pos: Vec2f) -> Option<WRef> {
        self.base.get_hovered(pos)
    }

    fn check_bounds(&self, pos: Vec2f) -> bool {
        self.base.check_bounds(pos)
    }

    fn add_widget(&mut self, self_ref: WRef, widget: &mut dyn Widget, widget_ref: WRef) {
        self.base.add_widget(self_ref, widget, widget_ref);
    }

    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Widget>>>) {
        self.base.set_parent(parent);
    }

    fn get_parent(&mut self) -> Option<Weak<RefCell<dyn Widget>>> {
        self.base.get_parent()
    }

    fn detach(&mut self, self_rc: &WRef) {
        self.base.detach(self_rc);
    }

    fn erase_widget(&mut self, widget: &WRef) {
        self.base.erase_widget(widget);
    }

    fn set_position(&mut self, pos: Vec2f) {
        self.base.set_position(pos);
    }

    fn get_position(&self) -> Vec2f {
        self.base.get_position()
    }

    fn set_global_position(&mut self, pos: Vec2f) {
        self.base.set_global_position(pos);
    }

    fn get_global_position(&self) -> Vec2f {
        self.base.get_global_position()
    }

    fn get_rect(&self) -> &Rectf {
        self.base.get_rect()
    }

    fn find(&self, id: &str) -> Option<WRef> {
        self.base.find(id)
    }

    fn get_id(&self) -> String {
        self.base.get_id()
    }

    fn set_visible_flag(&mut self, is_visible: bool) {
        self.base.set_visible_flag(is_visible);
    }

    fn is_visible(&self) -> bool {
        self.base.is_visible()
    }
}

impl Drawable for Flag {
    fn draw(&self, renderer: &mut dyn Renderer) {
        let rect = match (self.hovered, self.state) {
            (true, true) => &self.texture_rect_hovered_on,
            (true, false) => &self.texture_rect_hovered_off,
            (false, true) => &self.texture_rect_on,
            (false, false) => &self.texture_rect_off,
        };
        renderer.draw_img(self.base.get_rect(), self.texture, rect);
        self.base.draw(renderer);
    }
}

impl BuildFromCfg<WRef> for Flag {
    fn build(mut cfg: Config, res: &mut dyn resources::Manager) -> Result<WRef, builder::Error> {
        let bg_name = cfg
            .take::<String>("background")
            .change_context(builder::Error::msg("Failed to init flag background texture"))?;
        let state = cfg
            .take_opt::<bool>("state")
            .change_context(builder::Error::msg("Failed to init flag state"))?
            .unwrap_or(false);
        let texture = res.get_texture(&bg_name).change_context(builder::Error::msg(format!(
            "Failed to init flag, texture: \"{bg_name}\" not found"
        )))?;

        let mut get_rect = |name| -> Result<Rectf, builder::Error> {
            cfg.take(name).change_context(builder::Error::msg("Failed to init flag"))
        };

        Ok(WRef::new(Self {
            hovered: false,
            state,
            texture,
            texture_rect_on: get_rect("texture_rect_on")?,
            texture_rect_hovered_off: get_rect("texture_rect_hovered_off")?,
            texture_rect_hovered_on: get_rect("texture_rect_hovered_on")?,
            texture_rect_off: get_rect("texture_rect_off")?,
            base: Base::new(cfg)?,
            cb: None,
        }))
    }
}
