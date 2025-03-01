//! Rectangle with position.
use builder::config::value::{Error as ParseError, ParseFormValue, Value};
use error_stack::Result;
use std::{
    fmt::{Debug, Display},
    ops::Add,
};

/// Rectangle with f64 fields.
pub type Rectf = Rect<f64>;

/// Rectangle
#[derive(Clone, Copy, Debug)]
pub struct Rect<T> {
    /// X coordinate.
    pub x: T,
    /// Y coordinate.
    pub y: T,
    /// Rectangle width.
    pub w: T,
    /// Rectangle hight.
    pub h: T,
}

impl<T: Display + Debug + Copy + Add<Output = T> + PartialOrd<T>> Rect<T> {
    /// Check what (`x`, `y`) point inside rectangle bounds.
    pub fn check_bounds(&self, x: T, y: T) -> bool {
        self.x < x && self.y < y && (self.x + self.w) > x && (self.y + self.h) > y
    }
}

impl<T> From<[T; 4]> for Rect<T> {
    fn from(value: [T; 4]) -> Self {
        let [x, y, w, h] = value;
        Self { x, y, w, h }
    }
}

impl<T: ParseFormValue> ParseFormValue for Rect<T> {
    fn parse_val(val: Value) -> Result<Self, ParseError> {
        Ok(<[T; 4]>::parse_val(val)?.into())
    }
}
