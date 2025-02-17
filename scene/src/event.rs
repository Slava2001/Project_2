//! Scene events.

/// Mouse buttons.
#[derive(Clone)]
pub enum MouseButton {
    /// Mouse left button.
    Left,
    /// Mouse right button.
    Right,
    /// Mouse middle button.
    Middle,
}

/// Scene events.
#[derive(Clone)]
pub enum Event {
    /// Mouse press button event.
    MousePress(MouseButton),
    /// Mouse release button event.
    MouseRelease(MouseButton),
    /// Mouse move event.
    MouseMove(f64, f64),
    /// Text input event
    TextInput(String),
    /// Keyboard key press event, arg: [`KeyCode`].
    KeyPress(KeyCode),
    /// Keyboard key release event, arg: [`KeyCode`].
    KeyRelease(KeyCode),
}

/// Keyboard button codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
