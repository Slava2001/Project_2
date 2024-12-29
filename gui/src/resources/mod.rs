//! Resource manager
//!
//! Resource Manager manages the resources needed for GUI operation (textures, sounds, etc.)

use error_stack::Result;

/// Resource manger error
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message
    pub fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

/// Texture identifier
#[derive(Clone, Copy)]
pub struct TextureId(pub usize);

/// Resource manager
pub trait Manger {
    /// Load specified resource
    ///
    /// # Errors
    /// Error is failed to load specified resource
    fn load(&mut self, kind: &str, name: &str, path: &str) -> Result<TextureId, Error>;

    /// Get texture identifier by name
    ///
    /// Before use, texture must be loaded with kind: "texture"
    ///
    /// # Errors
    /// Return error if the specified texture was not loaded
    fn get_texture(&self, name: &str) -> Result<TextureId, Error>;
}