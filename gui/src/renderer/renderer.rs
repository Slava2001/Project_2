use super::{color::Color, vec2::Vec2f, Rect};

pub trait Renderer {
    fn push_state(&mut self);
    fn pop_state(&mut self);
    fn translate(&mut self, x: f64, y: f64);

    fn draw_rect(&mut self, rect: &Rect<f64>, color: &Color);
    fn draw_line(&mut self, from: Vec2f, to: Vec2f, color: &Color);
}
