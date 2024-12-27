//! Panel
//!
//! Simple widget. It used for groups other widgets

use error_stack::{Result, ResultExt};
use std::{cell::RefCell, rc::Weak};

use crate::{
    manager::{
        input_event::{InputEvent, MouseButton},
        widget::{
            builder::{self, BuildFromCfg},
            Base, Error, Event, WRef, Widget,
        },
        Caught, State,
    },
    renderer::{
        color::{self, Color},
        rect::Rect,
        vec2::Vec2f,
        Drawable, Renderer,
    },
};

/// Panel widget
pub struct Panel {
    /// Base widget
    base: Base,
    /// Background color
    color: Color,
}

impl Widget for Panel {
    fn handle_event(
        &mut self,
        self_rc: WRef,
        event: Event,
        state: &mut State,
    ) -> Result<(), Error> {
        match event {
            Event::MouseEnter => self.color = color::GREEN,
            Event::MouseLeave => self.color = color::BLACK,
            Event::InputEvent(i) => match i {
                InputEvent::MousePress(MouseButton::Left) => {
                    if state.caught.is_none() {
                        self.get_parent()
                            .map(|p| p.upgrade().map(|p| p.borrow_mut().erase_widget(&self_rc)));
                        let offset = self.get_global_position() - state.mouse;
                        state.caught = Some(Caught { widget: self_rc, offset });
                    }
                }
                InputEvent::MouseRelease(MouseButton::Left) => {
                    if let Some(ref caught) = state.caught {
                        if caught.widget == self_rc {
                            self.get_parent().map(|p| {
                                p.upgrade().map(|p| {
                                    p.clone().borrow_mut().add_widget(p.into(), self, self_rc);
                                })
                            });
                            state.caught = None;
                            self.set_global_position(self.get_position());
                        }
                    }
                }
                _ => {}
            },
        }
        Ok(())
    }

    fn get_hovered(&self, pos: Vec2f) -> Option<WRef> {
        self.base.get_hovered(pos)
    }

    fn check_bounds(&self, pos: Vec2f) -> bool {
        self.base.check_bounds(pos)
    }

    fn add_widget(&mut self, self_ref: WRef, widget: &mut dyn Widget, widget_ref: WRef) {
        self.base.add_widget(self_ref, widget, widget_ref);
    }

    fn set_parent(&mut self, parent: Option<Weak<RefCell<dyn Widget>>>) {
        self.base.set_parent(parent);
    }

    fn get_parent(&mut self) -> Option<Weak<RefCell<dyn Widget>>> {
        self.base.get_parent()
    }

    fn detach(&mut self, self_rc: &WRef) {
        self.base.detach(self_rc);
    }

    fn erase_widget(&mut self, widget: &WRef) {
        self.base.erase_widget(widget);
    }

    fn set_position(&mut self, pos: Vec2f) {
        self.base.set_position(pos);
    }

    fn get_position(&self) -> Vec2f {
        self.base.get_position()
    }

    fn set_global_position(&mut self, pos: Vec2f) {
        self.base.set_global_position(pos);
    }

    fn get_global_position(&self) -> Vec2f {
        self.base.get_global_position()
    }

    fn get_rect(&self) -> &Rect<f64> {
        self.base.get_rect()
    }
}

impl Drawable for Panel {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.draw_rect(self.base.get_rect(), &self.color);
        self.base.draw(renderer);
    }
}

impl BuildFromCfg for Panel {
    fn build(
        mut cfg: config::Map<String, config::Value>,
        _res: &mut dyn crate::renderer::ResourceManger,
    ) -> Result<WRef, builder::Error> {
        let color = if let Some(rect) = cfg.remove("background_color") {
            rect.into_string()
                .change_context(builder::Error::msg("Field \"background_color\" is not a string"))?
                .parse::<Color>()
                .change_context(builder::Error::msg(
                    "Failed to parse \"background_color\" field as color",
                ))?
        } else {
            color::BLACK
        };
        Ok(WRef::new(Self { base: Base::new(cfg)?, color }))
    }
}
