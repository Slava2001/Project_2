//! Widget events

#[derive(Clone, Copy)]
pub enum MouseButton {
    /// Mouse left button
    Left,
    /// Mouse right button
    Right,
    /// Mouse middle button
    Middle,
}

/// Widget events
#[derive(Clone, Copy)]
pub enum Event {
    /// Mouse press button event
    MousePress(MouseButton),
    /// Mouse release button event
    MouseRelease(MouseButton),
    /// Cursor moving event
    MouseMove,
    /// Cursor enter in widget bounds
    MouseEnter,
    /// Cursor leave from widget bounds
    MouseLeave,
}

/// Event conversion error.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message
    pub fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

impl TryFrom<scene::event::Event> for Event {
    type Error = Error;

    fn try_from(value: scene::event::Event) -> Result<Self, Self::Error> {
        use scene::event;
        Ok(match value {
            event::Event::MousePress(b) => Event::MousePress(b.try_into()?),
            event::Event::MouseRelease(b) => Event::MouseRelease(b.try_into()?),
            event::Event::MouseMove(..) => Event::MouseMove,
        })
    }
}

impl TryFrom<scene::event::MouseButton> for MouseButton {
    type Error = Error;

    fn try_from(value: scene::event::MouseButton) -> Result<Self, Self::Error> {
        use scene::event;
        Ok(match value {
            event::MouseButton::Left => MouseButton::Left,
            event::MouseButton::Right => MouseButton::Right,
            event::MouseButton::Middle => MouseButton::Middle,
        })
    }
}
