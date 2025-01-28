//! Simple renderer implementation.

use super::resmgr::ResMngr;
use graphics::rectangle::Border;
use graphics::{line, text, Context, DrawState, Image, Rectangle, Transformed};
use opengl_graphics::GlGraphics;
use renderer::vec2::Vec2f;
use renderer::{color::Color, rect::Rect};

/// Simple implementation of renderer
pub struct Renderer<'a> {
    /// Gl graphics
    pub g: &'a mut GlGraphics,
    /// Contexts stack
    pub ctx: Vec<Context>,
    /// Resources
    pub res: &'a mut ResMngr,
}

impl renderer::Renderer for Renderer<'_> {
    fn draw_rect(&mut self, rect: &Rect<f64>, color: &Color) {
        Rectangle::new([0.0; 4]).border(Border { color: color.into(), radius: 1.0 }).draw(
            [rect.x, rect.y, rect.w, rect.h],
            &DrawState::default(),
            self.ctx.last().unwrap().transform,
            self.g,
        );
    }

    fn push_state(&mut self) {
        self.ctx.push(*self.ctx.last().unwrap());
    }

    fn pop_state(&mut self) {
        self.ctx.pop();
    }

    fn translate(&mut self, x: f64, y: f64) {
        let state = self.ctx.pop().unwrap().trans(x, y);
        self.ctx.push(state);
    }

    fn draw_line(&mut self, from: Vec2f, to: Vec2f, color: &Color) {
        line(
            color.into(),
            1.0,
            [from.x, from.y, to.x, to.y],
            self.ctx.last().unwrap().transform,
            self.g,
        );
    }

    fn draw_img(
        &mut self,
        rect: &Rect<f64>,
        texture: resources::TextureId,
        texture_rect: &Rect<f64>,
    ) {
        Image::new()
            .rect([rect.x, rect.y, rect.w, rect.h])
            .src_rect(Into::<[f64; 4]>::into([
                texture_rect.x,
                texture_rect.y,
                texture_rect.w,
                texture_rect.h,
            ]))
            .draw(
                &self.res.textures[texture.0],
                &DrawState::default(),
                self.ctx.last().unwrap().transform,
                self.g,
            );
    }

    fn draw_text(&mut self, txt: &str, size: f64, pos: Vec2f, font: resources::FontId) {
        let font = self.res.fonts.get_mut(font.0).unwrap();
        #[allow(clippy::cast_possible_truncation)]
        let scale = f64::from(font.font.scale_for_pixel_height(size as f32));
        let transform =
            self.ctx.last().unwrap().transform.trans(
                pos.x,
                f64::from(font.font.v_metrics_unscaled().ascent).mul_add(scale, pos.y),
            );
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        text([0.0, 0.0, 0.0, 1.0], size as u32, txt, font, transform, self.g).unwrap();
    }
}
