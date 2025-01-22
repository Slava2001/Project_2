//! Implementations of widget

mod base;
mod button;
mod flag;
mod label;
mod panel;

use std::ops::{Deref, DerefMut};

pub use base::Base;
pub use button::Button;
pub use flag::Flag;
pub use label::Label;
pub use panel::Panel;

use crate::manager::widget::WRef;
use builder::{BuildFromCfg, Builder as BaseBuilder};

/// Widget builder.
pub struct Builder(BaseBuilder<WRef>);

impl Default for Builder {
    /// Default builder, that can build all default widgets
    fn default() -> Self {
        let mut builder = BaseBuilder::<WRef>::new();
        builder.reg_builder("base", Base::build);
        builder.reg_builder("button", Button::build);
        builder.reg_builder("flag", Flag::build);
        builder.reg_builder("label", Label::build);
        builder.reg_builder("panel", Panel::build);
        Self(builder)
    }
}

impl Deref for Builder {
    type Target = BaseBuilder<WRef>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Builder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
