//! Scene manager
pub mod event;
pub mod runtime;

use renderer::Drawable;
use error_stack::Result;

/// Scene error
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message
    pub fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

pub trait Scene: Drawable {
    fn handle_event(&mut self, e: event::Event) -> Result<(), Error>;
}
