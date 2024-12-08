use super::{color::Color, Rect};

pub trait Renderer {
    fn draw_rect(&mut self, rect: &Rect<f64>, color: &Color);
}
