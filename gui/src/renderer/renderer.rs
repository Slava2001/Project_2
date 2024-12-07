use super::{Color, Rect};

pub trait Renderer {
    fn draw_border(&mut self, rect: &Rect<f64>, width: f64, color: &Color);
}
