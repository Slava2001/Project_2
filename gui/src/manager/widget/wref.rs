//! Widget reference. It is wrapper on `Rc<RefCell<dyn Widget>>`

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::Widget;

/// Widget reference
#[derive(Clone)]
pub struct WRef(Rc<RefCell<dyn Widget>>);
impl WRef {
    /// Create new widget reference
    #[must_use]
    pub fn new<T: 'static + Widget>(widget: T) -> Self {
        Self(Rc::new(RefCell::new(widget)))
    }

    /// Try cast widget reference to concrete widget
    pub fn try_cast<T: Any>(self) -> Option<Rc<RefCell<T>>> {
        if TypeId::of::<T>() == (*self.0.borrow()).type_id() {
            unsafe {
                let r = (&*(&self.0 as *const Rc<RefCell<dyn Widget>> as *const Rc<RefCell<T>>))
                    .clone();
                Some(r)
            }
        } else {
            None
        }
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

impl<T: Widget> From<Rc<RefCell<T>>> for WRef {
    fn from(value: Rc<RefCell<T>>) -> Self {
        Self(value as Rc<RefCell<dyn Widget>>)
    }
}
