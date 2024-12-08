#[derive(Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Copy)]
pub enum InputEvent {
    MouseClick(MouseButton),
    MouseMove(f64, f64),
}
