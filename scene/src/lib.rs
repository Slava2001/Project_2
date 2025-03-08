//! Scene manager.
mod builder;
pub mod event;

pub use builder::Builder;

use ::builder::config::Config;
use error_stack::Result;
use renderer::Drawable;
use resources::Manager as ResManger;

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

/// Time unit.
pub type TimeTick = usize;

/// Scene state.
pub trait State {
    /// Request load next scene by config.
    /// Scene will be load on next frame.
    ///
    /// # Errors
    /// Return errors if loading scene already requested (two request on one frame).
    fn load_next_scene(&mut self, cfg: Config) -> Result<(), Error>;

    /// Request program termination.
    fn exit(&mut self);

    /// Get resource manager.
    fn get_resources_manager(&mut self) -> &dyn ResManger;

}

/// Scene interface.
pub trait Scene: Drawable {
    /// Handles the event.
    ///
    /// # Errors
    /// Return error if the event could not be handled.
    fn handle_event(&mut self, e: event::Event, state: &mut dyn State) -> Result<(), Error>;
}
