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
use builder::{self, config::Config, BuildFromCfg};
use error_stack::{Result, ResultExt};
use renderer::{Drawable, Renderer, TextTruncateMode};
use scene::event::KeyCode;
use std::{cell::RefCell, rc::Weak};
use utils::{rect::Rectf, vec2::Vec2f};

use super::Label;

/// Textbox widget.
pub struct Textbox {
    /// Base widget.
    base: Label,
    /// Last pressed key.
    last_key: Option<KeyCode>,
    /// Is textbox focused.
    is_focused: bool,
    /// Cursor char.
    cursor: char,
    /// Cursor offset.
    cursor_offset: usize,
}

impl Textbox {
    /// Create new textbox.
    ///
    /// # Errors
    /// Return error if the config is incorrect or the required resource is not found.
    pub fn new(mut cfg: Config, res: &dyn resources::Manager) -> Result<Self, builder::Error> {
        let cursor = cfg
            .take_opt("cursor")
            .change_context(builder::Error::msg("Failed to init textbox cursor"))?
            .unwrap_or('\u{033F}');
        let mut base = Label::new(cfg, res)?;
        base.set_text_truncating(false);
        Ok(Self { base, last_key: None, is_focused: false, cursor, cursor_offset: 0 })
    }

    /// Get textbox text.
    pub fn get_text(&self) -> String {
        self.base.get_text()
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
                        if !state.is_focused(self_rc.clone()) {
                            state.focus_self(self, self_rc)?;
                        }
                    } else if state.is_focused(self_rc.clone()) {
                        state.unfocus(self, self_rc)?;
                    }
                }
            }
            Event::TextInput(txt) if !txt.is_empty() => {
                if self.is_focused && state.get_focused() == Some(self_rc) {
                    for c in txt.chars() {
                        self.base.chars_mut().insert(self.cursor_offset, c);
                        self.cursor_offset += 1;
                    }
                }
            }
            Event::TextInput(_) => {
                if self.is_focused {
                    if Some(KeyCode::Backspace) == self.last_key && self.cursor_offset > 0 {
                        self.base.chars_mut().remove(self.cursor_offset - 1);
                        self.cursor_offset -= 1;
                    } else if Some(KeyCode::Delete) == self.last_key
                        && self.cursor_offset < self.base.chars().len() - 1
                    {
                        self.base.chars_mut().remove(self.cursor_offset + 1);
                    }
                }
            }
            Event::Focused => {
                self.is_focused = true;
                self.cursor_offset = self.base.chars().len();
                self.base.chars_mut().push(self.cursor);
                self.base.set_draw_truncate_mode(TextTruncateMode::Front);
            }
            Event::Unfocused => {
                self.is_focused = false;
                self.base.chars_mut().remove(self.cursor_offset);
                self.last_key = None;
                self.base.set_draw_truncate_mode(TextTruncateMode::Back);
            }
            Event::KeyPress(k) => {
                if self.is_focused && self.last_key.is_none() {
                    match k {
                        KeyCode::ArrowLeft => {
                            if self.cursor_offset > 0 {
                                self.base
                                    .chars_mut()
                                    .swap(self.cursor_offset, self.cursor_offset - 1);
                                self.cursor_offset -= 1;
                            }
                        }
                        KeyCode::ArrowRight => {
                            if self.cursor_offset < self.base.chars().len() - 1 {
                                self.base
                                    .chars_mut()
                                    .swap(self.cursor_offset, self.cursor_offset + 1);
                                self.cursor_offset += 1;
                            }
                        }
                        KeyCode::Home => {
                            self.base.chars_mut().remove(self.cursor_offset);
                            self.cursor_offset = 0;
                            self.base.chars_mut().insert(0, self.cursor);
                        }
                        KeyCode::End => {
                            self.base.chars_mut().remove(self.cursor_offset);
                            self.cursor_offset = self.base.chars().len();
                            self.base.chars_mut().push(self.cursor);
                        }
                        _ => {}
                    }
                    self.last_key = Some(k);
                }
            }
            Event::KeyRelease(k) => {
                if self.is_focused && self.last_key == Some(k) {
                    self.last_key = None;
                }
            }
            Event::MouseRelease(_)
            | Event::MouseMove
            | Event::MouseEnter
            | Event::MouseLeave
            | Event::Caught
            | Event::Released => {}
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

impl Drawable for Textbox {
    fn draw(&self, renderer: &mut dyn Renderer) {
        self.base.draw(renderer);
    }
}

impl BuildFromCfg<WRef> for Textbox {
    fn build(cfg: Config, res: &mut dyn resources::Manager) -> Result<WRef, builder::Error> {
        Ok(WRef::new(Self::new(cfg, res)?))
    }
}
