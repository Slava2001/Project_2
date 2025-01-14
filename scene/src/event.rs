//! GUI input events

/// Mouse buttons
#[derive(Clone, Copy)]
pub enum MouseButton {
    /// Mouse left button
    Left,
    /// Mouse right button
    Right,
    /// Mouse middle button
    Middle,
}

/// GUI input event
#[derive(Clone, Copy)]
pub enum Event {
    /// Mouse press button event
    MousePress(MouseButton),
    /// Mouse release button event
    MouseRelease(MouseButton),
    /// Mouse move event
    MouseMove(f64, f64),
}
