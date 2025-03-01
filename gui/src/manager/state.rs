//! GUI manager state.
use super::widget::{event::Event, Error, WRef, Widget};
use error_stack::Result;
use utils::vec2::Vec2f;

/// Manager state.
pub struct State {
    /// Caught widget.
    caught: Option<WRef>,
    /// Focused widget.
    focused: Option<WRef>,
    /// Cursor position.
    pub mouse: Vec2f,
    /// Hovered widget.
    pub(super) hovered: WRef,
}

impl State {
    /// Create new state.
    #[must_use]
    pub(super) fn new(hovered: WRef) -> Self {
        Self { hovered, caught: None, mouse: (0.0, 0.0).into(), focused: None }
    }

    /// Check if specified widget is hovered.
    #[must_use]
    pub fn is_hovered(&self, wref: &WRef) -> bool {
        self.hovered == *wref
    }

    /// Get current caught widget.
    #[must_use]
    pub fn get_caught(&self) -> Option<WRef> {
        self.caught.clone()
    }

    /// Check if specified widget is caught.
    #[must_use]
    pub fn is_caught(&self, wref: WRef) -> bool {
        self.caught == Some(wref)
    }

    /// Catch a widget.
    /// The function takes pointers to the widget that calls this method and the widget that
    /// needs to be captured.
    /// If you need to capture the widget that calls the method, use [`catch_self`].
    ///
    /// # Errors
    /// Return error if widget hailed to handle caught event.
    ///
    /// [`catch_self`]: State::catch_self
    pub fn catch(&mut self, s: &mut dyn Widget, s_ref: WRef, wref: WRef) -> Result<(), Error> {
        self.uncatch(s, s_ref)?;
        wref.borrow_mut().handle_event(wref.clone(), Event::Caught, self)?;
        self.caught = Some(wref);
        Ok(())
    }

    /// Catch a widget, that calls this method.
    ///
    /// # Errors
    /// Return error if widget hailed to handle caught event.
    pub fn catch_self(&mut self, s: &mut dyn Widget, s_ref: WRef) -> Result<(), Error> {
        self.uncatch(s, s_ref.clone())?;
        s.handle_event(s_ref.clone(), Event::Caught, self)?;
        self.caught = Some(s_ref);
        Ok(())
    }

    /// Release a widget.
    ///
    /// # Errors
    /// Return error if widget hailed to handle release event.
    pub fn uncatch(&mut self, s: &mut dyn Widget, s_ref: WRef) -> Result<(), Error> {
        if let Some(w) = self.caught.take() {
            if w == s_ref {
                s.handle_event(s_ref, Event::Released, self)?;
            } else {
                w.borrow_mut().handle_event(w.clone(), Event::Released, self)?;
            }
        }
        Ok(())
    }

    /// Get current focused widget.
    #[must_use]
    pub fn get_focused(&self) -> Option<WRef> {
        self.focused.clone()
    }

    /// Check if specified widget is focused.
    #[must_use]
    pub fn is_focused(&self, wref: WRef) -> bool {
        self.focused == Some(wref)
    }

    /// Focus a widget.
    /// The function takes pointers to the widget that calls this method and the widget that
    /// needs to be captured.
    /// If you need to capture the widget that calls the method, use [`focus_self`].
    ///
    /// # Errors
    /// Return error if widget hailed to handle focus event.
    ///
    /// [`focus_self`]: State::focus_self
    pub fn focus(&mut self, s: &mut dyn Widget, s_ref: WRef, wref: WRef) -> Result<(), Error> {
        self.unfocus(s, s_ref)?;
        wref.borrow_mut().handle_event(wref.clone(), Event::Focused, self)?;
        self.focused = Some(wref);
        Ok(())
    }

    /// Focus a widget, that calls this method.
    ///
    /// # Errors
    /// Return error if widget hailed to handle focus event.
    pub fn focus_self(&mut self, s: &mut dyn Widget, s_ref: WRef) -> Result<(), Error> {
        self.unfocus(s, s_ref.clone())?;
        s.handle_event(s_ref.clone(), Event::Focused, self)?;
        self.focused = Some(s_ref);
        Ok(())
    }

    /// Unfocus a widget.
    ///
    /// # Errors
    /// Return error if widget hailed to handle unfocus event.
    pub fn unfocus(&mut self, s: &mut dyn Widget, s_ref: WRef) -> Result<(), Error> {
        if let Some(w) = self.focused.take() {
            if w == s_ref {
                s.handle_event(s_ref, Event::Unfocused, self)?;
            } else {
                w.borrow_mut().handle_event(w.clone(), Event::Unfocused, self)?;
            }
        }
        Ok(())
    }
}
