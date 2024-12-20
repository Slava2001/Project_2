//! Base implementation of widget. It used as root of GUI tree and for implementing other widget

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::{
    super::{
        super::renderer::{color, rect::Rect, vec2::Vec2f, Drawable, Renderer},
        State,
    },
    event::Event,
    wref::WRef,
    Widget,
};

/// Base implementation of widget
pub struct Base {
    /// Widget bounds
    rect: Rect<f64>,
    /// Widget childs
    childs: Vec<WRef>,
    /// Reference on parent widget
    parent: Option<Weak<RefCell<dyn Widget>>>,
}

impl Base {
    /// Create new base widget
    #[must_use]
    pub fn new(rect: Rect<f64>) -> Self {
        Self { rect, childs: Vec::new(), parent: None }
    }
}

impl Widget for Base {
    fn handle_event(&mut self, _self_rc: WRef, _event: Event, _state: &mut State) {}

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

    fn set_positon(&mut self, pos: Vec2f) {
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
}

impl Drawable for Base {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.push_state();
        renderer.translate(self.rect.x, self.rect.y);
        for c in &self.childs {
            c.borrow().draw(renderer);
            renderer.draw_line((0.0, 0.0).into(), c.borrow_mut().get_position(), &color::RED);
        }
        renderer.pop_state();
    }
}
