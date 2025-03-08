//! Rectangle with position.
use builder::config::value::{Error as ParseError, Value};
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

impl<E: std::error::Error, T: TryFrom<Value, Error = E>> TryFrom<Value> for Rect<T> {
    type Error = ParseError;
    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        Ok(<[T; 4]>::try_from(value)?.into())
    }
}
