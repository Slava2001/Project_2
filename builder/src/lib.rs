//! Object builder.

mod config;

pub use config::Config;
use error_stack::{bail, Result, ResultExt};
use std::collections::HashMap;

use resources::Manger;

/// Builder error
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message
    pub fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

/// It allow build object with [`Config`]
pub trait BuildFromCfg<T> {
    /// Build object with given config
    ///
    /// # Errors
    /// Return error if config is not valid
    fn build(cfg: Config, resources: &mut dyn Manger) -> Result<T, Error>;
}

/// Object builder function [`BuildFromCfg::build`]).
type BuildFunc<T> = fn(Config, &mut dyn Manger) -> Result<T, Error>;

/// Object builders map. key - object type, value - builder function ([`BuildFunc`])
type BuildFuncsMap<T> = HashMap<String, BuildFunc<T>>;

/// Object builder
pub struct Builder<T> {
    /// Object builders map
    builders_map: BuildFuncsMap<T>,
}

impl<T> Builder<T> {
    /// Create empty object builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Build object with config. Field "type" used for find builder function
    ///
    /// # Errors
    /// Return error if failed to find builder func (request unknown object type)
    /// or if failed to build object (invalid config)
    pub fn build(&self, mut cfg: Config, res: &mut dyn Manger) -> Result<T, Error> {
        let object_type: String =
            cfg.take("type").change_context(Error::msg("Failed to get object type"))?;

        let Some(ref builder) = self.builders_map.get(&object_type) else {
            bail!(Error::msg(format!("Failed to find builder for \"{object_type}\"")));
        };
        builder(cfg, res)
    }

    /// Register object builder function ([`BuildFromCfg::build`])
    pub fn reg_builder<K: Into<String>>(&mut self, object_type: K, builder: BuildFunc<T>) {
        self.builders_map.insert(object_type.into(), builder);
    }
}

impl<T> Default for Builder<T> {
    fn default() -> Self {
        Self { builders_map: HashMap::default() }
    }
}
