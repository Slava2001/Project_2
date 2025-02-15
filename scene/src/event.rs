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
    /// Keyboard key press event, arg: [`Scancode`].
    KeyPress(i32),
    /// Keyboard key release event, arg: [`Scancode`].
    KeyRelease(i32),
}

pub struct Scancode;
#[allow(dead_code)]
impl Scancode {
    pub const ESCAPE: i32 = 1;
    pub const BACKSPACE: i32 = 14;
    pub const TAB: i32 = 15;
    pub const F1: i32 = 59;
    pub const ENTER: i32 = 28;
}
