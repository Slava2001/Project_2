//! Widget interface.

use error_stack::Result;
use std::{any::Any, cell::RefCell, rc::Weak};

pub mod event;
mod wref;

use super::State;
use event::Event;
use renderer::{rect::Rect, vec2::Vec2f, Drawable};
pub use wref::WRef;

/// Widget error
#[derive(Debug, thiserror::Error)]
#[error("Widget error")]
pub struct Error;

/// Widget interface.
///
/// Implement it if you want to create a widget.
/// You can use the composition with the [`Base`] widget to implement a new one.
///
/// [`base`]: base::Base
pub trait Widget: Drawable + Any {
    /// Handle input event.
    /// - `self_ref`: ref on self. Do not try borrow, use `self`.
    /// - `event`: event to handle.
    /// - `state`: current state.
    ///
    /// # Errors
    /// Return error if widget failed to handle event.
    fn handle_event(
        &mut self,
        self_ref: WRef,
        event: Event,
        state: &mut State,
    ) -> Result<(), Error>;

    /// Find the widget under the cursor.
    /// - `pos`: cursor position in local coordinates.
    fn get_hovered(&self, pos: Vec2f) -> Option<WRef>;

    /// Add child widget.
    /// - `self_ref`: ref on self. Do not try borrow, use `self`.
    /// - `widget`: widget to add.
    /// - `widget_ref`: ref on widget to add. Do not try borrow, use `widget`.
    fn add_widget(&mut self, self_ref: WRef, widget: &mut dyn Widget, widget_ref: WRef);

    /// Remove child widget.
    /// - `widget_ref`: ref on widget to remove. Do not try borrow.
    fn erase_widget(&mut self, widget_ref: &WRef);

    /// Detach widget.
    /// -`self_ref`:  ref on self. Do not try borrow, use `self`.
    fn detach(&mut self, self_ref: &WRef);

    /// Set parent.
    /// - `parent`: parent to set (May be `None` for remove parent).
    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Widget>>>);

    /// Get parent.
    fn get_parent(&mut self) -> Option<Weak<RefCell<dyn Widget>>>;

    /// Set widget local (relative to parent) position.
    /// - `pos`: new widget position.
    fn set_position(&mut self, pos: Vec2f);

    /// Get widget local (relative to parent) position.
    fn get_position(&self) -> Vec2f;

    /// Set widget global (relative to root widget) position.
    /// - `pos`: new widget position.
    fn set_global_position(&mut self, pos: Vec2f);

    /// Get widget global (relative to root widget) position.
    fn get_global_position(&self) -> Vec2f;

    /// Check that the point is within the widget boundaries.
    /// - `pos`: position of the point in local (relative to parent) coordinates.
    fn check_bounds(&self, pos: Vec2f) -> bool;

    /// Get widget boundaries in local (relative to parent) coordinates.
    fn get_rect(&self) -> &Rect<f64>;

    /// Get widget identifier.
    fn get_id(&self) -> String;

    /// Find widget by with specified id.
    /// Returns the first widget witt specified id.
    /// - `id`: widget id.
    fn find(&self, id: &str) -> Option<WRef>;

    /// Set widget is visible flag.
    fn set_visible_flag(&mut self, is_visible: bool);

    /// Check if widget is visible.
    fn is_visible(&self) -> bool;
}
