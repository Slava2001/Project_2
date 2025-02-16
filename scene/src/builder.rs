//! Scene builder.

use crate::Scene;
use builder::Builder as BaseBuilder;
use std::ops::{Deref, DerefMut};

/// Scene builder.
pub struct Builder(BaseBuilder<Box<dyn Scene>>);

impl Builder {
    /// Creates empty scene builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self(BaseBuilder::new())
    }
}

impl Deref for Builder {
    type Target = BaseBuilder<Box<dyn Scene>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Builder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
