mod renderer;
mod color;
mod rect;

pub use renderer::Renderer;
pub use color::Color;
pub use rect::Rect;

pub trait Drawble {
    fn draw(&self, renderer: &mut dyn Renderer);
}