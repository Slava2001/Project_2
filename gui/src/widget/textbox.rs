//! Textbox.
//!
//! Textbox widget, that used for input and display text.

use crate::manager::{
    widget::{
        event::{Event, MouseButton},
        Error, WRef, Widget,
    },
    State,
};
use builder::{self, BuildFromCfg, Config};
use error_stack::Result;
use renderer::{rect::Rect, vec2::Vec2f, Drawable, Renderer, TextTruncateMode};
use scene::event::Scancode;
use std::{cell::RefCell, rc::Weak};

use super::Label;

/// Cursor char.
const CURSOR_CHAR: char = '▎';

/// Textbox widget.
pub struct Textbox {
    /// Base widget.
    base: Label,
    /// Last pressed key.
    last_key: Option<i32>,
}

impl Textbox {
    /// Create new textbox.
    ///
    /// # Errors
    /// Return error if the config is incorrect or the required resource is not found.
    pub fn new(cfg: Config, res: &mut dyn resources::Manger) -> Result<Self, builder::Error> {
        let mut base = Label::new(cfg, res)?;
        base.set_text_truncating(false);
        Ok(Self { base, last_key: None })
    }
}

impl Widget for Textbox {
    fn handle_event(
        &mut self,
        self_rc: WRef,
        event: Event,
        state: &mut State,
    ) -> Result<(), Error> {
        match event {
            Event::MousePress(mouse_button) => {
                if matches!(mouse_button, MouseButton::Left) {
                    if state.is_hovered(&self_rc) {
                        state.focus_self(self, self_rc)?;
                    } else if state.is_focused(self_rc.clone()) {
                        state.unfocus(self, self_rc)?;
                    }
                }
            }
            Event::TextInput(txt) if !txt.is_empty() => {
                if state.get_focused() == Some(self_rc) {
                    self.base.text_mut().pop();
                    *self.base.text_mut() += &txt;
                    self.base.text_mut().push(CURSOR_CHAR);
                }
            }
            Event::TextInput(_) => {
                if Some(Scancode::BACKSPACE) == self.last_key {
                    self.base.text_mut().pop();
                    self.base.text_mut().pop();
                    self.base.text_mut().push(CURSOR_CHAR);
                }
            }
            Event::Focused => {
                self.base.text_mut().push(CURSOR_CHAR);
                self.base.set_draw_truncate_mode(TextTruncateMode::Front);
            }
            Event::Unfocused => {
                self.base.text_mut().pop();
                self.base.set_draw_truncate_mode(TextTruncateMode::Back);
            }
            Event::KeyPress(k) if self.last_key.is_none() => self.last_key = Some(k),
            Event::KeyRelease(k) if self.last_key == Some(k) => self.last_key = None,
            Event::MouseRelease(_)
            | Event::KeyPress(_)
            | Event::KeyRelease(_)
            | Event::MouseMove
            | Event::MouseEnter
            | Event::MouseLeave
            | Event::Caught
            | Event::Released => {}
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

impl Drawable for Textbox {
    fn draw(&self, renderer: &mut dyn Renderer) {
        self.base.draw(renderer);
    }
}

impl BuildFromCfg<WRef> for Textbox {
    fn build(cfg: Config, res: &mut dyn resources::Manger) -> Result<WRef, builder::Error> {
        Ok(WRef::new(Self::new(cfg, res)?))
    }
}
