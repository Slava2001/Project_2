//! Panel.
//!
//! Simple widget. It used for groups other widgets.

use error_stack::{Result, ResultExt};
use std::{cell::RefCell, rc::Weak};

use super::Base;
use crate::manager::{
    widget::{
        event::{Event, MouseButton},
        Error, WRef, Widget,
    },
    State,
};
use builder::{self, config::Config, BuildFromCfg};
use renderer::{Drawable, Renderer};
use resources::TextureId;
use utils::{rect::Rectf, vec2::Vec2f};

/// Panel widget.
pub struct Panel {
    /// Base widget.
    base: Base,
    /// Background texture.
    texture: TextureId,
    /// Background texture rectangle.
    texture_rect: Rectf,
    /// Offset, used when widget cached.
    offset: Vec2f,
}

impl Widget for Panel {
    fn handle_event(
        &mut self,
        self_rc: WRef,
        event: Event,
        state: &mut State,
    ) -> Result<(), Error> {
        match event {
            Event::MousePress(MouseButton::Left) => {
                if state.get_caught().is_none() {
                    self.get_parent()
                        .map(|p| p.upgrade().map(|p| p.borrow_mut().erase_widget(&self_rc)));
                    self.offset = self.get_global_position() - state.mouse;
                    self.set_position(state.mouse + self.offset);
                    state.catch_self(self, self_rc)?;
                }
            }
            Event::MouseRelease(MouseButton::Left) => {
                if let Some(caught) = state.get_caught() {
                    if caught == self_rc {
                        self.get_parent().map(|p| {
                            p.upgrade().map(|p| {
                                p.clone().borrow_mut().add_widget(p.into(), self, self_rc.clone());
                            })
                        });
                        state.uncatch(self, self_rc)?;
                        self.set_global_position(self.get_position());
                    }
                }
            }
            Event::MouseMove => {
                if state.is_caught(self_rc) {
                    self.set_position(state.mouse + self.offset);
                }
            }
            _ => {}
        }
        Ok(())
    }

    delegate::delegate! {
        to self.base {
            fn get_hovered(&self, pos: Vec2f) -> Option<WRef>;
            fn check_bounds(&self, pos: Vec2f) -> bool;
            fn add_widget(&mut self, self_ref: WRef, widget: &mut dyn Widget, widget_ref: WRef);
            fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Widget>>>);
            fn get_parent(&mut self) -> Option<Weak<RefCell<dyn Widget>>>;
            fn detach(&mut self, self_rc: &WRef);
            fn erase_widget(&mut self, widget: &WRef);
            fn set_position(&mut self, pos: Vec2f);
            fn get_position(&self) -> Vec2f;
            fn set_global_position(&mut self, pos: Vec2f);
            fn get_global_position(&self) -> Vec2f;
            fn set_size(&mut self, size: Vec2f);
            fn handle_parent_resize(&mut self, size: Vec2f);
            fn get_rect(&self) -> &Rectf;
            fn find(&self, id: &str) -> Option<WRef>;
            fn get_id(&self) -> String;
            fn set_visible_flag(&mut self, is_visible: bool);
            fn is_visible(&self) -> bool;
        }
    }
}

impl Drawable for Panel {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.draw_img(self.base.get_rect(), self.texture, &self.texture_rect);
        self.base.draw(renderer);
    }
}

impl BuildFromCfg<WRef> for Panel {
    fn build(mut cfg: Config, res: &mut dyn resources::Manager) -> Result<WRef, builder::Error> {
        let bg_name = cfg
            .take::<String>("background")
            .change_context(builder::Error::msg("Failed to init button background texture"))?;
        let texture = res.get_texture(&bg_name).change_context(builder::Error::msg(format!(
            "Failed to init button, texture: \"{bg_name}\" not found"
        )))?;

        let texture_rect = cfg
            .take("background_rect")
            .change_context(builder::Error::msg("Failed to init button"))?;

        Ok(WRef::new(Self {
            base: Base::new(cfg)?,
            texture,
            texture_rect,
            offset: Vec2f::new(0.0, 0.0),
        }))
    }
}
