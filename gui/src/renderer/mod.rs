//! GUI renderer interface

pub mod color;
pub mod rect;
pub mod vec2;

/// Objects that implement this trait can be rendered with [`Renderer`]
pub trait Drawable {
    /// Draw object
    fn draw(&self, renderer: &mut dyn Renderer);
}

use color::Color;
use rect::Rect;
use vec2::Vec2f;

/// GUI rendered interface
pub trait Renderer {
    /// Save current renderer state (offset, rotation and scale)
    fn push_state(&mut self);
    /// Restore preview renderer state (offset, rotation and scale)
    fn pop_state(&mut self);
    /// Translate the origin of coordinates
    fn translate(&mut self, x: f64, y: f64);

    /// Draw specified rectangle with specified color
    fn draw_rect(&mut self, rect: &Rect<f64>, color: &Color);
    /// Draw specified line with specified color and width
    fn draw_line(&mut self, from: Vec2f, to: Vec2f, color: &Color);
    /// Draw specified part of texture in some area
    fn draw_img(&mut self, rect: &Rect<f64>, texture: TextureId, texture_rect: &Rect<f64>);
}


/// Resource manger error
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message
    pub fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

pub struct TextureId(pub usize);

pub trait ResourceManger {
    fn load_texture(&mut self, name: &str, path: &str) -> Result<TextureId, Error>;
    fn get_texture(&self, name: &str) -> Result<TextureId, Error>;
}
