use super::{
    input_event::InputEvent,
    widget::{event::Event, BaseWidget, WidgetRef},
};
use crate::renderer::{vec2::Vec2f, Drawble, Renderer};

pub struct Manager {
    root: WidgetRef,
    hovered: WidgetRef,
    caught: Option<WidgetRef>,
    mouse: Vec2f,
}

impl Manager {
    pub fn new() -> Self {
        let root = WidgetRef::new(BaseWidget::new([0.0; 4].into()));
        let sub_widget = WidgetRef::new(BaseWidget::new([100.0, 100.0, 50.0, 50.0].into()));
        for y in 0..5 {
            for x in 0..5 {
                let subsub_widget = WidgetRef::new(BaseWidget::new(
                    [100.0 + 20.0 * x as f64, 100.0 + 20.0 * y as f64, 10.0, 10.0].into(),
                ));
                sub_widget.borrow_mut().add_widget(sub_widget.clone(), subsub_widget);
            }
        }
        root.borrow_mut().add_widget(root.clone(), sub_widget);
        Self { hovered: root.clone(), root, mouse: Vec2f::new(0.0, 0.0), caught: None }
    }

    pub fn handle_event(&mut self, event: InputEvent) {
        if let InputEvent::MouseMove(x, y) = event {
            self.mouse = (x, y).into();
            self.caught.as_ref().map(|t| t.borrow_mut().set_positon(self.mouse));
        }
        self.hovered.borrow_mut().handle_event(
            self.hovered.clone(),
            Event::InputEvent(event),
            &mut self.caught,
        );
        // if let Some(c) = self.caught.clone() {
        //     c.borrow_mut().handle_event(c.clone(), Event::InputEvent(event), &mut self.caught);
        // }
        self.update_hovered(self.mouse);
    }

    fn update_hovered(&mut self, pos: Vec2f) {
        let hovered = self.root.borrow().get_hovered(pos).unwrap_or(self.root.clone());
        if self.hovered != hovered {
            hovered.borrow_mut().handle_event(hovered.clone(), Event::MouseLeave, &mut self.caught);
            self.hovered.borrow_mut().handle_event(
                self.hovered.clone(),
                Event::MouseEnter,
                &mut self.caught,
            );
            self.hovered = hovered;
        }
    }
}

impl Drawble for Manager {
    fn draw(&self, renderer: &mut dyn Renderer) {
        self.root.borrow().draw(renderer);
        self.caught.as_ref().map(|c| c.borrow().draw(renderer));
    }
}
