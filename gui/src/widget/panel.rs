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
        State,
    },
    renderer::{rect::Rect, vec2::Vec2f, Drawable, Renderer},
    resources::TextureId,
};

/// Panel widget
pub struct Panel {
    /// Base widget
    base: Base,
    /// Background texture
    texture: TextureId,
    /// Background texture rectangle
    texture_rect: Rect<f64>,
    /// Offset, used when widget cached
    offset: Vec2f,
}

impl Widget for Panel {
    fn handle_event(
        &mut self,
        self_rc: WRef,
        event: Event,
        state: &mut State,
    ) -> Result<(), Error> {
        if let Event::InputEvent(i) = event {
            match i {
                InputEvent::MousePress(MouseButton::Left) => {
                    if state.caught.is_none() {
                        self.get_parent()
                            .map(|p| p.upgrade().map(|p| p.borrow_mut().erase_widget(&self_rc)));
                        self.offset = self.get_global_position() - state.mouse;
                        self.set_position(state.mouse + self.offset);
                        state.caught = Some(self_rc);
                    }
                }
                InputEvent::MouseRelease(MouseButton::Left) => {
                    if let Some(caught) = state.caught.clone() {
                        if caught == self_rc {
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
                InputEvent::MouseMove( .. ) => {
                    if state.caught == Some(self_rc) {
                        self.set_position(state.mouse + self.offset);
                    }
                }
                _ => {}
            }
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

    fn find(&self, id: &str) -> Option<WRef> {
        self.base.find(id)
    }

    fn get_id(&self) -> String {
        self.base.get_id()
    }
}

impl Drawable for Panel {
    fn draw(&self, renderer: &mut dyn Renderer) {
        renderer.draw_img(self.base.get_rect(), self.texture, &self.texture_rect);
        self.base.draw(renderer);
    }
}

impl BuildFromCfg for Panel {
    fn build(
        mut cfg: config::Map<String, config::Value>,
        res: &mut dyn crate::resources::Manger,
    ) -> Result<WRef, builder::Error> {
        let bg_texture = cfg
            .remove("background")
            .ok_or_else(|| builder::Error::msg("Failed to init panel, no filed \"background\""))?;
        let bg_name = bg_texture.into_string().change_context(builder::Error::msg(
            "Failed to init panel, filed \"background\" is not a string",
        ))?;
        let texture = res.get_texture(&bg_name).change_context(builder::Error::msg(format!(
            "Failed to init panel, texture: \"{bg_name}\" not found"
        )))?;
        let texture_rect = cfg
            .remove("background_rect")
            .ok_or_else(|| {
                builder::Error::msg("Failed to init panel, no filed \"background_rect\"")
            })?
            .try_deserialize::<[f64; 4]>()
            .change_context(builder::Error::msg(
                "Failed deserialize filed \"background_rect\" as rectangle",
            ))?
            .into();
        Ok(WRef::new(Self {
            base: Base::new(cfg)?,
            texture,
            texture_rect,
            offset: Vec2f::new(0.0, 0.0),
        }))
    }
}
