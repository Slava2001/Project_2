use super::{color::Color, Rect};

pub trait State<'a> {
    fn boxed_clone(&self) -> Box<dyn State>;
    fn translate(&mut self, x: f64, y: f64);
}

pub trait Renderer {
    fn draw_rect(&mut self, state: &dyn State, rect: &Rect<f64>, color: &Color);
}
