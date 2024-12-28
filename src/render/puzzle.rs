use std::time::Duration;

use bevy::prelude::*;

use crate::warehouse::take_step::take_step;
use crate::warehouse::structs::Warehouse;
use crate::PuzzleState;

use super::player::{player_transform, RenderPlayer, RenderPlayerLight};
use super::objects::{object_transform, RenderObject};

const TICK:u64 = 400;

#[derive(Resource)]
pub struct PuzzleSolvingTicker {
    pub timer: Timer,
}

pub fn setup_puzzle_ticker( mut commands: Commands,) {
    commands.insert_resource(PuzzleSolvingTicker {
        timer: Timer::new(Duration::from_millis(TICK), TimerMode::Repeating),
    })
}

#[derive(Component)]
pub struct SmoothObject {
    pub index: usize,
    pub from: Transform,
    pub to: Transform,
    pub timer: Timer,
    pub time: u128
}

#[derive(Component)]
pub struct SmoothPlayer {
    pub from: Transform,
    pub to: Transform,
    pub timer: Timer,
    pub time: u128,
    pub good: bool

}
#[derive(Component)]
pub struct ScoreBoard {
    pub score: usize
}

pub fn smooth_object(
    mut commands: Commands,
    time: Res<Time>,
    mut objects_query: Query<(&RenderObject, &mut Transform)>,
    mut smooth_query: Query<(Entity, &mut SmoothObject)>,
    next_puzzle_state: Res<NextState<PuzzleState>>,
) {
    if !next_puzzle_state.is_added() {
        for (entity, mut smooth) in &mut smooth_query {
            smooth.timer.tick(time.delta());
            for (o, mut t) in &mut objects_query {
                if smooth.timer.finished() { commands.entity(entity).despawn(); }
                if o.index == smooth.index {
                    if smooth.timer.finished() || smooth.timer.duration().as_millis() < 3 {
                        *t = smooth.to;
                    } else {
                        let d = (smooth.timer.elapsed().as_millis() as f32) / (smooth.time as f32);
                        t.translation = smooth.from.translation +  (smooth.to.translation - smooth.from.translation) * d;
                    }
                }
            }
        }
    }
}

pub fn smooth_player(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(&RenderPlayer, &mut Transform)>,
    mut smooth_query: Query<(Entity, &mut SmoothPlayer)>,
    mut light_query: Query<&mut PointLight, With<RenderPlayerLight>>, 
    next_puzzle_state: Res<NextState<PuzzleState>>,
) {
    if !next_puzzle_state.is_added() {
        for (entity, mut smooth) in &mut smooth_query {
            smooth.timer.tick(time.delta());
            for ( _, mut t) in &mut player_query {
                if smooth.timer.finished() || smooth.timer.duration().as_millis() < 3 {
                    *t = if smooth.good { smooth.to } else { smooth.from };
                    commands.entity(entity).despawn();
                   
                    light_query.single_mut().color = Color::srgb(1.0, 1.0, 1.0);

                } else {
                    let elapsed = smooth.timer.elapsed().as_millis();

                    let elapsed = if !smooth.good && elapsed > (smooth.time / 2) {
                        smooth.time - elapsed
                    } else {
                        elapsed
                    };

                    let d = (elapsed as f32) / (smooth.time as f32);
                    t.translation = smooth.from.translation +  (smooth.to.translation - smooth.from.translation) * d;
                }
            }
        }
    }
}

pub fn escape_the_matrix(
    keys: Res<ButtonInput<KeyCode>>,
    // mut commands: Commands,
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
) {
    puzzle_ticker.timer.tick(time.delta());
    let mut anim = puzzle_ticker.timer.duration().as_millis();

    if !next_puzzle_state.is_added() && puzzle_ticker.timer.finished() && !warehouse.movements.is_empty() {
        let step = warehouse.movements.remove(0);

        if warehouse.movements.is_empty() { next_puzzle_state.set(PuzzleState::Scoring) } 
        else if anim > 2 { puzzle_ticker.timer.set_duration(Duration::from_millis(anim as u64 - 1)); }

        if anim > 150 { anim = (anim * 7) / 10; }

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
