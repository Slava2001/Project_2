use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::{
    super::{
        super::renderer::{color, rect::Rect, vec2::Vec2f, Drawble, Renderer},
        State,
    },
    event::Event,
    widget_ref::WidgetRef,
    Widget,
};

pub struct BaseWidget {
    rect: Rect<f64>,
    childs: Vec<WidgetRef>,
    parent: Option<Weak<RefCell<dyn Widget>>>,
}

impl BaseWidget {
    pub fn new(rect: Rect<f64>) -> Self {
        Self { rect, childs: Vec::new(), parent: None }
    }
}

impl Widget for BaseWidget {
    fn handle_event(&mut self, _self_rc: WidgetRef, _event: Event, _state: &mut State) {}

    fn get_hovered(&self, mut pos: Vec2f) -> Option<WidgetRef> {
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

    fn detach(&mut self, self_rc: &WidgetRef) {
        if let Some(ref p) = self.parent {
            if let Some(ref p) = p.upgrade() {
                p.borrow_mut().erase_widget(self_rc);
            }
        }
        self.parent = None;
    }

    fn erase_widget(&mut self, widget: &WidgetRef) {
        self.childs.retain(|c| c != widget);
    }

    fn set_positon(&mut self, pos: Vec2f) {
        self.rect.x = pos.x;
        self.rect.y = pos.y;
    }

    fn get_positon(&self) -> Vec2f {
        Vec2f::new(self.rect.x, self.rect.y)
    }

    fn set_global_positon(&mut self, mut pos: Vec2f) {
        self.parent
            .as_ref()
            .map(|p| p.upgrade().map(|p| pos = pos - p.borrow().get_global_positon()));
        self.rect.x = pos.x;
        self.rect.y = pos.y;
    }

    fn get_global_positon(&self) -> Vec2f {
        let mut pos = Vec2f::new(self.rect.x, self.rect.y);
        self.parent
            .as_ref()
            .map(|p| p.upgrade().map(|p| pos = pos + p.borrow().get_global_positon()));
        pos
    }

    fn get_rect(&self) -> &Rect<f64> {
        &self.rect
    }

    fn get_parent(&mut self) -> Option<Weak<RefCell<dyn Widget>>> {
        self.parent.clone()
    }

    fn add_widget(&mut self, self_ref: WidgetRef, widget: &mut dyn Widget, widget_ref: WidgetRef) {
        widget.set_parent(Some(Rc::downgrade(&self_ref)));
        self.childs.push(widget_ref);
    }
}

impl Drawble for BaseWidget {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.push_state();
        renderer.translate(self.rect.x, self.rect.y);
        for c in self.childs.iter() {
            c.borrow().draw(renderer);
            renderer.draw_line((0.0, 0.0).into(), c.borrow_mut().get_positon(), &color::RED);
        }
        renderer.pop_state();
    }
}
