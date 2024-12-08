use std::{cell::RefCell, rc::Rc};

use super::{event::Event, Widget};
use crate::{
    manager::input_event::InputEvent,
    renderer::{
        color::{self, Color},
        Drawble, Rect, Renderer,
    },
};

pub struct BaseWidget {
    rect: Rect<f64>,
    color: Color,
    childs: Vec<Rc<RefCell<dyn Widget>>>,
}

impl BaseWidget {
    pub fn new(rect: Rect<f64>) -> Self {
        Self { rect, childs: Vec::new(), color: color::BLACK }
    }
}

impl Widget for BaseWidget {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::InputEvent(input_event) => match input_event {
                InputEvent::MouseClick(_mouse_button) => self.color = color::GREEN,
                InputEvent::MouseMove(_, _) => {}
            },
            Event::MouseEnter => self.color = color::GRAY,
            Event::MouseLeave => self.color = color::BLACK,
        }
    }

    fn get_hovered(&self, x: f64, y: f64) -> Option<Rc<RefCell<dyn Widget>>> {
        for c in self.childs.iter().rev() {
            if let Some(c) = c.borrow().get_hovered(x, y) {
                return Some(c);
            }
            if c.borrow().check_bounds(x, y) {
                return Some(Rc::clone(c));
            }
        }
        None
    }

    fn check_bounds(&self, x: f64, y: f64) -> bool {
        self.rect.check_bounds(x, y)
    }

    fn add_widget(&mut self, widget: Rc<RefCell<dyn Widget>>) {
        self.childs.push(widget);
    }
}

impl Drawble for BaseWidget {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.draw_rect(&self.rect, &self.color);
        for c in self.childs.iter() {
            c.borrow().draw(renderer);
        }
    }
}
