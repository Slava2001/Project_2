//! Gui manager. This module manages the life cycle of GUI elements
use super::{
    renderer::{vec2::Vec2f, Drawable, Renderer},
    widget::Panel,
};
use input_event::InputEvent;
use widget::{Base, Event, WRef};

pub mod input_event;
pub mod widget;

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
    #[must_use]
    pub fn new(_cfg: ()) -> Self {
        let root = WRef::new(Base::new([0.0; 4].into()));
        let sub_widget = WRef::new(Panel::new([100.0, 100.0, 50.0, 50.0].into()));
        for y in 0..3 {
            for x in 0..3 {
                let subsub_widget = WRef::new(Panel::new(
                    [
                        60.0f64.mul_add(f64::from(x), 100.0),
                        60.0f64.mul_add(f64::from(y), 100.0),
                        50.0,
                        50.0,
                    ]
                    .into(),
                ));
                sub_widget.borrow_mut().add_widget(
                    sub_widget.clone(),
                    &mut *subsub_widget.borrow_mut(),
                    subsub_widget.clone(),
                );
            }
        }
        root.borrow_mut().add_widget(
            root.clone(),
            &mut *sub_widget.borrow_mut(),
            sub_widget.clone(),
        );
        Self {
            hovered: root.clone(),
            root,
            state: State { mouse: Vec2f::new(0.0, 0.0), caught: None },
        }
    }

    /// Handle event
    pub fn handle_event(&mut self, event: InputEvent) {
        if let InputEvent::MouseMove(x, y) = event {
            self.state.mouse = (x, y).into();
        }

        if let Some(ref c) = self.state.caught {
            let w = c.widget.clone();
            w.borrow_mut().handle_event(w.clone(), Event::InputEvent(event), &mut self.state);
        }
        self.hovered.borrow_mut().handle_event(
            self.hovered.clone(),
            Event::InputEvent(event),
            &mut self.state,
        );
        if let Some(ref c) = self.state.caught {
            c.widget.borrow_mut().set_positon(self.state.mouse + c.offset);
        }
        self.update_hovered(self.state.mouse);
    }

    /// Update hovered widget
    fn update_hovered(&mut self, pos: Vec2f) {
        let hovered = self.root.borrow().get_hovered(pos).unwrap_or_else(|| self.root.clone());
        if self.hovered != hovered {
            hovered.borrow_mut().handle_event(hovered.clone(), Event::MouseEnter, &mut self.state);
            self.hovered.borrow_mut().handle_event(
                self.hovered.clone(),
                Event::MouseLeave,
                &mut self.state,
            );
            self.hovered = hovered;
        }
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
