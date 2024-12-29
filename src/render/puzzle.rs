use std::time::Duration;

use bevy::prelude::*;

use crate::warehouse::take_step::take_step;
use crate::warehouse::structs::Warehouse;
use crate::PuzzleState;

use super::player::{player_transform, RenderPlayer, RenderPlayerLight};
use super::objects::{object_transform, RenderObject};
use super::smooth::{SmoothObject, SmoothPlayer};

const TICK:u64 = 400;

#[derive(Resource)] pub struct PuzzleSolvingTicker { pub timer: Timer, }
#[derive(Resource)] pub struct NextDuration { pub duration: u64 }
#[derive(Component)] pub struct ScoreBoard { pub score: usize }

pub fn setup_puzzle_ticker( mut commands: Commands,) {
    commands.insert_resource(PuzzleSolvingTicker { timer: Timer::new(Duration::from_millis(TICK), TimerMode::Repeating), });
    commands.insert_resource(NextDuration { duration: 400 });
}

pub fn change_speed(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_duration: ResMut<NextDuration>,
) {
    if keys.just_pressed(KeyCode::Digit1) { next_duration.duration = 800 }
    if keys.just_pressed(KeyCode::Digit2) { next_duration.duration = 400 }
    if keys.just_pressed(KeyCode::Digit3) { next_duration.duration = 300 }
    if keys.just_pressed(KeyCode::Digit4) { next_duration.duration = 200 }
    if keys.just_pressed(KeyCode::Digit5) { next_duration.duration = 150 }
    if keys.just_pressed(KeyCode::Digit6) { next_duration.duration = 100 }
    if keys.just_pressed(KeyCode::Digit7) { next_duration.duration = 50 }
    if keys.just_pressed(KeyCode::Digit8) { next_duration.duration = 20 }
    if keys.just_pressed(KeyCode::Digit9) { next_duration.duration = 10 }
    if keys.just_pressed(KeyCode::Digit0) { next_duration.duration = 1 }
}

pub fn escape_the_matrix(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&RenderPlayer, &mut Transform), Without<RenderObject>>,
    mut light_query: Query<&mut PointLight, With<RenderPlayerLight>>, 
    mut objects_query: Query<(&RenderObject, &mut Transform)>,
    mut next_puzzle_state: ResMut<NextState<PuzzleState>>,
    mut warehouse: ResMut<Warehouse>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        for step in  warehouse.movements.clone() {
            let (player, objects) = take_step(&warehouse.player, &step, &warehouse.objects, &warehouse.walls); 
            if let Some(player) = player {
                warehouse.player = player;
            }
            if let Some(objects) = objects {
                for (key, value) in objects {
                    warehouse.objects.insert(key, value);
                }
            }
        }

        let (_, mut t) = player_query.single_mut();
        *t = player_transform(&warehouse);

        for (o, mut t) in &mut objects_query {
            if let Some(pos) = warehouse.objects.get(&o.index) {
                *t = object_transform(pos, &warehouse);
            }
        }

        light_query.single_mut().color = Color::srgb(1.0, 1.0, 1.0);

        next_puzzle_state.set(PuzzleState::Scoring);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn step_trigger(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<(&RenderPlayer, &Transform), Without<RenderObject>>,
    mut light_query: Query<&mut PointLight, With<RenderPlayerLight>>, 
    objects_query: Query<(&RenderObject, &Transform)>,
    mut next_puzzle_state: ResMut<NextState<PuzzleState>>,
    mut warehouse: ResMut<Warehouse>,
    mut puzzle_ticker: ResMut<PuzzleSolvingTicker>,
    next_duration: Res<NextDuration>,
) {
    puzzle_ticker.timer.tick(time.delta());

    if !next_puzzle_state.is_added() && puzzle_ticker.timer.finished() && !warehouse.movements.is_empty() {
        let mut anim = puzzle_ticker.timer.duration().as_millis();
        if anim > 150 { anim = (anim * 7) / 10; }

        let step = warehouse.movements.remove(0);

        if warehouse.movements.is_empty() { next_puzzle_state.set(PuzzleState::Scoring) } 
        
        puzzle_ticker.timer.set_duration(Duration::from_millis(next_duration.duration));

        let (player, moved_objects) = take_step(&warehouse.player, &step, &warehouse.objects, &warehouse.walls);

        if let Some(player) = player {
            warehouse.player = player;
            let (_,t) = player_query.single();
            commands.spawn(SmoothPlayer {
                from: *t,
                to: player_transform(&warehouse),
                timer: Timer::new(Duration::from_millis(anim as u64), TimerMode::Once),
                time: anim,
                good: true

            });
        } else {
            let (_,t) = player_query.single();
            let pos = warehouse.player + step.delta_position();
            commands.spawn(SmoothPlayer {
                from: *t,
                to: object_transform(&pos, &warehouse),
                timer: Timer::new(Duration::from_millis(anim as u64), TimerMode::Once),
                time: anim,
                good: false
            });
            light_query.single_mut().color = Color::srgb(1.0, 0.0, 0.0);
        }
        if let Some(objects) = moved_objects {
            for (idx, pos) in objects {
                warehouse.objects.insert(idx, pos);
                for (o, t) in &objects_query {
                    if o.index == idx {
                        commands.spawn(SmoothObject {
                            index: idx,
                            from: *t,
                            to: object_transform(&pos, &warehouse),
                            timer: Timer::new(Duration::from_millis(anim as u64), TimerMode::Once),
                            time: anim
                        });
                    }
                }
            }
        }
    } 
}
