pub mod color;
mod rect;
mod renderer;

pub use rect::Rect;
pub use renderer::Renderer;
pub use renderer::State;

pub trait Drawble {
    fn draw(&self, renderer: &mut dyn Renderer, state: &dyn State);
}
