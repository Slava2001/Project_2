use super::super::input_event::InputEvent;

pub enum Event {
    InputEvent(InputEvent),
    MouseEnter,
    MouseLeave,
}
