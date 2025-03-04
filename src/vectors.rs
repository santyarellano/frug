use std::ops;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2d<T> {
    pub x: T,
    pub y: T,
}

impl ops::AddAssign for Vec2d<i32> {
    fn add_assign(&mut self, rhs: Vec2d<i32>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
