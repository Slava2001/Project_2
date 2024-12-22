//! Gui manager. This module manages the life cycle of GUI elements

use error_stack::{Result, ResultExt};
use input_event::InputEvent;

use super::renderer::{vec2::Vec2f, Drawable, Renderer};
use widget::{builder::Builder, Event, WRef};

pub mod input_event;
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

/// The coughed widget, that follows the cursor
pub struct Caught {
    /// Reference on coughed widget
    pub widget: WRef,
    /// Coughed widget offset. Widget position calculated as cursor position + `offset`
    pub offset: Vec2f,
}

/// Manager state
pub struct State {
    /// Coughed widget
    pub caught: Option<Caught>,
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
    pub fn new(builder: &Builder, cfg: config::Map<String, config::Value>) -> Result<Self, Error> {
        let root = Self::make_gui_tree(builder, cfg)?;
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
    ) -> Result<WRef, Error> {
        let childs_cfg = cfg.remove("childs");
        let widget = builder.build(cfg).change_context(Error::msg("Failed to build widget"))?;

        if let Some(childs_cfg) = childs_cfg {
            let childs_cfg = childs_cfg.into_array().unwrap();
            for child_cfg in childs_cfg {
                let child = Self::make_gui_tree(builder, child_cfg.into_table().unwrap())?;
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
    pub fn handle_event(&mut self, event: InputEvent) -> Result<(), Error> {
        if let InputEvent::MouseMove(x, y) = event {
            self.state.mouse = (x, y).into();
        }

        if let Some(ref c) = self.state.caught {
            let w = c.widget.clone();
            w.borrow_mut()
                .handle_event(w.clone(), Event::InputEvent(event), &mut self.state)
                .change_context(Error::msg("Couched widget failed when handle event"))?;
        }
        self.hovered
            .borrow_mut()
            .handle_event(self.hovered.clone(), Event::InputEvent(event), &mut self.state)
            .change_context(Error::msg("Hovered widget failed when handle event"))?;
        if let Some(ref c) = self.state.caught {
            c.widget.borrow_mut().set_position(self.state.mouse + c.offset);
        }
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
}

impl Drawable for Manager {
    /// Draw all visible widgets
    fn draw(&self, renderer: &mut dyn Renderer) {
        self.root.borrow().draw(renderer);
        if let Some(ref c) = self.state.caught {
            c.widget.borrow().draw(renderer);
        }
    }
}
