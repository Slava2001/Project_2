use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::Widget;

#[derive(Clone)]
pub struct WRef(Rc<RefCell<dyn Widget>>);
impl WRef {
    pub fn new<T: 'static + Widget>(widget: T) -> Self {
        Self(Rc::new(RefCell::new(widget)))
    }
}

impl Deref for WRef {
    type Target = Rc<RefCell<dyn Widget>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for WRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl From<Rc<RefCell<dyn Widget>>> for WRef {
    fn from(value: Rc<RefCell<dyn Widget>>) -> Self {
        Self(value)
    }
}
