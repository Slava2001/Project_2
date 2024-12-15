use std::{cell::RefCell, rc::Weak};

use crate::{
    manager::{
    input_event::{InputEvent, MouseButton}, widget::{event::{self, Event}, BaseWidget, Widget, WidgetRef}, Caught, State},
    renderer::{
        color::{self, Color},
        vec2::Vec2f,
        Drawble, Rect, Renderer,
    },
};

pub struct Panel {
    base: BaseWidget,
    color: Color,
}

impl Panel {
    pub fn new(rect: Rect<f64>) -> Self {
        Self { color: color::BLACK, base: BaseWidget::new(rect) }
    }
}

impl Widget for Panel {
    fn handle_event(&mut self, self_rc: WidgetRef, event: Event, state: &mut State) {
        match event {
            Event::MouseEnter => self.color = color::GRAY,
            Event::MouseLeave => self.color = color::BLACK,
            Event::InputEvent(i) => match i {
                // InputEvent::MousePress(MouseButton::Left) => {
                //     self.color = color::GREEN;
                //     if state.caught.is_none() {
                //         self.get_parent().map(|p| p.upgrade().map(|p| p.borrow_mut().erase_widget(&self_rc)));
                //         let offset = self.get_global_positon() - state.mouse;
                //         state.caught = Some(Caught { widget: self_rc, offset: offset });
                //     }
                // },
                // InputEvent::MouseRelease(MouseButton::Left) => {
                //     self.color = color::RED;
                //     if let Some(caught) = &state.caught {
                //         if caught.widget == self_rc {
                //             self.get_parent().map(|p| p.upgrade().map(|p| p.clone().borrow_mut().add_widget(p.into(), self_rc)));
                //             state.caught = None;
                //         }
                //     }
                // },
                _ => {}
            }
            _ => {}
        }
    }

    fn get_hovered(&self, pos: Vec2f) -> Option<WidgetRef> {
        self.base.get_hovered(pos)
    }

    fn check_bounds(&self, pos: Vec2f) -> bool {
        self.base.check_bounds(pos)
    }

    fn add_widget(&mut self, self_rc: WidgetRef, widget: WidgetRef) {
        self.base.add_widget(self_rc, widget);
    }

    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Widget>>>) {
        self.base.set_parent(parent);
    }

    fn get_parent(&mut self) -> Option<Weak<RefCell<dyn Widget>>> {
        self.base.get_parent()
    }

    fn detach(&mut self, self_rc: &WidgetRef) {
        self.base.detach(self_rc);
    }

    fn erase_widget(&mut self, widget: &WidgetRef) {
        self.base.erase_widget(widget);
    }

    fn set_positon(&mut self, pos: Vec2f) {
        self.base.set_positon(pos);
    }

    fn get_positon(&self) -> Vec2f {
        self.base.get_positon()
    }

    fn set_global_positon(&mut self, pos: Vec2f) {
        self.base.set_global_positon(pos);
    }

    fn get_global_positon(&self) -> Vec2f {
        self.base.get_global_positon()
    }

    fn get_rect(&self) -> &Rect<f64> {
        self.base.get_rect()
    }
}

impl Drawble for Panel {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.draw_rect(&self.base.get_rect(), &self.color);
        self.base.draw(renderer);
    }
}
