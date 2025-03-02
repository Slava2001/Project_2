//! Slider widget.

use super::Base;
use crate::manager::{
    widget::{
        event::{Event, MouseButton},
        Error, WRef, Widget,
    },
    State,
};
use builder::{self, config::Config, BuildFromCfg};
use core::f64;
use error_stack::{Result, ResultExt};
use renderer::{Drawable, Renderer};
use resources::{Manager, TextureId};
use std::{cell::RefCell, rc::Weak};
use utils::{rect::Rectf, vec2::Vec2f};

/// Slider.
pub struct Slider {
    /// Base widget.
    base: Base,
    /// Slider texture.
    texture: TextureId,
    /// Slider background texture rectangle.
    texture_background_rect: Rectf,
    /// Slider cursor texture rectangle.
    texture_cursor_rect: Rectf,
    /// Slider cursor rectangle.
    cursor_rect: Rectf,
    /// Slider minimum value.
    value_min: f64,
    /// Slider maximum value.
    value_max: f64,
    /// Slider value step.
    value_step: f64,
    /// Cursor maximum `x`.
    max_x: f64,
}

impl Slider {
    /// Create new slider.
    ///
    /// # Errors
    /// Return error if config is not valid.
    pub fn new(mut cfg: Config, res: &mut dyn Manager) -> Result<Self, builder::Error> {
        let texture_name = cfg
            .take::<String>("texture")
            .change_context(builder::Error::msg("Failed to init slide texture"))?;
        let texture = res
            .get_texture(&texture_name)
            .change_context(builder::Error::msg("Failed to find slider texture"))?;
        let texture_background_rect = cfg
            .take("texture_background_rect")
            .change_context(builder::Error::msg("Failed to init slide texture_background_rect"))?;
        let texture_cursor_rect = cfg
            .take("texture_cursor_rect")
            .change_context(builder::Error::msg("Failed to init slide texture_cursor_rect"))?;
        let cursor_rect: Rectf = cfg
            .take("cursor_rect")
            .change_context(builder::Error::msg("Failed to init slide cursor_rect"))?;
        let value_min = cfg
            .take("value_min")
            .change_context(builder::Error::msg("Failed to init slide value_min"))?;
        let value_max = cfg
            .take("value_max")
            .change_context(builder::Error::msg("Failed to init slide value_max"))?;
        let value = cfg
            .take::<f64>("value")
            .change_context(builder::Error::msg("Failed to init slide value"))?;
        let step_number = cfg
            .take::<f64>("step_number")
            .change_context(builder::Error::msg("Failed to init slide value_step"))?;
        let base = Base::new(cfg)
            .change_context(builder::Error::msg("Failed to init base widget for slider"))?;
        let max_x = base.get_rect().w - cursor_rect.w;
        let value_step = if step_number == 0.0 { 0.0 } else { max_x / step_number };

        let mut s = Self {
            base,
            texture,
            texture_background_rect,
            texture_cursor_rect,
            cursor_rect,
            value_min,
            value_max,
            value_step,
            max_x,
        };
        s.cursor_rect.y = (s.base.get_rect().h - cursor_rect.h) / 2.0;
        s.set_value(value);
        Ok(s)
    }

    /// Convert slider value to cursor `x` coordinate.
    fn value_to_x(&self, value: f64) -> f64 {
        (value - self.value_min) / (self.value_max - self.value_min) * (self.max_x)
    }

    /// Update cursor `x` coordinate.
    /// `x` must be in the range 0..(slider width - (cursor width / 2)).
    fn update_cursor_pos(&mut self, x: f64) {
        let mut x = x.clamp(0.0, self.max_x);
        if self.value_step != 0.0 {
            x = (x / self.value_step).round() * self.value_step;
        }
        self.cursor_rect.x = x;
    }

    /// Get slider value.
    #[must_use]
    pub fn get_value(&self) -> f64 {
        (self.value_max - self.value_min).mul_add(self.cursor_rect.x / self.max_x, self.value_min)
    }

    /// Set slider value.
    /// The value will be automatically clipped and sampled.
    pub fn set_value(&mut self, value: f64) {
        self.update_cursor_pos(self.value_to_x(value));
    }
}

impl Widget for Slider {
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
                    self.update_cursor_pos(
                        state.mouse.x
                            - self.base.get_global_position().x
                            - self.cursor_rect.w / 2.0,
                    );
                }
            }
            Event::MouseRelease(mouse_button) => {
                if matches!(mouse_button, MouseButton::Left) && state.is_caught(self_rc.clone()) {
                    state.uncatch(self, self_rc.clone())?;
                    self.get_parent().map(|p| {
                        p.upgrade().map(|p| {
                            p.clone().borrow_mut().add_widget(p.into(), self, self_rc);
                        })
                    });
                    self.set_global_position(self.get_position());
                }
            }
            Event::MouseMove => {
                if state.is_caught(self_rc) {
                    self.update_cursor_pos(
                        state.mouse.x
                            - self.base.get_global_position().x
                            - self.cursor_rect.w / 2.0,
                    );
                }
            }
            _ => {}
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

    fn set_visible_flag(&mut self, is_visible: bool) {
        self.base.set_visible_flag(is_visible);
    }

    fn get_id(&self) -> String {
        self.base.get_id()
    }

    fn is_visible(&self) -> bool {
        self.base.is_visible()
    }
}

impl Drawable for Slider {
    fn draw(&self, renderer: &mut dyn Renderer) {
        let rect = self.base.get_rect();
        renderer.draw_img(rect, self.texture, &self.texture_background_rect);
        renderer.push_state();
        renderer.translate(rect.x, rect.y);
        renderer.draw_img(&self.cursor_rect, self.texture, &self.texture_cursor_rect);
        renderer.pop_state();
        self.base.draw(renderer);
    }
}

impl BuildFromCfg<WRef> for Slider {
    fn build(cfg: Config, res: &mut dyn Manager) -> Result<WRef, builder::Error> {
        Ok(WRef::new(Self::new(cfg, res)?))
    }
}
