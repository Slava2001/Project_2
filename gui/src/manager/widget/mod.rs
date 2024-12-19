use std::{cell::RefCell, rc::Weak};

use super::State;
use crate::renderer::{rect::Rect, vec2::Vec2f, Drawble};
pub use base::Base;
pub use event::Event;
pub use wref::WRef;

mod base;
mod event;
mod wref;

/// Widget interface
/// Implement it if you want to create a widget
/// You can use the composition with the [`Base`] widget to implement a new one
/// 
/// [`base`]: base::Base
pub trait Widget: Drawble {
    /// Handle input event
    /// - `self_ref`: ref on self. Do not try borow, use `self`
    /// - `event`: event to handle
    /// - `state`: current state
    fn handle_event(&mut self, self_ref: WRef, event: Event, state: &mut State);

    /// Find the widget under the cursor.
    /// - `pos': cursor position in local coordinates
    fn get_hovered(&self, pos: Vec2f) -> Option<WRef>;

    /// Add child widget
    /// - `self_ref`: ref on self. Do not try borow, use `self`
    /// - `widget`: widget to add
    /// - `widget_ref`: ref on widget to add. Do not try borow, use `widget`
    fn add_widget(&mut self, self_ref: WRef, widget: &mut dyn Widget, widget_ref: WRef);

    /// Remove child widget
    /// - `widget_ref`: ref on widget to remove. Do not try borow
    fn erase_widget(&mut self, widget_ref: &WRef);

    /// Detach widget
    /// -`self_ref`:  ref on self. Do not try borow, use `self`
    fn detach(&mut self, self_ref: &WRef);

    /// Set parent
    /// - `parent`: parent to set (May be `None` for remove parent)
    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Widget>>>);

    /// Get parent
    fn get_parent(&mut self) -> Option<Weak<RefCell<dyn Widget>>>;

    /// Set widgel local (relative to parent) position
    /// - `pos`: new widget position
    fn set_positon(&mut self, pos: Vec2f);

    /// Get widgel local (relative to parent) position
    fn get_positon(&self) -> Vec2f;

    /// Set widgel global (relative to root widget) position
    /// - `pos`: new widget position
    fn set_global_positon(&mut self, pos: Vec2f);

    /// Get widgel global (relative to root widget) position
    fn get_global_positon(&self) -> Vec2f;

    /// Сheck that the point is within the widget boundaries
    /// - `pos`: position of the point in local (relative to parent) coordinates
    fn check_bounds(&self, pos: Vec2f) -> bool;

    /// Get widget boundaries in local (relative to parent) coordinates
    fn get_rect(&self) -> &Rect<f64>;
}
