//! Config value.
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use super::Config;
use error_stack::{bail, ensure, report, Result, ResultExt};

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

/// Parsable from config [`Value`] types.
pub trait ParseFormValue
where
    Self: Sized,
{
    /// Parse Self from config [`Value`].
    ///
    /// # Errors
    /// Return error if failed to parse [`Value`] as Self.
    fn parse(val: Value) -> Result<Self, Error>;
}

/// Implement [`ParseFormValue`] for deserializable types.
macro_rules! impl_parse_from_value {
    ($($t:ty),*) => {
        $(impl ParseFormValue for $t {
            fn parse(val: Value) -> Result<Self, Error> {
                val.val.try_deserialize::<$t>()
                .change_context(Error::msg(
                    format!("Failed parse value as {:?}", std::any::type_name::<$t>()))
                )
            }
        })*
    };
}

impl_parse_from_value!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool, char, String
);

impl ParseFormValue for PathBuf {
    fn parse(value: Value) -> Result<Self, Error> {
        let path = value
            .val
            .try_deserialize::<Self>()
            .change_context(Error::msg("Failed to parse value as path"))?;
        let file = Self::from(value.path);
        let parent = file.parent().ok_or_else(|| {
            report!(Error::msg(format!("Failed to get path parent: path: {}", file.display())))
        })?;
        let path = parent.join(path);
        Ok(path)
    }
}

impl<T: ParseFormValue> ParseFormValue for Vec<T> {
    fn parse(value: Value) -> Result<Self, Error> {
        let v = value
            .val
            .try_deserialize::<Vec<config::Value>>()
            .change_context(Error::msg("Failed to parse value as Vec<_>"))?;
        let mut res = Self::new();
        for val in v {
            let val = Value { val, path: value.path.clone() };
            res.push(T::parse(val).change_context(Error::msg("Failed to parse vector item"))?);
        }
        Ok(res)
    }
}

impl<T: ParseFormValue> ParseFormValue for HashMap<String, T> {
    fn parse(value: Value) -> Result<Self, Error> {
        let v = value
            .val
            .try_deserialize::<HashMap<String, config::Value>>()
            .change_context(Error::msg("Failed to parse value as HashMap<String, _>"))?;
        let mut res = Self::new();
        for (k, val) in v {
            let val = Value { val, path: value.path.clone() };
            res.insert(k, T::parse(val).change_context(Error::msg("Failed to parse vector item"))?);
        }
        Ok(res)
    }
}

impl<T: ParseFormValue, const L: usize> ParseFormValue for [T; L] {
    fn parse(value: Value) -> Result<Self, Error> {
        let res = Vec::<T>::parse(value)?;
        TryInto::<[T; L]>::try_into(res)
            .map_err(|_| report!(Error::msg("Failed to parse value as array")))
    }
}

impl ParseFormValue for Config {
    fn parse(value: Value) -> Result<Self, Error> {
        let mut file = value.path.clone();
        let mut val = value.val;
        if let config::ValueKind::String(ref path) = val.kind {
            if let Some(path) = path.strip_prefix("file!:") {
                file = calc_path(&file, path).change_context(Error::msg(format!(
                    "Failed to calculate path to include config: file: {:?} to file: {:?}",
                    file, value.path
                )))?;
                val = config::Config::builder()
                    .add_source(config::File::with_name(&file))
                    .build()
                    .change_context(Error::msg(format!("Failed to include config file: {file:?}")))?
                    .cache;
            }
        }
        let cfg = val.into_table().change_context(Error::msg("Failed to parse value as table"))?;
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
