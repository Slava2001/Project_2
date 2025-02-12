//! Rectangle with position.
use std::ops::Add;

/// Rectangle
#[derive(Clone, Copy)]
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

impl<T: Copy + Add<Output = T> + PartialOrd<T>> Rect<T> {
    /// Check what (`x`, `y`) point inside rectangle bounds.
    pub fn check_bounds(&self, x: T, y: T) -> bool {
        self.x < x && self.y < y && (self.x + self.w) > x && (self.y + self.h) > y
    }
}

impl<T: Copy> From<[T; 4]> for Rect<T> {
    fn from(v: [T; 4]) -> Self {
        Self { x: v[0], y: v[1], w: v[2], h: v[3] }
    }
}
