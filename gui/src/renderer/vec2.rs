use std::ops::{Add, Sub};

pub type Vec2f = Vec2<f64>;

#[derive(Copy, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
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
