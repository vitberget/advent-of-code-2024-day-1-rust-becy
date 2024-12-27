use std::time::Duration;

use bevy::prelude::*;

use crate::warehouse::take_step::take_step;
use crate::warehouse::structs::Warehouse;

use super::player::{player_transform, RenderPlayer};
use super::objects::{object_transform, RenderObject};

const TICK:u64 = 500;
const ANIM:usize = 400;

#[derive(Resource)]
pub struct PuzzleSolvingTicker {
    timer: Timer,
    // step_timer: Option<Timer>,
    // player_move: Option<WarehousePosition>,
    // objects_move: Option<HashMap<usize, WarehousePosition>>
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
    pub time: usize
}

#[derive(Component)]
pub struct SmoothPlayer {
    pub from: Transform,
    pub to: Transform,
    pub timer: Timer,
    pub time: usize,
    pub good: bool

}

pub fn smooth_object(
    mut commands: Commands,
    time: Res<Time>,
    mut objects_query: Query<(&RenderObject, &mut Transform)>,
    mut smooth_query: Query<(Entity, &mut SmoothObject)>
) {
    for (entity, mut smooth) in &mut smooth_query {
        smooth.timer.tick(time.delta());
        for (o, mut t) in &mut objects_query {
            if smooth.timer.finished() { commands.entity(entity).despawn(); }
            if o.index == smooth.index {
                if smooth.timer.finished() {
                    *t = smooth.to;
                } else {
                    let d = (smooth.timer.elapsed().as_millis() as f32) / (smooth.time as f32);
                    t.translation = smooth.from.translation +  (smooth.to.translation - smooth.from.translation) * d;
                }
            }
        }
    }
}

pub fn smooth_player(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(&RenderPlayer, &mut Transform)>,
    mut smooth_query: Query<(Entity, &mut SmoothPlayer)>
) {
    for (entity, mut smooth) in &mut smooth_query {
        smooth.timer.tick(time.delta());
        for (_, mut t) in &mut player_query {
            if smooth.timer.finished() {
                *t = if smooth.good { smooth.to } else { smooth.from };
                commands.entity(entity).despawn();
            } else {
                let elapsed = smooth.timer.elapsed().as_millis();

                let elapsed = if !smooth.good && elapsed > (smooth.time as u128 / 2) {
                    smooth.time as u128 - elapsed
                } else {
                    elapsed
                };

                let d = (elapsed as f32) / (smooth.time as f32);
                t.translation = smooth.from.translation +  (smooth.to.translation - smooth.from.translation) * d;
            }
        }
    }
}

pub fn step_trigger(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<(&RenderPlayer, &Transform), Without<RenderObject>>,
    objects_query: Query<(&RenderObject, &Transform)>,
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
                let (_,t) = player_query.single();
                commands.spawn(SmoothPlayer {
                    from: *t,
                    to: player_transform(&warehouse),
                    timer: Timer::new(Duration::from_millis(ANIM as u64), TimerMode::Once),
                    time: ANIM,
                    good: true

                });
            } else {
                let (_,t) = player_query.single();
                let pos = warehouse.player + step.delta_position();
                commands.spawn(SmoothPlayer {
                    from: *t,
                    to: object_transform(&pos, &warehouse),
                    timer: Timer::new(Duration::from_millis(ANIM as u64), TimerMode::Once),
                    time: ANIM,
                    good: false
                });
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
                                timer: Timer::new(Duration::from_millis(ANIM as u64), TimerMode::Once),
                                time: ANIM
                            });
                        }
                    }
                }
            }

        } else {
            println!("do stuff");
        }
    } 
}

