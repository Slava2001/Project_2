use super::{color::Color, Rect};

pub trait Renderer {
    fn push_state(&mut self);
    fn pop_state(&mut self);
    fn translate(&mut self, x: f64, y: f64);
    fn draw_rect(&mut self, rect: &Rect<f64>, color: &Color);
}
