//! Config.
//! Allows you to upload configuration files in different formats (json, yaml, etc.).
//! In addition, it supports the inclusion of configuration files (format: "file!:<path>").
//! Example:
//! ```json
//! {
//!     "included_cfg": "file!:other_cfg.yml"
//! }
//! ```

use config::{Config as Cfg, File, Value, ValueKind};
use error_stack::{bail, ensure, Result, ResultExt};
use serde::Deserialize;
use std::{any::Any, collections::HashMap, path::Path};

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
    cfg: HashMap<String, Value>,
}

impl Config {
    /// Creates new config from file.
    ///
    /// # Errors
    /// Return error if failed to load file.
    pub fn new(path: &str) -> Result<Self, Error> {
        let mut cfg = Cfg::builder()
            .add_source(File::with_name(path))
            .build()
            .change_context(Error::msg(format!("Failed to load config file: {path}")))?
            .try_deserialize::<Self>()
            .change_context(Error::msg(format!("Failed to parse config file as table: {path}")))?;
        cfg.file = path.into();
        Ok(cfg)
    }

    /// Take option value.
    /// if the required field is in the config, it is retrieved and returned as `Some(T)`, otherwise `None`.
    ///
    /// # Errors
    /// Return error if required field exist, but has unexpected type.
    pub fn take_opt<T: 'static + Deserialize<'static>>(
        &mut self,
        key: &str,
    ) -> Result<Option<T>, Error> {
        let Some(mut val) = self.cfg.remove(key) else {
            return Ok(None);
        };
        let mut include_path = self.file.clone();
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Self>() {
            if let ValueKind::String(ref path) = val.kind {
                if let Some(path) = path.strip_prefix("file!:") {
                    include_path = Self::calc_path(&include_path, path).change_context(
                        Error::msg(format!(
                            "Failed to calculate path to include config: \
                                            file: \"{include_path}\" field: \"{key}\""
                        )),
                    )?;
                    val = Cfg::builder()
                        .add_source(File::with_name(&include_path))
                        .build()
                        .change_context(Error::msg(format!(
                            "Failed to include config file: {include_path}"
                        )))?
                        .cache;
                }
            }
        }

        let mut val = val.try_deserialize::<T>().change_context(Error::msg(format!(
            "Failed to parse field \"{}\" as {}",
            key,
            std::any::type_name::<T>()
        )))?;

        let as_any: &mut dyn Any = &mut val;
        if let Some(cfg) = as_any.downcast_mut::<Self>() {
            cfg.file = include_path;
        } else if let Some(cfg) = as_any.downcast_mut::<Vec<Self>>() {
            for c in cfg.iter_mut() {
                c.file.clone_from(&include_path);
            }
        }

        Ok(Some(val))
    }

    /// Take value.
    /// if the required field is in the config, it is retrieved and returned.
    ///
    /// # Errors
    /// Return error if required field dos not exist or exist, but has unexpected type.
    pub fn take<T: 'static + Deserialize<'static>>(&mut self, key: &str) -> Result<T, Error> {
        Ok(self
            .take_opt(key)?
            .ok_or_else(|| Error::msg(format!("Config does not contain the \"{key}\" field")))?)
    }

    /// Calculate path to include file relative root file.
    ///
    /// # Errors
    /// Return error if failed to calculate path.
    fn calc_path(root_file: &str, include_file: &str) -> Result<String, Error> {
        let root = Path::new(root_file);
        ensure!(
            root.file_name().is_some(),
            Error::msg(format!("Path \"{root_file}\" is not a path to root file"))
        );
        let include = Path::new(include_file);
        ensure!(
            include.file_name().is_some(),
            Error::msg(format!("Path \"{include_file}\" is not a path to include file"))
        );

        let Some(parent) = root.parent() else {
            bail!(Error::msg(format!("Failed to find path \"{}\" parent", root.display())));
        };
        Ok(parent.join(include).display().to_string())
    }
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self { file: String::new(), cfg: Deserialize::deserialize(deserializer)? })
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    #[cfg(target_family = "unix")]
    fn calc_path_test() {
        assert_eq!(Config::calc_path("./cfg/cfg.json", "oth.json").unwrap(), "./cfg/oth.json");
        assert_eq!(Config::calc_path("./cfg/cfg.json", "./oth.json").unwrap(), "./cfg/./oth.json");
        assert_eq!(Config::calc_path("./cfg.json", "./oth.json").unwrap(), "././oth.json");
        assert_eq!(Config::calc_path("cfg.json", "./oth.json").unwrap(), "./oth.json");
        assert_eq!(Config::calc_path("/cfg.json", "./oth.json").unwrap(), "/./oth.json");
        assert_eq!(Config::calc_path("/cfg.json", "../oth.json").unwrap(), "/../oth.json");
        assert_eq!(Config::calc_path("/cfg/cfg.json", "../oth.json").unwrap(), "/cfg/../oth.json");
        assert!(Config::calc_path("/", "./oth.json").is_err());
    }
}
