//! Label.
//!
//! Label widget, that used for display text.

use crate::manager::{
    widget::{event::Event, Error, WRef, Widget},
    State,
};
use builder::{self, BuildFromCfg, Config};
use error_stack::{Result, ResultExt};
use renderer::{color::Color, rect::Rect, vec2::Vec2f, Drawable, Renderer, TextTruncateMode};
use resources::FontId;
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Weak,
};

use super::Base;

/// Label widget.
pub struct Label {
    /// Base widget.
    base: Base,
    /// Label text.
    text: RefCell<String>,
    /// Font size.
    size: f64,
    /// Font identification.
    font: FontId,
    /// Text color.
    color: Color,
    /// Text draw truncate mode.
    draw_truncate: TextTruncateMode,
    /// Text truncate mode.
    need_to_truncate_text: bool,
    /// Label border color.
    rect_color: Color,
}

impl Label {
    /// Create new label.
    ///
    /// # Errors
    /// Return error if the config is incorrect or the required resource is not found.
    pub fn new(mut cfg: Config, res: &mut dyn resources::Manger) -> Result<Self, builder::Error> {
        Ok(Self {
            text: RefCell::new(
                cfg.take::<String>("text")
                    .change_context(builder::Error::msg("Failed to init label text"))?,
            ),
            size: cfg
                .take::<f64>("font_size")
                .change_context(builder::Error::msg("Failed to init label font size"))?,
            font: res
                .get_font(
                    &cfg.take::<String>("font")
                        .change_context(builder::Error::msg("Failed to init label font"))?,
                )
                .change_context(builder::Error::msg("Failed to find required font"))?,
            color: cfg
                .take::<String>("color")
                .change_context(builder::Error::msg("Failed to init color"))?
                .parse::<Color>()
                .change_context(builder::Error::msg("Failed to parse color"))?,
            rect_color: cfg
                .take::<String>("rect_color")
                .change_context(builder::Error::msg("Failed to init Textbox border color"))?
                .parse::<Color>()
                .change_context(builder::Error::msg("Failed to parse border color"))?,
            base: Base::new(cfg)?,
            draw_truncate: TextTruncateMode::Back,
            need_to_truncate_text: false,
        })
    }

    /// Get access to read label text.
    pub fn text(&self) -> Ref<'_, String> {
        self.text.borrow()
    }

    /// Get access to change label text.
    pub fn text_mut(&self) -> RefMut<'_, String> {
        self.text.borrow_mut()
    }

    /// Set text truncate mode.
    pub fn set_draw_truncate_mode(&mut self, mode: TextTruncateMode) {
        self.draw_truncate = mode;
    }

    /// Set to true to clip the text to fit it into the rectangle.
    pub fn set_text_truncating(&mut self, mode: bool) {
        self.need_to_truncate_text = mode;
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

    fn set_visible_flag(&mut self, is_visible: bool) {
        self.base.set_visible_flag(is_visible);
    }

    fn is_visible(&self) -> bool {
        self.base.is_visible()
    }
}

impl Drawable for Label {
    fn draw(&self, renderer: &mut dyn Renderer) {
        let rc = renderer.draw_text(
            &self.text.borrow(),
            self.size,
            self.base.get_rect(),
            self.font,
            &self.color,
            self.draw_truncate,
        );
        if self.need_to_truncate_text && rc != 0 {
            match self.draw_truncate {
                TextTruncateMode::Front => {
                    self.text.borrow_mut().drain(..rc);
                }
                TextTruncateMode::Back => {
                    self.text.borrow_mut().drain(rc..);
                }
            }
        }
        renderer.draw_rect(self.base.get_rect(), &self.rect_color);
        self.base.draw(renderer);
    }
}

impl BuildFromCfg<WRef> for Label {
    fn build(cfg: Config, res: &mut dyn resources::Manger) -> Result<WRef, builder::Error> {
        Ok(WRef::new(Self::new(cfg, res)?))
    }
}
