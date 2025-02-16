//! Button.
//!
//! Button widget.

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
use builder::{self, BuildFromCfg, Config};
use renderer::{rect::Rect, vec2::Vec2f, Drawable, Renderer};
use resources::TextureId;

/// Button click callback. Called then user click on button.
type ButtonCb = dyn FnMut(&mut Button);

/// Button widget.
pub struct Button {
    /// Base widget.
    base: Base,
    /// Background texture.
    texture: TextureId,
    /// Background texture rectangle when button is preset.
    texture_rect_pressed: Rect<f64>,
    /// Background texture rectangle when button is released.
    texture_rect: Rect<f64>,
    /// Background texture rectangle when button is hovered and released.
    texture_rect_hovered: Rect<f64>,
    /// Is widget hovered.
    hovered: bool,
    /// Button is pressed.
    state: bool,
    /// Pressed cb.
    cb: Option<Box<ButtonCb>>,
}

impl Button {
    /// Set click callback. This callback will be called when user click on button.
    pub fn click_cb<F: 'static + FnMut(&mut Self)>(&mut self, cb: F) {
        self.cb = Some(Box::new(cb));
    }
}

impl Widget for Button {
    fn handle_event(
        &mut self,
        self_rc: WRef,
        event: Event,
        state: &mut State,
    ) -> Result<(), Error> {
        match event {
            Event::MousePress(mouse_button) => {
                if matches!(mouse_button, MouseButton::Left) && state.get_caught().is_none() {
                    self.set_position(self.get_global_position());
                    self.get_parent()
                        .map(|p| p.upgrade().map(|p| p.borrow_mut().erase_widget(&self_rc)));
                    state.catch_self(self, self_rc)?;
                    self.state = true;
                }
            }
            Event::MouseRelease(mouse_button) => {
                if matches!(mouse_button, MouseButton::Left) && state.is_caught(self_rc.clone()) {
                    self.state = false;
                    if self.check_bounds(state.mouse) {
                        if let Some(mut cb) = self.cb.take() {
                            cb(self);
                            self.cb = Some(cb);
                        }
                    }
                    state.uncatch(self, self_rc.clone())?;
                    self.get_parent().map(|p| {
                        p.upgrade().map(|p| {
                            p.clone().borrow_mut().add_widget(p.into(), self, self_rc);
                        })
                    });
                    self.hovered = self.check_bounds(state.mouse);
                    self.set_global_position(self.get_position());
                }
            }
            Event::MouseEnter => self.hovered = true,
            Event::MouseLeave => self.hovered = state.is_caught(self_rc),
            Event::MouseMove
            | Event::TextInput(_)
            | Event::Caught
            | Event::Released
            | Event::Focused
            | Event::Unfocused
            | Event::KeyPress(_)
            | Event::KeyRelease(_) => {}
        }
        Ok(())
    }

    fn get_hovered(&self, pos: Vec2f) -> Option<WRef> {
        self.base.get_hovered(pos)
    }

    fn check_bounds(&self, pos: Vec2f) -> bool {
        self.base.check_bounds(pos)
    }

    fn add_widget(&mut self, self_ref: WRef, widget: &mut dyn Widget, widget_ref: WRef) {
        self.base.add_widget(self_ref, widget, widget_ref);
    }

    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Widget>>>) {
        self.base.set_parent(parent);
    }

    fn get_parent(&mut self) -> Option<Weak<RefCell<dyn Widget>>> {
        self.base.get_parent()
    }

    fn detach(&mut self, self_rc: &WRef) {
        self.base.detach(self_rc);
    }

    fn erase_widget(&mut self, widget: &WRef) {
        self.base.erase_widget(widget);
    }

    fn set_position(&mut self, pos: Vec2f) {
        self.base.set_position(pos);
    }

    fn get_position(&self) -> Vec2f {
        self.base.get_position()
    }

    fn set_global_position(&mut self, pos: Vec2f) {
        self.base.set_global_position(pos);
    }

    fn get_global_position(&self) -> Vec2f {
        self.base.get_global_position()
    }

    fn get_rect(&self) -> &Rect<f64> {
        self.base.get_rect()
    }

    fn find(&self, id: &str) -> Option<WRef> {
        self.base.find(id)
    }

    fn get_id(&self) -> String {
        self.base.get_id()
    }
}

impl Drawable for Button {
    fn draw(&self, renderer: &mut dyn Renderer) {
        let rect = if self.state {
            &self.texture_rect_pressed
        } else if self.hovered {
            &self.texture_rect_hovered
        } else {
            &self.texture_rect
        };
        renderer.draw_img(self.base.get_rect(), self.texture, rect);
        self.base.draw(renderer);
    }
}

impl BuildFromCfg<WRef> for Button {
    fn build(mut cfg: Config, res: &mut dyn resources::Manger) -> Result<WRef, builder::Error> {
        let bg_name = cfg
            .take::<String>("background")
            .change_context(builder::Error::msg("Failed to init button background texture"))?;
        let texture = res.get_texture(&bg_name).change_context(builder::Error::msg(format!(
            "Failed to init button, texture: \"{bg_name}\" not found"
        )))?;

        let mut get_rect = |name| -> Result<Rect<f64>, builder::Error> {
            Ok(cfg
                .take::<[f64; 4]>(name)
                .change_context(builder::Error::msg("Failed to init button"))?
                .into())
        };

        Ok(WRef::new(Self {
            hovered: false,
            state: false,
            texture,
            texture_rect_pressed: get_rect("texture_rect_pressed")?,
            texture_rect_hovered: get_rect("texture_rect_hovered")?,
            texture_rect: get_rect("texture_rect")?,
            base: Base::new(cfg)?,
            cb: None,
        }))
    }
}
