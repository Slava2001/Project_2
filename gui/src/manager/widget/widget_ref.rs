use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::Widget;

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

impl From<Rc<RefCell<dyn Widget>>> for WidgetRef {
    fn from(value: Rc<RefCell<dyn Widget>>) -> Self {
        Self(value)
    }
}
