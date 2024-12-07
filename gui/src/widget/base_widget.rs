use super::Widget;
use crate::renderer::{Drawble, Renderer};

pub struct BaseWidget {}

impl BaseWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for BaseWidget {}

impl Drawble for BaseWidget {
    fn draw(&self, _renderer: &mut dyn Renderer) {
        // renderer.draw_border([100.0, 100.0, 100.0, 100.0].into(), 2.0, [1.0, 0.0, 0.0].into());
    }
}
