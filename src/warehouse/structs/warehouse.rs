use std::collections::{HashMap, HashSet};

use bevy::prelude::{Resource, Transform};

use super::movement::WarehouseMovement;
use super::position::WarehousePosition;

#[derive(Clone, Debug, Resource)]
pub struct Warehouse {
    pub walls: HashSet<WarehousePosition>,
    pub objects: HashMap<usize, WarehousePosition>,
    pub player: WarehousePosition,

    pub height: i32,
    pub width: i32,

    pub movements: Vec<WarehouseMovement>
}

impl Warehouse {
    pub fn get_bevy_transform(&self, pos: &WarehousePosition, z: f32) -> Transform {
        Transform::from_xyz ( 
            pos.x as f32 - self.width as f32 / 2.0, 
            pos.y as f32 - self.height as f32 / 2.0,
            z)
    }
}
