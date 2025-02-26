//! Config.
//! Allows you to load configuration files in different formats (json, yaml, etc.).
//! In addition, it supports the inclusion of configuration files (format: "file!:<path/to/file>").
//! Example:
//! ```json
//! {
//!     "included_cfg": "file!:other_cfg.yml"
//! }
//! ```
//!
//! Also config autocomplete relative path.
//! //! Example:
//! ```json
//! ./assets/config.json:
//! {
//!     "path_to_img": "./textures/img.png"
//! }
//! ```
//! ```no_run
//! # use std::path::PathBuf;
//! # use builder::Config;
//! # let mut cfg = Config::from_json("").unwrap();
//! let path = cfg.take::<PathBuf>("path_to_img").unwrap();
//! assert!(path.to_string_lossy() == "./assets/textures/img.png");
//! ```

pub mod value;

use config::{Config as Cfg, File, FileFormat};
use error_stack::{Result, ResultExt};
use std::collections::HashMap;
use value::{ParseFormValue, Value};

/// Config error.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message.
    pub fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

/// Config.
pub struct Config {
    /// Path to config file.
    file: String,
    /// Config.
    cfg: HashMap<String, config::Value>,
}

impl Config {
    /// Creates new config from file.
    ///
    /// # Errors
    /// Return error if failed to load or parse file.
    pub fn from_file(path: &str) -> Result<Self, Error> {
        let val = Value {
            path: path.into(),
            val: Cfg::builder()
                .add_source(File::with_name(path))
                .build()
                .change_context(Error::msg(format!("Failed to load config file: {path}")))?
                .cache,
        };
        Self::parse(val)
            .change_context(Error::msg(format!("Failed to parse file {path} as config")))
    }

    /// Creates new config from json5 str.
    ///
    /// # Errors
    /// Return error if failed to parse provided config.
    pub fn from_json(json: &str) -> Result<Self, Error> {
        let val = Value {
            path: ".".into(),
            val: Cfg::builder()
                .add_source(File::from_str(json, FileFormat::Json5))
                .build()
                .change_context(Error::msg(format!(
                    "Failed to load config from json str: {json:?}"
                )))?
                .cache,
        };
        Self::parse(val)
            .change_context(Error::msg(format!("Failed to parse json {json:?} as config")))
    }

    /// Take option value.
    /// if the required field is in the config, it is retrieved and returned as `Some(T)`,
    /// otherwise `None`.
    ///
    /// # Errors
    /// Return error if required field exist, but has unexpected type.
    pub fn take_opt<T: ParseFormValue>(&mut self, key: &str) -> Result<Option<T>, Error> {
        let Some(val) = self.cfg.remove(key) else {
            return Ok(None);
        };
        let val = Value { val, path: self.file.clone() };
        let val = T::parse(val).change_context(Error::msg(format!(
            "Failed to parse field \"{}\" as {}",
            key,
            std::any::type_name::<T>()
        )))?;
        Ok(Some(val))
    }

    /// Take value.
    /// if the required field is in the config, it is retrieved and returned.
    ///
    /// # Errors
    /// Return error if required field dos not exist or exist, but has unexpected type.
    pub fn take<T: ParseFormValue>(&mut self, key: &str) -> Result<T, Error> {
        Ok(self
            .take_opt(key)?
            .ok_or_else(|| Error::msg(format!("Config does not contain the \"{key}\" field")))?)
    }
}
