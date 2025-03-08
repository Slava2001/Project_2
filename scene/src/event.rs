//! Scene events.

use crate::TimeTick;
use utils::vec2::Vec2f;

/// Mouse buttons.
#[derive(Clone, Debug)]
pub enum MouseButton {
    /// Mouse left button.
    Left,
    /// Mouse right button.
    Right,
    /// Mouse middle button.
    Middle,
}

/// Scene events.
#[derive(Clone, Debug)]
pub enum Event {
    /// Mouse press button event.
    MousePress(MouseButton),
    /// Mouse release button event.
    MouseRelease(MouseButton),
    /// Mouse move event.
    MouseMove(Vec2f),
    /// Text input event
    TextInput(String),
    /// Keyboard key press event, arg: [`KeyCode`].
    KeyPress(KeyCode),
    /// Keyboard key release event, arg: [`KeyCode`].
    KeyRelease(KeyCode),
    /// Time tick. Used for update time depended object.
    TimeTick(TimeTick),
    /// Window resize.
    Resize(Vec2f),
}

/// Keyboard button codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum KeyCode {
    Escape,
    Backspace,
    Tab,
    F1,
    Enter,
    ArrowUp,
    ArrowDown,
    ArrowRight,
    ArrowLeft,
    Home,
    End,
    Delete,
}
