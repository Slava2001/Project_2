//! Implementations of widget.

mod base;
mod button;
mod flag;
mod graph;
mod label;
mod panel;
mod slider;
mod textbox;

use std::ops::{Deref, DerefMut};

pub use base::Base;
pub use button::Button;
pub use flag::Flag;
pub use graph::Graph;
pub use label::Label;
pub use panel::Panel;
pub use slider::Slider;
pub use textbox::Textbox;

use crate::manager::widget::WRef;
use builder::{BuildFromCfg, Builder as BaseBuilder};

/// Widget builder.
pub struct Builder(BaseBuilder<WRef>);

impl Default for Builder {
    /// Default builder, that can build all default widgets.
    fn default() -> Self {
        let mut builder = BaseBuilder::<WRef>::new();
        builder.reg_builder("base", Base::build);
        builder.reg_builder("button", Button::build);
        builder.reg_builder("flag", Flag::build);
        builder.reg_builder("label", Label::build);
        builder.reg_builder("panel", Panel::build);
        builder.reg_builder("graph", Graph::build);
        builder.reg_builder("slider", Slider::build);
        builder.reg_builder("textbox", Textbox::build);
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
