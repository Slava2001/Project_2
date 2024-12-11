use std::{cell::RefCell, rc::Rc};

use crate::renderer::Drawble;

use super::event::Event;

pub trait Widget: Drawble {
    fn handle_event(&mut self, event: Event);
    fn get_hovered(&self, x: f64, y: f64) -> Option<Rc<RefCell<dyn Widget>>>;
    fn check_bounds(&self, x: f64, y: f64) -> bool;
    fn add_widget(&mut self, widget: Rc<RefCell<dyn Widget>>);
}
