//! GUI manager.
//!
//! This module manages the life cycle of GUI elements.

use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::widget::Builder;
use builder::config::Config;
use error_stack::{Result, ResultExt};
use renderer::{Drawable, Renderer};
use resources::Manager as ResMngr;
use scene::event::{self, Event as SceneEvent};
use utils::vec2::Vec2f;
use widget::{event::Event, WRef};

mod state;
pub mod widget;

pub use state::State;

/// Manager error.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message.
    fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

/// GUI manager.
pub struct Manager {
    /// Reference on root widget.
    root: WRef,
    /// Manager state.
    state: State,
}

impl Manager {
    /// Create new GUI manager.
    ///
    /// # Errors
    /// Return error if config is not valid.
    pub fn new(builder: &Builder, res: &mut dyn ResMngr, cfg: Config) -> Result<Self, Error> {
        let root = Self::make_gui_tree(builder, cfg, res)?;
        Ok(Self { state: State::new(root.clone()), root })
    }

    /// Recursive make gui tree with given config.
    fn make_gui_tree(
        builder: &Builder,
        mut cfg: Config,
        res_mngr: &mut dyn ResMngr,
    ) -> Result<WRef, Error> {
        if let Some(res_arr) = cfg
            .take_opt::<Vec<Config>>("recourses")
            .change_context(Error::msg("Failed to init recourses"))?
        {
            for mut res in res_arr {
                let name = res
                    .take::<String>("name")
                    .change_context(Error::msg("Failed to init resource"))?;
                let kind = res
                    .take::<String>("type")
                    .change_context(Error::msg("Failed to init resource"))?;
                let path = res
                    .take::<PathBuf>("path")
                    .change_context(Error::msg("Failed to init resource"))?;
                res_mngr.load(&kind, &name, &path).change_context(Error::msg(format!(
                    "Failed to load resource: name: {:?}, type: {:?}, path: {:?}",
                    name,
                    kind,
                    path.display()
                )))?;
            }
        }

        let childs_cfg = cfg
            .take_opt::<Vec<Config>>("childs")
            .change_context(Error::msg("Failed to get childs config"))?;
        let widget =
            builder.build(cfg, res_mngr).change_context(Error::msg("Failed to build widget"))?;

        if let Some(childs_cfg) = childs_cfg {
            for child_cfg in childs_cfg {
                let child = Self::make_gui_tree(builder, child_cfg, res_mngr)?;
                widget.borrow_mut().add_widget(
                    widget.clone(),
                    &mut *child.borrow_mut(),
                    child.clone(),
                );
            }
        }

        Ok(widget)
    }

    /// Handle event.
    ///
    /// # Errors
    /// Return error if widget failed to handle event.
    pub fn handle_event(&mut self, event: SceneEvent) -> Result<(), Error> {
        if let SceneEvent::MouseMove(pos) = event {
            self.state.mouse = pos;
        }

        if let event::Event::Resize(size) = event {
            self.root.borrow_mut().set_size(size);
            return Ok(());
        }

        let Ok(event) = TryInto::<widget::event::Event>::try_into(event) else {
            return Ok(());
        };

        if let Some(w) = self.state.get_caught() {
            let is_visible = w.borrow().is_visible();
            if is_visible {
                w.borrow_mut()
                    .handle_event(w.clone(), event.clone(), &mut self.state)
                    .change_context(Error::msg("Caught widget failed when handle event"))?;
            } else {
                self.state.uncatch(&mut *w.borrow_mut(), w.clone()).change_context(Error::msg(
                    "Caught widget failed when handle uncaught event",
                ))?;
            }
        }

        if let Some(w) = self.state.get_focused() {
            let is_visible = w.borrow().is_visible();
            if is_visible {
                w.borrow_mut()
                    .handle_event(w.clone(), event.clone(), &mut self.state)
                    .change_context(Error::msg("Focused widget failed when handle event"))?;
            } else if !self.state.is_caught(w.clone()) {
                self.state
                    .unfocus(&mut *w.borrow_mut(), w.clone())
                    .change_context(Error::msg("Caught widget failed when handle unfocus event"))?;
            }
        }

        let is_visible = self.state.hovered.borrow().is_visible();
        if !self.state.is_caught(self.state.hovered.clone())
            && !self.state.is_focused(self.state.hovered.clone())
            && is_visible
        {
            self.state
                .hovered
                .clone()
                .borrow_mut()
                .handle_event(self.state.hovered.clone(), event, &mut self.state)
                .change_context(Error::msg("Hovered widget failed when handle event"))?;
        }
        self.update_hovered(self.state.mouse)?;
        Ok(())
    }

    /// Update hovered widget.
    fn update_hovered(&mut self, pos: Vec2f) -> Result<(), Error> {
        let hovered = self.root.borrow().get_hovered(pos).unwrap_or_else(|| self.root.clone());
        if self.state.hovered != hovered {
            hovered
                .borrow_mut()
                .handle_event(hovered.clone(), Event::MouseEnter, &mut self.state)
                .change_context(Error::msg("Widget failed to handle mouse enter event"))?;
            self.state
                .hovered
                .clone()
                .borrow_mut()
                .handle_event(self.state.hovered.clone(), Event::MouseLeave, &mut self.state)
                .change_context(Error::msg("Widget failed to handle mouse leave event"))?;
            self.state.hovered = hovered;
        }
        Ok(())
    }

    /// Find widget by specified identification.
    #[must_use]
    pub fn get_by_id(&self, id: &str) -> Option<WRef> {
        if self.root.borrow().get_id() == id {
            return Some(self.root.clone());
        }
        self.root.borrow().find(id)
    }

    /// Find widget by specified identification and downcast it.
    ///
    /// # Errors
    /// Return error if widget not found or can not be casted to specified type.
    pub fn get_by_id_cast<T: 'static>(&self, id: &str) -> Result<Rc<RefCell<T>>, Error> {
        Ok(self
            .get_by_id(id)
            .ok_or_else(|| Error::msg(format!("Failed to find requested widget: id: \"{id}\"")))?
            .try_cast::<T>()
            .ok_or_else(|| {
                Error::msg(format!(
                    "Widget \"{}\" has unexpected type. Expected: {}",
                    id,
                    std::any::type_name::<T>()
                ))
            })?)
    }
}

impl Drawable for Manager {
    /// Draw all visible widgets.
    fn draw(&self, renderer: &mut dyn Renderer) {
        if self.root.borrow().is_visible() {
            self.root.borrow().draw(renderer);
        }
        if let Some(ref c) = self.state.get_caught() {
            c.borrow().draw(renderer);
        }
    }
}
