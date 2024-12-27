use std::collections::{HashMap, HashSet};

use anyhow::{bail, ensure};

use super::structs::{Warehouse, WarehouseMovement, WarehousePosition};

impl Warehouse {
    pub fn parse(text: &str) -> anyhow::Result<Self> {
        let mut walls: HashSet<WarehousePosition> = HashSet::new();
        let mut objects: HashMap<usize, WarehousePosition> = HashMap::new();
        let mut player: Option<WarehousePosition> = None;

        text.lines()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .for_each(|(y, line)| line.chars().enumerate()
                .for_each(|(x, ch)| match ch {
                    '#' => { let _ = walls.insert(WarehousePosition::new(x as i32,y as i32)); }
                    'O' => { let _ = objects.insert(objects.len(), WarehousePosition::new(x as i32, y as i32)); }
                    '@' => player = Some(WarehousePosition::new(x as i32, y as i32)),
                    _ => ()
                }));

        let (height, width) = walls.iter().fold((0,0), |(x, y), pos| (x.max(pos.x), y.max(pos.y)));

        let movements: Vec<WarehouseMovement> = text.lines()
            .skip_while(|line| !line.is_empty())
            .flat_map(|line| line.chars())
            .filter_map(WarehouseMovement::parse)
            .collect();

        ensure!(!walls.is_empty(), "Missing walls");
        ensure!(!objects.is_empty(), "Missing objects");
        ensure!(!movements.is_empty(), "Missing movements");

        if let Some(player) = player {
            Ok(Self {
                walls,
                objects,
                height,
                width,
                movements,
                player,
            })
        } else {
            bail!("Missing player");
        }
    }
}

impl WarehouseMovement {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example() {
        let warehouse = Warehouse::parse(include_str!("small_example.txt"));
        assert!(warehouse.is_ok());
        let warehouse = warehouse.unwrap();
        assert_eq!(warehouse.objects.len(), 6);
        assert_eq!(warehouse.player, WarehousePosition::new(2, 2));
        assert_eq!(warehouse.movements.len(), 15);
    }
}
