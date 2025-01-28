//! Label.
//!
//! Label widget, that used for display text.

use crate::manager::{
    widget::{event::Event, Error, WRef, Widget},
    State,
};
use builder::{self, BuildFromCfg, Config};
use error_stack::{Result, ResultExt};
use renderer::{rect::Rect, vec2::Vec2f, Drawable, Renderer};
use resources::FontId;
use std::{cell::RefCell, rc::Weak};

use super::Base;

/// Label widget.
pub struct Label {
    /// Base widget.
    base: Base,
    /// Label text.
    text: String,
    /// Font size.
    size: f64,
    /// Font identification.
    font: FontId,
}

impl Label {
    /// Set label text.
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
    fn build(mut cfg: Config, res: &mut dyn resources::Manger) -> Result<WRef, builder::Error> {
        Ok(WRef::new(Self {
            text: cfg
                .take::<String>("text")
                .change_context(builder::Error::msg("Failed to init label text"))?,
            size: cfg
                .take::<f64>("font_size")
                .change_context(builder::Error::msg("Failed to init label font size"))?,
            font: res
                .get_font(
                    &cfg.take::<String>("font")
                        .change_context(builder::Error::msg("Failed to init label font"))?,
                )
                .change_context(builder::Error::msg("Failed to find required font"))?,
            base: Base::new(cfg)?,
        }))
    }
}
