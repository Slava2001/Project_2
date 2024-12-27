//! Widget builder. GUI manager use it for create widgets by config.

use config::{Map, Value};
use error_stack::{bail, Result, ResultExt};
use std::collections::HashMap;

use super::WRef;
use crate::renderer::ResourceManger;

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

/// It allow build widget with config
pub trait BuildFromCfg {
    /// Build widget with given config
    ///
    /// # Errors
    /// Return error if config is not valid
    fn build(cfg: Map<String, Value>, resources: &mut dyn ResourceManger) -> Result<WRef, Error>;
}

/// Widget builder function [`BuildFromCfg::build`]).
type BuildFunc = fn(Map<String, Value>, &mut dyn ResourceManger) -> Result<WRef, Error>;

/// Widget builders map. key - widget type, value - builder function ([`BuildFunc`])
type BuildFuncsMap = HashMap<String, BuildFunc>;

/// Widget builder
pub struct Builder {
    /// Widget builders map
    builders_map: BuildFuncsMap,
}

impl Builder {
    /// Create empty widget builder
    #[must_use]
    pub fn new() -> Self {
        Self { builders_map: HashMap::new() }
    }

    /// Build widget with config. Field "type" used for find builder function
    ///
    /// # Errors
    /// Return error if failed to find builder func (request unknown widget type)
    /// or if failed to build widget (invalid config)
    pub fn build(
        &self,
        mut cfg: Map<String, Value>,
        res: &mut dyn ResourceManger,
    ) -> Result<WRef, Error> {
        let Some(widget_type) = cfg.remove("type") else {
            bail!(Error::msg("Config dos not contain widget type"));
        };
        let widget_type = widget_type
            .into_string()
            .change_context(Error::msg("Failed to parse as widget type"))?;
        let Some(ref builder) = self.builders_map.get(&widget_type) else {
            bail!(Error::msg(format!("Failed to find builder for \"{widget_type}\"")));
        };
        builder(cfg, res)
    }

    /// Register widget builder function ([`BuildFromCfg::build`])
    pub fn reg_widget_builder(
        &mut self,
        widget_type: String,
        builder: fn(Map<String, Value>, &mut dyn ResourceManger) -> Result<WRef, Error>,
    ) {
        self.builders_map.insert(widget_type, builder);
    }
}

impl Default for Builder {
    /// Default builder, that can build all default widgets
    fn default() -> Self {
        let mut builder = Self::new();
        builder.reg_widget_builder("base".into(), super::Base::build);
        builder.reg_widget_builder("panel".into(), crate::widget::Panel::build);
        builder
    }
}
