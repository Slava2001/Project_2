//! Base implementation of widget. It used as root of GUI tree and for implementing other widget

use error_stack::{Result, ResultExt};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::resources::Manger;

use super::{
    super::{
        super::renderer::{color, rect::Rect, vec2::Vec2f, Drawable, Renderer},
        State,
    },
    builder::{self, BuildFromCfg},
    event::Event,
    wref::WRef,
    Error, Widget,
};

/// Base implementation of widget
pub struct Base {
    /// Widget bounds
    rect: Rect<f64>,
    /// Widget childs
    childs: Vec<WRef>,
    /// Reference on parent widget
    parent: Option<Weak<RefCell<dyn Widget>>>,
    /// Enable debug mode
    debug: bool,
    /// Widget identifier
    id: String,
}

impl Base {
    /// Create new Base widget from config
    ///
    /// # Errors
    /// Return error if config is not valid
    pub fn new(mut cfg: config::Map<String, config::Value>) -> Result<Self, builder::Error> {
        let rect = if let Some(rect) = cfg.remove("rect") {
            rect.try_deserialize::<[f64; 4]>()
                .change_context(builder::Error::msg("Failed to parse \"rect\" field as bounds"))?
        } else {
            [0.0; 4]
        }
        .into();
        let debug = if let Some(debug) = cfg.remove("debug") {
            debug.into_bool().change_context(builder::Error::msg(
                "Failed to parse \"debug\" field as debug flag",
            ))?
        } else {
            false
        };
        let id = if let Some(debug) = cfg.remove("id") {
            debug.into_string().change_context(builder::Error::msg(
                "Failed to parse \"id\" field widget identifier",
            ))?
        } else {
            String::new()
        };
        Ok(Self { rect, childs: Vec::new(), parent: None, debug, id })
    }
}

impl Widget for Base {
    fn handle_event(
        &mut self,
        _self_rc: WRef,
        _event: Event,
        _state: &mut State,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn get_hovered(&self, mut pos: Vec2f) -> Option<WRef> {
        pos = pos - (self.rect.x, self.rect.y).into();
        for c in self.childs.iter().rev() {
            if let Some(c) = c.borrow().get_hovered(pos) {
                return Some(c);
            }
            if c.borrow().check_bounds(pos) {
                return Some(c.clone());
            }
        }
        None
    }

    fn check_bounds(&self, pos: Vec2f) -> bool {
        self.rect.check_bounds(pos.x, pos.y)
    }

    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Widget>>>) {
        self.parent = parent;
    }

    fn detach(&mut self, self_rc: &WRef) {
        if let Some(ref p) = self.parent {
            if let Some(ref p) = p.upgrade() {
                p.borrow_mut().erase_widget(self_rc);
            }
        }
        self.parent = None;
    }

    fn erase_widget(&mut self, widget: &WRef) {
        self.childs.retain(|c| c != widget);
    }

    fn set_position(&mut self, pos: Vec2f) {
        self.rect.x = pos.x;
        self.rect.y = pos.y;
    }

    fn get_position(&self) -> Vec2f {
        Vec2f::new(self.rect.x, self.rect.y)
    }

    fn set_global_position(&mut self, mut pos: Vec2f) {
        self.parent
            .as_ref()
            .map(|p| p.upgrade().map(|p| pos = pos - p.borrow().get_global_position()));
        self.rect.x = pos.x;
        self.rect.y = pos.y;
    }

    fn get_global_position(&self) -> Vec2f {
        let mut pos = Vec2f::new(self.rect.x, self.rect.y);
        self.parent
            .as_ref()
            .map(|p| p.upgrade().map(|p| pos = pos + p.borrow().get_global_position()));
        pos
    }

    fn get_rect(&self) -> &Rect<f64> {
        &self.rect
    }

    fn get_parent(&mut self) -> Option<Weak<RefCell<dyn Widget>>> {
        self.parent.clone()
    }

    fn add_widget(&mut self, self_ref: WRef, widget: &mut dyn Widget, widget_ref: WRef) {
        widget.set_parent(Some(Rc::downgrade(&self_ref)));
        self.childs.push(widget_ref);
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn find(&self, id: &str) -> Option<WRef> {
        for c in self.childs.iter().rev() {
            if let Some(c) = c.borrow().find(id) {
                return Some(c);
            }
            if c.borrow().get_id() == id {
                return Some(c.clone());
            }
        }
        None
    }
}

impl Drawable for Base {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.push_state();
        if self.debug {
            renderer.draw_rect(&self.rect, &color::RED);
        }
        renderer.translate(self.rect.x, self.rect.y);
        for c in &self.childs {
            c.borrow().draw(renderer);
            if self.debug {
                renderer.draw_line((0.0, 0.0).into(), c.borrow_mut().get_position(), &color::RED);
            }
        }
        renderer.pop_state();
    }
}

impl BuildFromCfg for Base {
    fn build(
        cfg: config::Map<String, config::Value>,
        _r: &mut dyn Manger,
    ) -> Result<WRef, builder::Error> {
        Ok(WRef::new(Self::new(cfg)?))
    }
}
