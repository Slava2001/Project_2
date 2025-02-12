//! Simple renderer implementation.

use super::resmgr::ResMngr;
use graphics::rectangle::Border;
use graphics::{line, text, CharacterCache, Context, DrawState, Image, Rectangle, Transformed};
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

    fn draw_line(&mut self, points: &[Vec2f], color: &Color) {
        for (from, to) in points.iter().zip(points.iter().skip(1)) {
            line(
                color.into(),
                1.0,
                [from.x, from.y, to.x, to.y],
                self.ctx.last().unwrap().transform,
                self.g,
            );
        }
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

    fn draw_text(&mut self, txt: &str, size: f64, rect: &Rect<f64>, font: resources::FontId) {
        let font = self.res.fonts.get_mut(font.0).unwrap();
        #[allow(clippy::cast_possible_truncation)]
        let scale = f64::from(font.font.scale_for_pixel_height(size as f32));
        let transform =
            self.ctx.last().unwrap().transform.trans(
                rect.x,
                f64::from(font.font.v_metrics_unscaled().ascent).mul_add(scale, rect.y),
            );

        let mut image = Image::new_color([0.0, 0.0, 0.0, 1.0]);

        let mut x = 0.0;
        let mut y = 0.0;
        for ch in txt.chars() {
            let character = font.character(size as u32, ch).unwrap();
            let ch_x = x + character.left();
            let ch_y = y - character.top();
            image = image.src_rect([
                character.atlas_offset[0],
                character.atlas_offset[1],
                character.atlas_size[0],
                character.atlas_size[1],
            ]);
            image.draw(
                character.texture,
                &DrawState::default(),
                transform.trans(ch_x, ch_y),
                self.g,
            );
            x += character.advance_width();
            y += character.advance_height();
        }
    }
}
