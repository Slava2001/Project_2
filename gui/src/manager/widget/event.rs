//! Widget events.

/// Mouse buttons.
#[derive(Clone, Copy)]
pub enum MouseButton {
    /// Mouse left button.
    Left,
    /// Mouse right button.
    Right,
    /// Mouse middle button.
    Middle,
}

/// Widget events.
#[derive(Clone)]
pub enum Event {
    /// Mouse press button event.
    MousePress(MouseButton),
    /// Mouse release button event.
    MouseRelease(MouseButton),
    /// Cursor moving event.
    MouseMove,
    /// Cursor enter in widget bounds.
    MouseEnter,
    /// Cursor leave from widget bounds.
    MouseLeave,
    /// Input text.
    TextInput(String),
    /// The cursor has captured the widget.
    Caught,
    /// Cursor released widget.
    Released,
    /// Widget got into focus.
    Focused,
    /// Widget went out of focus.
    Unfocused,
    /// Keyboard key press event, arg: [`Scancode`].
    ///
    /// [`Scancode`]: scene::event::Scancode
    KeyPress(i32),
    /// Keyboard key release event, arg: [`Scancode`].
    ///
    /// [`Scancode`]: scene::event::Scancode
    KeyRelease(i32)
}

/// Event conversion error.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message.
    pub fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

impl TryFrom<scene::event::Event> for Event {
    type Error = Error;

    fn try_from(value: scene::event::Event) -> Result<Self, Self::Error> {
        use scene::event;
        Ok(match value {
            event::Event::MousePress(b) => Self::MousePress(b.try_into()?),
            event::Event::MouseRelease(b) => Self::MouseRelease(b.try_into()?),
            event::Event::MouseMove(..) => Self::MouseMove,
            event::Event::TextInput(text) => Self::TextInput(text),
            event::Event::KeyPress(k) => Self::KeyPress(k),
            event::Event::KeyRelease(k) => Self::KeyRelease(k),

        })
    }
}

impl TryFrom<scene::event::MouseButton> for MouseButton {
    type Error = Error;

    fn try_from(value: scene::event::MouseButton) -> Result<Self, Self::Error> {
        use scene::event;
        Ok(match value {
            event::MouseButton::Left => Self::Left,
            event::MouseButton::Right => Self::Right,
            event::MouseButton::Middle => Self::Middle,
        })
    }
}
