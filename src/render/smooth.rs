use bevy::prelude::*;

use crate::PuzzleState;

use super::objects::RenderObject;
use super::player::{RenderPlayer, RenderPlayerLight};

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
    pub good: bool
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
            let elapsed = smooth.timer.elapsed().as_millis();
            let duration = smooth.timer.duration().as_millis();

            for ( _, mut t) in &mut player_query {
                if smooth.timer.finished() || duration < 3 || !smooth.good && elapsed > duration / 2 {
                    *t = if smooth.good { smooth.to } else { smooth.from };
                    commands.entity(entity).despawn();
                    if !smooth.good {
                        light_query.single_mut().color = Color::srgb(1.0, 1.0, 1.0);
                    }
                } else {
                    let elapsed = match (smooth.good, elapsed > duration / 4) {
                        (true, _) => elapsed,
                        (false, false) => elapsed / 2,
                        (false, true) => duration / 2 - elapsed / 2
                    };

                    let d = (elapsed as f32) / (duration as f32);
                    t.translation = smooth.from.translation +  (smooth.to.translation - smooth.from.translation) * d;
                }
            }
        }
    }
}
