use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WarehousePosition {
    pub x: i32,
    pub y: i32
}

impl Add for WarehousePosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x:  self.x + rhs.x,
            y:  self.y + rhs.y,
        }
    }
}

impl AddAssign for WarehousePosition {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl WarehousePosition {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
