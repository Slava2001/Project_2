//! GUI manager
//!
//! This module manages the life cycle of GUI elements

use error_stack::{Result, ResultExt};
use scene::event::Event as SceneEvent;
use renderer::{vec2::Vec2f, Drawable, Renderer};
use resources::Manger;
use widget::{builder::Builder, event::Event, WRef};

pub mod widget;

/// Manager error
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message
    fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

/// Manager state
pub struct State {
    /// Coughed widget
    pub caught: Option<WRef>,
    /// Cursor position
    pub mouse: Vec2f,
}

/// GUI manager
pub struct Manager {
    /// Reference on widget under cursor
    hovered: WRef,
    /// Reference on root widget
    root: WRef,
    /// manager state
    state: State,
}

impl Manager {
    /// Create new GUI manager
    ///
    /// # Errors
    /// Return error if config is not valid
    pub fn new(
        builder: &Builder,
        res: &mut dyn Manger,
        cfg: config::Map<String, config::Value>,
    ) -> Result<Self, Error> {
        let root = Self::make_gui_tree(builder, cfg, res)?;
        Ok(Self {
            hovered: root.clone(),
            root,
            state: State { mouse: Vec2f::new(0.0, 0.0), caught: None },
        })
    }

    /// Recursive make gui tree with given config
    fn make_gui_tree(
        builder: &Builder,
        mut cfg: config::Map<String, config::Value>,
        res_mngr: &mut dyn Manger,
    ) -> Result<WRef, Error> {
        if let Some(res_cfg) = cfg.remove("recourses") {
            let res_arr = res_cfg
                .into_array()
                .change_context(Error::msg("\"recourses\" field is not array"))?;
            for res in res_arr {
                let mut table = res
                    .into_table()
                    .change_context(Error::msg("\"recourses\" array item is not a table"))?;

                let mut gets = |str| {
                    table
                        .remove(str)
                        .ok_or_else(|| {
                            Error::msg(format!("Some resource has not field \"{str}\""))
                        })?
                        .into_string()
                        .change_context(Error::msg(format!(
                            "Some resource has invalid field \"{str}\""
                        )))
                };
                let name = gets("name")?;
                let kind = gets("type")?;
                let path = gets("path")?;
                res_mngr.load(&kind, &name, &path).change_context(Error::msg(format!(
                    "Failed to load resource: name: \"{name}\", type: \"{kind}\", path: \"{path}\""
                )))?;
            }
        }

        let childs_cfg = cfg.remove("childs");
        let widget =
            builder.build(cfg, res_mngr).change_context(Error::msg("Failed to build widget"))?;

        if let Some(childs_cfg) = childs_cfg {
            let childs_cfg = childs_cfg.into_array().unwrap();
            for child_cfg in childs_cfg {
                let child =
                    Self::make_gui_tree(builder, child_cfg.into_table().unwrap(), res_mngr)?;
                widget.borrow_mut().add_widget(
                    widget.clone(),
                    &mut *child.borrow_mut(),
                    child.clone(),
                );
            }
        }

        Ok(widget)
    }

    /// Handle event
    ///
    /// # Errors
    /// Return error if widget failed to handle event
    pub fn handle_event(&mut self, event: SceneEvent) -> Result<(), Error> {
        if let SceneEvent::MouseMove(x, y) = event {
            self.state.mouse = (x, y).into();
        }

        let Ok(event) = TryInto::<widget::event::Event>::try_into(event) else {
            return Ok(());
        };

        if let Some(c) = self.state.caught.clone() {
            c.borrow_mut()
                .handle_event(c.clone(), event, &mut self.state)
                .change_context(Error::msg("Couched widget failed when handle event"))?;
        }
        self.hovered
            .borrow_mut()
            .handle_event(self.hovered.clone(), event, &mut self.state)
            .change_context(Error::msg("Hovered widget failed when handle event"))?;
        self.update_hovered(self.state.mouse)?;
        Ok(())
    }

    /// Update hovered widget
    fn update_hovered(&mut self, pos: Vec2f) -> Result<(), Error> {
        let hovered = self.root.borrow().get_hovered(pos).unwrap_or_else(|| self.root.clone());
        if self.hovered != hovered {
            hovered
                .borrow_mut()
                .handle_event(hovered.clone(), Event::MouseEnter, &mut self.state)
                .change_context(Error::msg("Widget failed to handle mouse enter event"))?;
            self.hovered
                .borrow_mut()
                .handle_event(self.hovered.clone(), Event::MouseLeave, &mut self.state)
                .change_context(Error::msg("Widget failed to handle mouse leave event"))?;
            self.hovered = hovered;
        }
        Ok(())
    }

    /// Find widget by specified identification
    #[must_use]
    pub fn get_by_id(&self, id: &str) -> Option<WRef> {
        self.root.borrow().find(id)
    }
}

impl Drawable for Manager {
    /// Draw all visible widgets
    fn draw(&self, renderer: &mut dyn Renderer) {
        self.root.borrow().draw(renderer);
        if let Some(ref c) = self.state.caught {
            c.borrow().draw(renderer);
        }
    }
}
