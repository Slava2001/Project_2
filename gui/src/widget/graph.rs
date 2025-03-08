//! Simple graph.

use error_stack::{Result, ResultExt};
use renderer::{Drawable, Renderer};
use std::{cell::RefCell, rc::Weak};
use utils::{
    color::{self, Color},
    rect::Rectf,
    vec2::Vec2f,
};

use crate::manager::{
    widget::{event::Event, Error, WRef, Widget},
    State,
};
use builder::{self, config::Config, BuildFromCfg};

use super::Base;

/// Simple graph.
pub struct Graph {
    /// Base widget.
    base: Base,
    /// Queue length.
    value_count: usize,
    /// Points.
    points: Vec<Vec2f>,
    /// Maximum value.
    value_max: f64,
    /// Minimum value.
    value_min: f64,
    /// Graph color.
    color: Color,
}

impl Graph {
    /// Push value.
    pub fn push(&mut self, value: f64) {
        let mut points = Vec::with_capacity(self.value_count);
        std::mem::swap(&mut self.points, &mut points);
        let bounds = self.base.get_rect();
        let v = bounds.h
            - bounds.h * (value.clamp(self.value_min, self.value_max) - self.value_min)
                / (self.value_max - self.value_min);
        self.points.push((bounds.w, v).into());
        #[allow(clippy::cast_precision_loss)]
        let x_step = bounds.w / (self.value_count - 1) as f64;
        for (v, i) in points.into_iter().zip(1..self.value_count) {
            #[allow(clippy::cast_precision_loss)]
            let x = x_step * (self.value_count - 1 - i) as f64;
            self.points.push(Vec2f::new(x, v.y));
        }
    }
}

impl Widget for Graph {
    fn handle_event(
        &mut self,
        _self_rc: WRef,
        _event: Event,
        _state: &mut State,
    ) -> Result<(), Error> {
        Ok(())
    }

    delegate::delegate! {
        to self.base {
            fn get_hovered(&self, pos: Vec2f) -> Option<WRef>;
            fn check_bounds(&self, pos: Vec2f) -> bool;
            fn add_widget(&mut self, self_ref: WRef, widget: &mut dyn Widget, widget_ref: WRef);
            fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Widget>>>);
            fn get_parent(&mut self) -> Option<Weak<RefCell<dyn Widget>>>;
            fn detach(&mut self, self_rc: &WRef);
            fn erase_widget(&mut self, widget: &WRef);
            fn set_position(&mut self, pos: Vec2f);
            fn get_position(&self) -> Vec2f;
            fn set_global_position(&mut self, pos: Vec2f);
            fn get_global_position(&self) -> Vec2f;
            fn set_size(&mut self, size: Vec2f);
            fn handle_parent_resize(&mut self, size: Vec2f);
            fn get_rect(&self) -> &Rectf;
            fn find(&self, id: &str) -> Option<WRef>;
            fn get_id(&self) -> String;
            fn set_visible_flag(&mut self, is_visible: bool);
            fn is_visible(&self) -> bool;
        }
    }
}

impl Drawable for Graph {
    fn draw(&self, renderer: &mut dyn Renderer) {
        let bounds = self.base.get_rect();
        renderer.draw_rect(bounds, &color::BLACK);
        renderer.push_state();
        renderer.translate(bounds.x, bounds.y);
        renderer.draw_line(&self.points, &self.color);
        renderer.pop_state();
        self.base.draw(renderer);
    }
}

impl BuildFromCfg<WRef> for Graph {
    fn build(mut cfg: Config, _res: &mut dyn resources::Manager) -> Result<WRef, builder::Error> {
        Ok(WRef::new(Self {
            value_count: cfg
                .take("value_count")
                .change_context(builder::Error::msg("Failed to init max values count"))?,
            value_min: cfg
                .take("value_min")
                .change_context(builder::Error::msg("Failed to init value minimum"))?,
            value_max: cfg
                .take("value_max")
                .change_context(builder::Error::msg("Failed to init value maximum"))?,
            points: Vec::new(),
            color: cfg.take("color").change_context(builder::Error::msg("Failed to init color"))?,
            base: Base::new(cfg)?,
        }))
    }
}
