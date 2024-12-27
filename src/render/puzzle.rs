use std::time::Duration;

use bevy::prelude::*;

use crate::warehouse::take_step::take_step;
use crate::warehouse::structs::Warehouse;

use super::{objects::{object_transform, RenderObject}, player::{player_transform, RenderPlayer}};


#[derive(Resource)]
pub struct PuzzleSolvingTicker {
    timer: Timer,
    // step_timer: Option<Timer>,
    // player_move: Option<WarehousePosition>,
    // objects_move: Option<HashMap<usize, WarehousePosition>>
}

pub fn setup_puzzle_ticker( mut commands: Commands,) {
    commands.insert_resource(PuzzleSolvingTicker {
        timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
    })
}

pub fn step_trigger(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(&RenderPlayer, &mut Transform), Without<RenderObject>>,
    mut objects_query: Query<(&RenderObject, &mut Transform)>,
    // mut objects_query: Query<(&RenderObject, &mut Transform), Without<RenderPlayer>>,
    mut warehouse: ResMut<Warehouse>,
    mut puzzle_ticker: ResMut<PuzzleSolvingTicker>,
) {
    puzzle_ticker.timer.tick(time.delta());


    if puzzle_ticker.timer.finished() {
        if !warehouse.movements.is_empty() {
            let step = warehouse.movements.remove(0);

            let (player, moved_objects) = take_step(&warehouse.player, &step, &warehouse.objects, &warehouse.walls);


            if let Some(player) = player {
                warehouse.player = player;
                let (_, mut transform) = player_query.single_mut();
                *transform = player_transform(&warehouse);
            }
            if let Some(objects) = moved_objects {
                for (idx, pos) in objects {
                    warehouse.objects.insert(idx, pos);
                    for (o, mut t) in objects_query.iter_mut() {
                        if o.index == idx {
                            *t = object_transform(&pos, &warehouse);
                        }
                    }
                }
            }

        } else {
            println!("do stuff");
        }
    } 
}

