use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign};

use bevy::prelude::Resource;

#[derive(Clone, Debug, Resource)]
pub struct Warehouse {
    pub walls: HashSet<WarehousePosition>,
    pub objects: HashMap<usize, WarehousePosition>,
    pub player: WarehousePosition,

    pub height: i32,
    pub width: i32,

    pub movements: Vec<WarehouseMovement>
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarehouseMovement {
    North,
    South,
    West,
    East
}

impl WarehouseMovement {
    pub fn delta_position(&self) -> WarehousePosition {
        match self {
            Self::North => WarehousePosition::new(0, -1),
            Self::South => WarehousePosition::new(0, 1),
            Self::West => WarehousePosition::new(-1, 0),
            Self::East => WarehousePosition::new(1, 0),
        }
    }
}

