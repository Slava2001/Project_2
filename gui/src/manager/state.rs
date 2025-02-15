use super::widget::{event::Event, Error, WRef, Widget};
use error_stack::Result;
use renderer::vec2::Vec2f;

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
    pub(super) fn new(hovered: WRef) -> Self {
        Self { hovered, caught: None, mouse: (0.0, 0.0).into(), focused: None }
    }

    pub fn is_hovered(&self, wref: WRef) -> bool {
        self.hovered == wref
    }

    pub fn get_caught(&self) -> Option<WRef> {
        self.caught.clone()
    }

    pub fn is_caught(&self, wref: WRef) -> bool {
        self.caught == Some(wref)
    }

    pub fn catch(&mut self, s: &mut dyn Widget, s_ref: WRef, wref: WRef) -> Result<(), Error> {
        self.uncatch(s, s_ref)?;
        wref.borrow_mut().handle_event(wref.clone(), Event::Caught, self)?;
        self.caught = Some(wref);
        Ok(())
    }

    pub fn catch_self(&mut self, s: &mut dyn Widget, s_ref: WRef) -> Result<(), Error> {
        self.uncatch(s, s_ref.clone())?;
        s.handle_event(s_ref.clone(), Event::Caught, self)?;
        self.caught = Some(s_ref);
        Ok(())
    }

    pub fn uncatch(&mut self, s: &mut dyn Widget, s_ref: WRef) -> Result<(), Error> {
        if let Some(w) = std::mem::replace(&mut self.caught, None) {
            if w == s_ref {
                s.handle_event(s_ref, Event::Released, self)?;
            } else {
                w.borrow_mut().handle_event(w.clone(), Event::Released, self)?;
            }
        }
        Ok(())
    }

    pub fn get_focused(&self) -> Option<WRef> {
        self.focused.clone()
    }

    pub fn is_focused(&self, wref: WRef) -> bool {
        self.focused == Some(wref)
    }

    pub fn focus(&mut self, s: &mut dyn Widget, s_ref: WRef, wref: WRef) -> Result<(), Error> {
        self.unfocus(s, s_ref)?;
        wref.borrow_mut().handle_event(wref.clone(), Event::Focused, self)?;
        self.focused = Some(wref);
        Ok(())
    }

    pub fn focus_self(&mut self, s: &mut dyn Widget, s_ref: WRef) -> Result<(), Error> {
        self.unfocus(s, s_ref.clone())?;
        s.handle_event(s_ref.clone(), Event::Focused, self)?;
        self.focused = Some(s_ref);
        Ok(())
    }

    pub fn unfocus(&mut self, s: &mut dyn Widget, s_ref: WRef) -> Result<(), Error> {
        if let Some(w) = std::mem::replace(&mut self.focused, None) {
            if w == s_ref {
                s.handle_event(s_ref, Event::Unfocused, self)?;
            } else {
                w.borrow_mut().handle_event(w.clone(), Event::Unfocused, self)?;
            }
        }
        Ok(())
    }
}
