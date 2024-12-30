use std::sync::LazyLock;
use std::time::Duration;

use bevy::prelude::*;

use crate::warehouse::take_step::take_step;
use crate::warehouse::structs::Warehouse;
use crate::PuzzleState;

use super::player::{player_transform, RenderPlayer, RenderPlayerLight};
use super::objects::{self, object_transform, RenderObject};
use super::smooth::{SmoothObject, SmoothPlayer};

const TICK:u64 = 800;

#[derive(Resource)] pub struct PuzzleSolvingTicker { pub timer: Timer, }
#[derive(Resource)] pub struct NextDuration { pub duration: u64 }
#[derive(Component)] pub struct ScoreBoard { pub score: usize }

pub fn setup_puzzle_ticker( mut commands: Commands,) {
    commands.insert_resource(PuzzleSolvingTicker { timer: Timer::new(Duration::from_millis(TICK), TimerMode::Repeating), });
    commands.insert_resource(NextDuration { duration: TICK });
}

pub fn change_speed(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_duration: ResMut<NextDuration>,
) {
    static KEY_DELAY: LazyLock<Vec<(KeyCode, u64)>> = LazyLock::new(|| vec![ 
        (KeyCode::Digit1, TICK),
        (KeyCode::Digit2, 400),
        (KeyCode::Digit3, 300),
        (KeyCode::Digit4, 200),
        (KeyCode::Digit5, 150),
        (KeyCode::Digit6, 100),
        (KeyCode::Digit7, 50),
        (KeyCode::Digit8, 20),
        (KeyCode::Digit9, 10),
        (KeyCode::Digit0, 1)]);

    if let Some((_, delay)) = KEY_DELAY.iter().find(|(key_code, _)| keys.just_pressed(*key_code) ) {
        next_duration.duration = *delay;
    }
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
                warehouse.objects.extend(objects);
            }
        }

        let (_, mut t) = player_query.single_mut();
        *t = player_transform(&warehouse.player ,&warehouse);

        objects_query.iter_mut()
            .for_each(|(o, mut t)| if let Some(pos) = warehouse.objects.get(&o.index) {
                *t = object_transform(pos, &warehouse);
            });

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
        if anim > 100 { anim = (anim * 7) / 10; }

        let step = warehouse.movements.remove(0);

        if warehouse.movements.is_empty() { next_puzzle_state.set(PuzzleState::Scoring) } 

        puzzle_ticker.timer.set_duration(Duration::from_millis(next_duration.duration));

        let (player, moved_objects) = take_step(&warehouse.player, &step, &warehouse.objects, &warehouse.walls);

        let (_,t) = player_query.single();
        let pos = warehouse.player + step.delta_position();

        if let Some(player) = player {
            warehouse.player = player;
        } else {
            light_query.single_mut().color = Color::srgb(1.0, 0.0, 0.0);
        }

        commands.spawn(SmoothPlayer {
            from: *t,
            to: player_transform(&pos, &warehouse),
            timer: Timer::new(Duration::from_millis(anim as u64), TimerMode::Once),
            time: anim,
            good: player.is_some(),
        });

        if let Some(objects) = moved_objects {
            for (idx, pos) in objects {
                warehouse.objects.insert(idx, pos);
                if let Some((_ , t)) = objects_query.iter().find(|(o, _)| o.index == idx) {
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
