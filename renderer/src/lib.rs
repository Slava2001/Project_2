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
use resources::{FontId, TextureId};
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
    /// Draw specified text
    fn draw_text(&mut self, text: &str, size: f64, pos: Vec2f, font: FontId);
}
