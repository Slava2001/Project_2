//! 2D vector.

use builder::config::value::{Error as ParseError, ParseFormValue, Value};
use error_stack::Result;
use std::ops::{Add, Sub};

/// f64 vec2.
pub type Vec2f = Vec2<f64>;

/// 2D vector.
#[derive(Copy, Clone, Debug)]
pub struct Vec2<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
}

impl<T> Vec2<T> {
    /// Create new 2D vector.
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from(value: [T; 2]) -> Self {
        let [x, y] = value;
        Self::new(x, y)
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: Add<Output = T> + Copy> Add for &Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::<T> { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for &Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::<T> { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T: ParseFormValue> ParseFormValue for Vec2<T> {
    fn parse_val(val: Value) -> Result<Self, ParseError> {
        Ok(<[T; 2]>::parse_val(val)?.into())
    }
}
