use std::collections::{HashMap, HashSet};

use super::structs::{WarehouseMovement, WarehousePosition};

pub fn take_step(
    player: &WarehousePosition, 
    step: &WarehouseMovement,  
    objects: &HashMap<usize, WarehousePosition>, 
    walls: &HashSet<WarehousePosition>
    ) -> (Option<WarehousePosition>, Option<HashMap<usize, WarehousePosition>>) {

    let step = step.delta_position();

    let next_player = *player + step;
    let mut test_position = *player + step;
    let mut moved_objects: HashMap<usize, WarehousePosition> = HashMap::new();
    
    loop {
        if walls.contains(&test_position) { return (None,None)}
        if let Some((idx, pos)) = objects.iter().find(|(_, pos)| **pos == test_position) {
            moved_objects.insert(*idx, *pos + step);
            test_position += step;
        } else if moved_objects.is_empty() {
            return (Some(next_player), None);
        } else {
            return (Some(next_player), Some(moved_objects));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::structs::WarehousePosition as Pos;
    use super::super::structs::WarehouseMovement as Mov;

    #[test]
    fn test_step_just_player() {
        let (player, objects) = take_step(&Pos::new(5,5), &Mov::North, &HashMap::new(), &HashSet::new());
        assert_eq!(player, Some(Pos::new(5,4)));
        assert_eq!(objects, None);
    }
    #[test]
    fn test_step_just_player_hitting_wall() {
        let (player, objects) = take_step(&Pos::new(5,5), &Mov::North, &HashMap::new(), &HashSet::from([Pos::new(5,4)]));
        assert_eq!(player, None);
        assert_eq!(objects, None);
    }
    #[test]
    fn test_step_player_and_one_object() {
        let mut objects: HashMap<usize, Pos> = HashMap::new();
        objects.insert(34, Pos::new(5,4));
        objects.insert(24, Pos::new(4,4));

        let (player, objects) = take_step(&Pos::new(5,5), &Mov::North, &objects, &HashSet::new());

        assert_eq!(player, Some(Pos::new(5,4)));
        let mut facit_objects: HashMap<usize, Pos> = HashMap::new();
        facit_objects.insert(34, Pos::new(5,3));
        assert_eq!(objects, Some(facit_objects) );
    }
    #[test]
    fn test_step_player_and_one_object_hitting_wall() {
        let mut objects: HashMap<usize, Pos> = HashMap::new();
        objects.insert(34, Pos::new(5,4));

        let (player, objects) = take_step(&Pos::new(5,5), &Mov::North, &objects, &HashSet::from([Pos::new(5,3)]));

        assert_eq!(player, None);
        assert_eq!(objects, None);
    }
}
