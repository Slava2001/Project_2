//! Widget events
use super::super::input_event::InputEvent;

/// Widget events
pub enum Event {
    /// Some input event [`InputEvent`]
    InputEvent(InputEvent),
    /// Cursor enter in widget bounds
    MouseEnter,
    /// Cursor leave from widget bounds
    MouseLeave,
}
