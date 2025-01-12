//! Scene manager
use renderer::Drawable;

/// Scene error
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message
    fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

pub enum Event {}


pub trait Scene: Drawable {
    fn handle_event(e: Event) -> Result<(), Error>;
}
