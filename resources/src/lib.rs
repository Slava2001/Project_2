//! Resource manager.
//!
//! Resource Manager manages the resources needed for GUI operation (textures, sounds, etc.).

use error_stack::Result;
use std::path::Path;

/// Resource manger error.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message.
    pub fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

/// Texture identifier.
#[derive(Clone, Copy)]
pub struct TextureId(pub usize);

/// Font identifier.
#[derive(Clone, Copy)]
pub struct FontId(pub usize);

/// Resource manager.
pub trait Manager {
    /// Load specified resource.
    ///
    /// # Errors
    /// Return error if failed to load specified resource.
    fn load(&mut self, kind: &str, name: &str, path: &Path) -> Result<(), Error>;

    /// Get texture identifier by name.
    ///
    /// Before use, texture must be loaded with kind: "texture".
    ///
    /// # Errors
    /// Return error if the specified texture was not loaded.
    fn get_texture(&self, name: &str) -> Result<TextureId, Error>;

    /// Get font identifier by name.
    ///
    /// Before use, font must be loaded with kind: "font".
    ///
    /// # Errors
    /// Return error if the specified font was not loaded.
    fn get_font(&self, name: &str) -> Result<FontId, Error>;
}
