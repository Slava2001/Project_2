//! Label
//!
//! Label widget, that used for display text

use error_stack::{Result, ResultExt};
use std::{cell::RefCell, rc::Weak};
use builder::{self, BuildFromCfg};
use crate::manager::{
    widget::{
         Error, event::Event, WRef, Widget,
    },
    State,
};
use renderer::{rect::Rect, vec2::Vec2f, Drawable, Renderer};
use resources::FontId;

use super::Base;

/// Label widget
pub struct Label {
    /// Base widget
    base: Base,
    /// Label text
    text: String,
    /// Font size
    size: f64,
    /// Font identification
    font: FontId,
}

impl Label {
    /// Set label text
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
    }
}

impl Widget for Label {
    fn handle_event(
        &mut self,
        _self_rc: WRef,
        _event: Event,
        _state: &mut State,
    ) -> Result<(), Error> {
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

impl Drawable for Label {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.draw_text(&self.text, self.size, self.base.get_position(), self.font);
        self.base.draw(renderer);
    }
}

impl BuildFromCfg<WRef> for Label {
    fn build(
        mut cfg: config::Map<String, config::Value>,
        res: &mut dyn resources::Manger,
    ) -> Result<WRef, builder::Error> {
        Ok(WRef::new(Self {
            text: cfg
                .remove("text")
                .ok_or_else(|| builder::Error::msg("Failed to init label, no filed \"text\""))?
                .into_string()
                .change_context(builder::Error::msg(
                    "Failed to init label, filed \"text\" is not a string",
                ))?,
            size: cfg
                .remove("font_size")
                .ok_or_else(|| builder::Error::msg("Failed to init label, no filed \"font_size\""))?
                .into_float()
                .change_context(builder::Error::msg(
                    "Failed to init label, filed \"font_size\" is not a float number",
                ))?,
            font: res
                .get_font(
                    &cfg.remove("font")
                        .ok_or_else(|| {
                            builder::Error::msg("Failed to init label, no filed \"font\"")
                        })?
                        .into_string()
                        .change_context(builder::Error::msg(
                            "Failed to init label, filed \"font\" is not a string",
                        ))?,
                )
                .change_context(builder::Error::msg("Failed to find required font"))?,
            base: Base::new(cfg)?,
        }))
    }
}
