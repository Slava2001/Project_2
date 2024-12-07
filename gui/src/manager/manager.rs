use crate::{renderer::{Drawble, Rect, Renderer}, widget::BaseWidget};

use super::InputEvent;

pub struct Manager {
    root: BaseWidget,
    rect: Rect<f64>,
    bound: bool
}

impl Manager {
    pub fn new() -> Self {
        Self { root: BaseWidget::new(), rect: [100.0, 100.0, 100.0, 100.0].into(), bound: false }
    }

    pub fn handle_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::MouseClick(_mouse_button) => todo!(),
            InputEvent::MouseMove(x, y) => self.bound = self.rect.check_bounds(x, y)
        }
    }
}

impl Drawble for Manager {
    fn draw(&self, renderer: &mut dyn Renderer) {
        self.root.draw(renderer);
        let color = match self.bound {
            true => [0.0, 1.0, 0.0].into(),
            false => [1.0, 0.0, 0.0].into(),
        };
        renderer.draw_border(&self.rect, 2.0, &color);
    }
}