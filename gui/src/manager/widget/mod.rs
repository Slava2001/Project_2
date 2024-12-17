use std::{cell::RefCell, rc::Weak};

use super::State;
use crate::renderer::{rect::Rect, vec2::Vec2f, Drawble};
use event::Event;
use widget_ref::WidgetRef;

pub mod base_widget;
pub mod event;
pub mod widget_ref;

pub trait Widget: Drawble {
    fn handle_event(&mut self, self_ref: WidgetRef, event: Event, state: &mut State);
    fn get_hovered(&self, pos: Vec2f) -> Option<WidgetRef>;

    fn add_widget(&mut self, self_ref: WidgetRef, widget: &mut dyn Widget, widget_ref: WidgetRef);
    fn erase_widget(&mut self, widget_ref: &WidgetRef);
    fn detach(&mut self, self_ref: &WidgetRef);
    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Widget>>>);
    fn get_parent(&mut self) -> Option<Weak<RefCell<dyn Widget>>>;

    fn set_positon(&mut self, pos: Vec2f);
    fn get_positon(&self) -> Vec2f;
    fn set_global_positon(&mut self, pos: Vec2f);
    fn get_global_positon(&self) -> Vec2f;

    fn check_bounds(&self, pos: Vec2f) -> bool;
    fn get_rect(&self) -> &Rect<f64>;
}
