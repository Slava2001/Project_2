use std::{
    cell::RefCell,
    mem::replace,
    rc::{Rc, Weak},
};

use super::{event::Event, widget::WidgetRef, Widget};
use crate::{
    manager::input_event::{InputEvent, MouseButton},
    renderer::{
        color::{self, Color},
        vec2::Vec2f,
        Drawble, Rect, Renderer,
    },
};

pub struct BaseWidget {
    rect: Rect<f64>,
    color: Color,
    childs: Vec<WidgetRef>,
    parent: Option<Weak<RefCell<dyn Widget>>>,
}

impl BaseWidget {
    pub fn new(rect: Rect<f64>) -> Self {
        Self { rect, childs: Vec::new(), color: color::BLACK, parent: None }
    }
}

impl Widget for BaseWidget {
    fn handle_event(&mut self, self_rc: WidgetRef, event: Event, caught: &mut Option<WidgetRef>) {
        match event {
            Event::InputEvent(input_event) => match input_event {
                InputEvent::MousePress(mouse_button) => match mouse_button {
                    MouseButton::Right => self.color = color::GREEN,
                    MouseButton::Left => {
                        if caught.is_none() {
                            self.set_positon(self.get_global_positon());
                            self.detach(&self_rc);
                            *caught = Some(self_rc);
                        }
                    },
                    _ => {}
                },
                InputEvent::MouseRelease(mouse_button) => match mouse_button {
                    MouseButton::Left => {
                        if caught.is_some() {
                            let caught = replace(caught, None).unwrap();
                            self.add_widget(self_rc, caught.clone());
                            let pos = caught.borrow().get_positon() - self.get_global_positon();
                            caught.borrow_mut().set_positon(pos);
                        } 
                    },
                    _ => {}
                },
                _ => {}
            },
            Event::MouseEnter => self.color = color::GRAY,
            Event::MouseLeave => self.color = color::BLACK,
        }
    }

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

    fn add_widget(&mut self, self_rc: WidgetRef, widget: WidgetRef) {
        widget.borrow_mut().set_parent(Some(Rc::downgrade(&self_rc)));
        self.childs.push(widget);
    }

    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Widget>>>) {
        self.parent = parent;
    }

    fn detach(&mut self, self_rc: &WidgetRef) {
        if let Some(p) = &self.parent {
            p.upgrade().map(|p| p.borrow_mut().erase_widget(self_rc));
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
}

impl Drawble for BaseWidget {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.draw_rect(&self.rect, &self.color);
        renderer.push_state();
        renderer.translate(self.rect.x, self.rect.y);
        for c in self.childs.iter() {
            c.borrow().draw(renderer);
            renderer.draw_line((0.0, 0.0).into(), c.borrow_mut().get_positon(), &color::RED);
        }
        renderer.pop_state();
    }
}
