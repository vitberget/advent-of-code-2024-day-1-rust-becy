use super::position::WarehousePosition;

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

    pub fn parse(ch: char) -> Option<Self> {
        match ch {
            '<' => Some(Self::West),
            '>' => Some(Self::East),
            '^' => Some(Self::North),
            'v' => Some(Self::South),
            _ => None
        } 
    }
}
