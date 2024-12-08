use std::{cell::RefCell, rc::Rc};

use crate::renderer::{color, Drawble, Renderer, State};

use super::{
    input_event::InputEvent,
    widget::{event::Event, BaseWidget, Widget},
};

pub struct Manager {
    root: Rc<RefCell<dyn Widget>>,
    hovered: Option<Rc<RefCell<dyn Widget>>>,
    mouse: (f64, f64),
}

impl Manager {
    pub fn new() -> Self {
        let mut root = BaseWidget::new([0.0; 4].into());
        let mut sub_widget = BaseWidget::new([100.0, 100.0, 50.0, 50.0].into());
        sub_widget
            .add_widget(Rc::new(RefCell::new(BaseWidget::new([100.0, 100.0, 50.0, 50.0].into()))));
        root.add_widget(Rc::new(RefCell::new(sub_widget)));

        Self { root: Rc::new(RefCell::new(root)), mouse: (0.0, 0.0), hovered: None }
    }

    pub fn handle_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::MouseClick(_) => {
                if let Some(h) = &self.hovered {
                    h.borrow_mut().handle_event(Event::InputEvent(event));
                }
            }
            InputEvent::MouseMove(x, y) => {
                self.mouse = (x, y);
                let hovered = self.root.borrow().get_hovered(x, y);
                match (&self.hovered, &hovered) {
                    (None, Some(h)) => h.borrow_mut().handle_event(Event::MouseEnter),
                    (Some(h), None) => h.borrow_mut().handle_event(Event::MouseLeave),
                    (Some(h1), Some(h2)) => {
                        if !Rc::ptr_eq(h1, h2) {
                            h1.borrow_mut().handle_event(Event::MouseLeave);
                            h2.borrow_mut().handle_event(Event::MouseEnter);
                        }
                    }
                    _ => {}
                };
                self.hovered = hovered;
            }
        }
    }
}

impl Drawble for Manager {
    fn draw(&self, renderer: &mut dyn Renderer, state: &dyn State) {
        self.root.borrow().draw(renderer, state);
        renderer.draw_rect(
            state,
            &[self.mouse.0 - 5.0, self.mouse.1 - 5.0, 10.0, 10.0].into(),
            &color::RED,
        );
    }
}
