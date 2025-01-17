use std::ops::{Deref, DerefMut};

use builder::Builder as BaseBuilder;

use crate::Scene;

pub struct Builder(BaseBuilder<Box<dyn Scene>>);

impl Builder {
    pub fn new() -> Self {
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
