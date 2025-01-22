//! Scene manager.
mod builder;
pub mod event;
pub use builder::Builder;

use error_stack::Result;
use renderer::Drawable;

/// Scene error.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message.
    pub fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

/// Scene interface.
pub trait Scene: Drawable {
    /// Handles the event.
    ///
    /// # Errors
    /// Return error if the event could not be handled.
    fn handle_event(&mut self, e: event::Event) -> Result<(), Error>;
}
