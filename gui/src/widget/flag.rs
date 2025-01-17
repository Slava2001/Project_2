//! Flag
//!
//! Flag widget, that change state by click

use error_stack::{Result, ResultExt};
use std::{cell::RefCell, rc::Weak};
use builder::{self, BuildFromCfg};
use crate::manager::{
    widget::{
        event::{Event, MouseButton},
        Error, WRef, Widget,
    },
    State,
};
use renderer::{rect::Rect, vec2::Vec2f, Drawable, Renderer};
use resources::TextureId;

use super::Base;

/// Change state callback. Args: flag and it is state
type FlagCb = dyn FnMut(&mut Flag, bool);

/// Flag widget
pub struct Flag {
    /// Base widget
    base: Base,
    /// Background texture
    texture: TextureId,
    /// Background texture rectangle on on state
    texture_rect_on: Rect<f64>,
    /// Background texture rectangle on off state
    texture_rect_off: Rect<f64>,
    /// Background texture rectangle on hovered and on state
    texture_rect_hovered_on: Rect<f64>,
    /// Background texture rectangle on hovered and off state
    texture_rect_hovered_off: Rect<f64>,
    /// Is widget hovered
    hovered: bool,
    /// Flag state
    state: bool,
    /// Change state cb
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
                if matches!(mouse_button, MouseButton::Left) && state.caught.is_none() {
                    self.set_position(self.get_global_position());
                    self.get_parent()
                        .map(|p| p.upgrade().map(|p| p.borrow_mut().erase_widget(&self_rc)));
                    state.caught = Some(self_rc);
                }
            }
            Event::MouseRelease(mouse_button) => {
                if matches!(mouse_button, MouseButton::Left)
                    && state.caught == Some(self_rc.clone())
                {
                    if self.check_bounds(state.mouse) {
                        self.state = !self.state;
                        if let Some(mut cb) = self.cb.take() {
                            cb(self, self.state);
                            self.cb = Some(cb);
                        }
                    }
                    state.caught = None;
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
            Event::MouseLeave => self.hovered = state.caught == Some(self_rc),
            Event::MouseMove => {}
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

    fn get_rect(&self) -> &Rect<f64> {
        self.base.get_rect()
    }

    fn find(&self, id: &str) -> Option<WRef> {
        self.base.find(id)
    }

    fn get_id(&self) -> String {
        self.base.get_id()
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
    fn build(
        mut cfg: config::Map<String, config::Value>,
        res: &mut dyn resources::Manger,
    ) -> Result<WRef, builder::Error> {
        let bg_texture = cfg
            .remove("background")
            .ok_or_else(|| builder::Error::msg("Failed to init flag, no filed \"background\""))?;
        let bg_name = bg_texture.into_string().change_context(builder::Error::msg(
            "Failed to init flag, filed \"background\" is not a string",
        ))?;
        let texture = res.get_texture(&bg_name).change_context(builder::Error::msg(format!(
            "Failed to init flag, texture: \"{bg_name}\" not found"
        )))?;

        let mut get_rect = |name| -> Result<Rect<f64>, builder::Error> {
            Ok(cfg
                .remove(name)
                .ok_or_else(|| {
                    builder::Error::msg(format!("Failed to init flag, no filed \"{name}\""))
                })?
                .try_deserialize::<[f64; 4]>()
                .change_context(builder::Error::msg(format!(
                    "Failed deserialize filed \"{name}\" as rectangle"
                )))?
                .into())
        };

        Ok(WRef::new(Self {
            hovered: false,
            state: false,
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
