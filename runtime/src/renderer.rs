//! Simple renderer implementation.

use super::resmgr::ResMngr;
use graphics::rectangle::Border;
use graphics::{
    line, Character, CharacterCache, Context, DrawState, Image, Rectangle, Transformed,
};
use opengl_graphics::{GlGraphics, Texture};
use renderer::vec2::Vec2f;
use renderer::TextTruncateMode;
use renderer::{color::Color, rect::Rect};
use resources::FontId;

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

    fn draw_text(
        &mut self,
        txt: &[char],
        size: f64,
        rect: &Rect<f64>,
        font: FontId,
        color: &Color,
        mode: TextTruncateMode,
    ) -> usize {
        let font = self.res.fonts.get_mut(font.0).unwrap();
        #[allow(clippy::cast_possible_truncation)]
        let scale = f64::from(font.font.scale_for_pixel_height(size as f32));
        let vmetric = font.font.v_metrics_unscaled();
        let ascent = f64::from(vmetric.ascent) * scale * 1.2;
        let descent = f64::from(vmetric.line_gap - vmetric.descent) * scale;
        let line_step = ascent + descent;
        let y_lim = rect.h - line_step;
        let transform = self.ctx.last().unwrap().transform.trans(rect.x, rect.y + ascent);

        #[allow(clippy::type_complexity)]
        let mut iter_over_char =
            |iter: &mut dyn Iterator<Item = char>,
             f: &mut dyn FnMut(char, &Character<'_, Texture>, f64, f64)| {
                let mut x = 0.0;
                let mut y = 0.0;
                for ch in iter {
                    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                    let character = font.character(size as u32, ch).unwrap();
                    let mut ch_x = x + character.left();
                    let mut ch_y = y - character.top();
                    if rect.w > 0.0 && (ch_x + character.advance_width() > rect.w || ch == '\n') {
                        x = 0.0;
                        y += line_step;
                        ch_x = x + character.left();
                        ch_y = y - character.top();
                        if rect.h > 0.0 && y > y_lim {
                            break;
                        }
                    }
                    f(ch, &character, ch_x, ch_y);

                    if ch != '\n' {
                        x += character.advance_width();
                    }
                    y += character.advance_height();
                }
            };

        let mut start_index = 0;
        if matches!(mode, TextTruncateMode::Front) {
            iter_over_char(&mut txt.iter().cloned().rev(), &mut |_, _, _, _| start_index += 1);
            start_index = txt.len() - start_index;
        }

        let mut image = Image::new_color(Into::<[f32; 4]>::into(color));
        let mut displayed_chars = 0;
        iter_over_char(&mut txt[start_index..].iter().cloned(), &mut |c, ch, x, y| {
            displayed_chars += 1;
            if c != '\n' {
                let rect =
                    [ch.atlas_offset[0], ch.atlas_offset[1], ch.atlas_size[0], ch.atlas_size[1]];
                image = image.src_rect(rect);
                image.draw(ch.texture, &DrawState::default(), transform.trans(x, y), self.g);
            }
        });
        txt.len() - displayed_chars
    }
}
