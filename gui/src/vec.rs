use std::ops::Add;

pub struct Vec2<T> {
    x: T,
    y: T
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self { x: value[0], y: value[1] }
    }
}

impl<T> Add<Output = T> for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T> Add<Output = T> for &Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}