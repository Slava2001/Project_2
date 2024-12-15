use super::{
    input_event::InputEvent,
    widget::{event::Event, BaseWidget, WidgetRef},
};
use crate::{
    renderer::{vec2::Vec2f, Drawble, Renderer},
    widget::Panel,
};

pub struct Caught {
    pub widget: WidgetRef,
    pub offset: Vec2f,
}

pub struct State {
    pub caught: Option<Caught>,
    pub mouse: Vec2f,
}

pub struct Manager {
    hovered: WidgetRef,
    root: WidgetRef,
    state: State,
}

impl Manager {
    pub fn new() -> Self {
        let root = WidgetRef::new(BaseWidget::new([0.0; 4].into()));
        let sub_widget = WidgetRef::new(Panel::new([100.0, 100.0, 50.0, 50.0].into()));
        for y in 0..3 {
            for x in 0..3 {
                let subsub_widget = WidgetRef::new(Panel::new(
                    [100.0 + 60.0 * x as f64, 100.0 + 60.0 * y as f64, 50.0, 50.0].into(),
                ));
                sub_widget.borrow_mut().add_widget(sub_widget.clone(), subsub_widget);
            }
        }
        root.borrow_mut().add_widget(root.clone(), sub_widget);
        Self {
            hovered: root.clone(),
            root,
            state: State { mouse: Vec2f::new(0.0, 0.0), caught: None },
        }
    }

    pub fn handle_event(&mut self, event: InputEvent) {
        if let InputEvent::MouseMove(x, y) = event {
            self.state.mouse = (x, y).into();
        }

        if let Some(c) = &self.state.caught {
            let w = c.widget.clone();
            w.borrow_mut().handle_event(w.clone(), Event::InputEvent(event), &mut self.state);
        }
        self.hovered.borrow_mut().handle_event(
            self.hovered.clone(),
            Event::InputEvent(event),
            &mut self.state,
        );
        self.state
            .caught
            .as_ref()
            .map(|t| t.widget.borrow_mut().set_positon(self.state.mouse + t.offset));
        self.update_hovered(self.state.mouse);
    }

    fn update_hovered(&mut self, pos: Vec2f) {
        let hovered = self.root.borrow().get_hovered(pos).unwrap_or(self.root.clone());
        if self.hovered != hovered {
            hovered.borrow_mut().handle_event(hovered.clone(), Event::MouseEnter, &mut self.state);
            self.hovered.borrow_mut().handle_event(
                self.hovered.clone(),
                Event::MouseLeave,
                &mut self.state,
            );
            self.hovered = hovered;
        }
    }
}

impl Drawble for Manager {
    fn draw(&self, renderer: &mut dyn Renderer) {
        self.root.borrow().draw(renderer);
        self.state.caught.as_ref().map(|c| c.widget.borrow().draw(renderer));
    }
}
