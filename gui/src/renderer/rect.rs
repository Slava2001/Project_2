use std::ops::Add;

pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T: Copy + Add<Output = T> + PartialOrd<T>> Rect<T> {
    pub fn check_bounds(&self, x: T, y: T) -> bool {
        self.x <= x &&
        self.y <= y &&
        (self.x+self.w) >= x &&
        (self.y+self.h) >= y
    }
}

impl<T: Copy> From<[T; 4]> for Rect<T> {
    fn from(v: [T; 4]) -> Self {
        Self { x: v[0], y: v[1], w: v[2], h: v[3] }
    }
}

impl<T: Copy> Into<[T; 4]> for Rect<T> {
    fn into(self) -> [T; 4] {
        [self.x, self.y, self.w, self.h]
    }
}