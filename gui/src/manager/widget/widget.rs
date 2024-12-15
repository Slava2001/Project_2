use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

use crate::{manager::manager::State, renderer::{vec2::Vec2f, Drawble, Rect}};

use super::event::Event;

#[derive(Clone)]
pub struct WidgetRef(Rc<RefCell<dyn Widget>>);
impl WidgetRef {
    pub fn new<T: 'static + Widget>(widget: T) -> Self {
        Self(Rc::new(RefCell::new(widget)))
    }
}

impl Deref for WidgetRef {
    type Target = Rc<RefCell<dyn Widget>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WidgetRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for WidgetRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl From<Rc<RefCell<dyn Widget>>> for WidgetRef{
    fn from(value: Rc<RefCell<dyn Widget>>) -> Self {
        Self(value)
    }
}

pub trait Widget: Drawble {
    fn handle_event(&mut self, self_rc: WidgetRef, event: Event, state: &mut State);
    fn get_hovered(&self, pos: Vec2f) -> Option<WidgetRef>;
    fn check_bounds(&self, pos: Vec2f) -> bool;
    fn add_widget(&mut self, self_rc: WidgetRef, widget: WidgetRef);
    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Widget>>>);
    fn get_parent(&mut self) -> Option<Weak<RefCell<dyn Widget>>>;
    fn detach(&mut self, self_rc: &WidgetRef);
    fn erase_widget(&mut self, widget: &WidgetRef);
    fn set_positon(&mut self, pos: Vec2f);
    fn get_positon(&self) -> Vec2f;
    fn set_global_positon(&mut self, pos: Vec2f);
    fn get_global_positon(&self) -> Vec2f;
    fn get_rect(&self) -> &Rect<f64>;
}
