//! Config value.
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use super::Config;
use error_stack::{bail, ensure, Result};

/// Parse error.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message.
    pub fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

/// Config value.
pub struct Value {
    /// Real value.
    pub(super) val: config::Value,
    /// Path to config file.
    pub(super) path: String,
}

/// Implement `TryFrom<Value>` for deserializable types.
macro_rules! impl_try_from_value {
    ($($t:ty),*) => {
        $(impl TryFrom<Value> for $t {
            type Error = Error;
            fn try_from(val: Value) -> std::result::Result<Self, Self::Error> {
                val.val.try_deserialize::<$t>().map_err(|e|
                    Error::msg(format!(
                        "Failed parse value as {:?}, error: {}",
                        std::any::type_name::<$t>(),
                        e
                    ))
                )
            }
        })*
    };
}

impl_try_from_value!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool, char, String
);

impl TryFrom<Value> for PathBuf {
    type Error = Error;
    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        let path = value
            .val
            .try_deserialize::<Self>()
            .map_err(|e| Error::msg(format!("Failed to parse value as path, error: {e}")))?;
        let file = Self::from(value.path);
        let parent = file.parent().ok_or_else(|| {
            Error::msg(format!("Failed to get path parent: path: {}", file.display()))
        })?;
        let path = parent.join(path);
        Ok(path)
    }
}

impl<E: std::error::Error, T: TryFrom<Value, Error = E>> TryFrom<Value> for Vec<T> {
    type Error = Error;
    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        let v = value
            .val
            .try_deserialize::<Vec<config::Value>>()
            .map_err(|e| Error::msg(format!("Failed to parse value as Vec<_>, errorL {e}")))?;
        let mut res = Self::new();
        for (i, val) in v.into_iter().enumerate() {
            let val = Value { val, path: value.path.clone() };
            res.push(T::try_from(val).map_err(|e| {
                Error::msg(format!("Failed to parse vector item. Index: {i}, error: {e}"))
            })?);
        }
        Ok(res)
    }
}

#[allow(clippy::implicit_hasher)]
impl<E: std::error::Error, T: TryFrom<Value, Error = E>> TryFrom<Value> for HashMap<String, T> {
    type Error = Error;
    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        let v = value.val.try_deserialize::<HashMap<String, config::Value>>().map_err(|e| {
            Error::msg(format!("Failed to parse value as HashMap<String, _>, error: {e}"))
        })?;
        let mut res = Self::new();
        for (k, val) in v {
            let val = Value { val, path: value.path.clone() };
            res.insert(
                k,
                T::try_from(val).map_err(|e| {
                    Error::msg(format!("Failed to parse vector item: error: {e:?}"))
                })?,
            );
        }
        Ok(res)
    }
}

impl<E: std::error::Error, T: TryFrom<Value, Error = E>, const L: usize> TryFrom<Value> for [T; L] {
    type Error = Error;
    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        let res = Vec::<T>::try_from(value)?;
        TryInto::<[T; L]>::try_into(res).map_err(|_| Error::msg("Failed to parse value as array"))
    }
}

impl TryFrom<Value> for Config {
    type Error = Error;
    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        let mut file = value.path.clone();
        let mut val = value.val;
        if let config::ValueKind::String(ref path) = val.kind {
            if let Some(path) = path.strip_prefix("file!:") {
                file = calc_path(&file, path).map_err(|e| {
                    Error::msg(format!(
                        "Failed to calculate path to include config: \
                     file: {:?} to file: {:?}, error: {}",
                        file, value.path, e
                    ))
                })?;
                val = config::Config::builder()
                    .add_source(config::File::with_name(&file))
                    .build()
                    .map_err(|e| {
                        Error::msg(format!("Failed to include config file: {file:?}, error: {e}"))
                    })?
                    .cache;
            }
        }
        let cfg = val
            .into_table()
            .map_err(|e| Error::msg(format!("Failed to parse value as table, error: {e}")))?;
        Ok(Self { file, cfg })
    }
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

/// Implement `From<T> for Value` for deserializable types.
macro_rules! impl_into_value {
    ($($t:ty),*) => {
        $(impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Self { val: config::Value::new(None, value), path: String::new() }
            }
        })*
    };
}

impl_into_value!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f64, bool, String);

impl<T: Into<Self>> From<Vec<T>> for Value {
    fn from(value: Vec<T>) -> Self {
        let val = value.into_iter().map(|v| Into::<Self>::into(v).val).collect::<Vec<_>>();
        Self { val: config::Value::from(val), path: String::new() }
    }
}

impl<T: Into<Self>> From<HashMap<String, T>> for Value {
    fn from(value: HashMap<String, T>) -> Self {
        let val = value
            .into_iter()
            .map(|(k, v)| (k, Into::<Self>::into(v).val))
            .collect::<HashMap<String, _>>();
        Self { val: config::Value::from(val), path: String::new() }
    }
}

impl<T: Into<Self>, const L: usize> From<[T; L]> for Value {
    fn from(value: [T; L]) -> Self {
        let val = value.into_iter().map(|v| Into::<Self>::into(v).val).collect::<Vec<_>>();
        Self { val: config::Value::from(val), path: String::new() }
    }
}

impl From<Config> for Value {
    fn from(value: Config) -> Self {
        Self { val: config::Value::from(value.cfg), path: String::new() }
    }
}

#[cfg(test)]
#[cfg(target_family = "unix")]
mod tests {
    use super::calc_path;

    #[test]
    fn calc_path_test() {
        assert_eq!(calc_path("./cfg/cfg.json", "oth.json").unwrap(), "./cfg/oth.json");
        assert_eq!(calc_path("./cfg/cfg.json", "./oth.json").unwrap(), "./cfg/./oth.json");
        assert_eq!(calc_path("./cfg.json", "./oth.json").unwrap(), "././oth.json");
        assert_eq!(calc_path("cfg.json", "./oth.json").unwrap(), "./oth.json");
        assert_eq!(calc_path("/cfg.json", "./oth.json").unwrap(), "/./oth.json");
        assert_eq!(calc_path("/cfg.json", "../oth.json").unwrap(), "/../oth.json");
        assert_eq!(calc_path("/cfg/cfg.json", "../oth.json").unwrap(), "/cfg/../oth.json");
        assert!(calc_path("/", "./oth.json").is_err());
    }
}
