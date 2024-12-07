pub enum MouseButton {
    Left,
    Right,
    Midle,
}

pub enum InputEvent {
    MouseClick(MouseButton),
    MouseMove(f64, f64),
}
