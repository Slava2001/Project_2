//! Simple graph.

use error_stack::{Result, ResultExt};
use std::{cell::RefCell, rc::Weak};

use renderer::{
    color::{self, Color},
    rect::Rect,
    vec2::Vec2f,
    Drawable, Renderer,
};

use crate::manager::{
    widget::{event::Event, Error, WRef, Widget},
    State,
};
use builder::{self, BuildFromCfg, Config};

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
    fn build(mut cfg: Config, _res: &mut dyn resources::Manger) -> Result<WRef, builder::Error> {
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
            color: cfg
                .take::<String>("color")
                .change_context(builder::Error::msg("Failed to init color"))?
                .parse::<Color>()
                .change_context(builder::Error::msg("Failed to parse color"))?,
            base: Base::new(cfg)?,
        }))
    }
}
