use std::sync::LazyLock;
use std::time::Duration;

use bevy::prelude::*;

use crate::PuzzleState;
use crate::warehouse::structs::warehouse::Warehouse;
use crate::warehouse::take_step::take_step;

use super::player::{player_transform, RenderPlayer, RenderPlayerLight};
use super::objects::{object_transform, RenderObject};
use super::smooth::{SmoothObject, TurnOffTheLight};

const TICK:u64 = 800;

#[derive(Resource)] pub struct PuzzleSolvingTicker { 
    pub timer: Timer,
    pub duration: u64
}

impl PuzzleSolvingTicker {
    pub fn update_duration(&mut self) {
        self.timer.set_duration(Duration::from_millis(self.duration));
    }
    
    pub fn new(time: u64) -> Self {
        PuzzleSolvingTicker { 
            timer: Timer::new(Duration::from_millis(time), TimerMode::Repeating),
            duration: time
        }
    }
}

pub fn setup_puzzle_ticker( mut commands: Commands,) {
    commands.insert_resource(PuzzleSolvingTicker::new(TICK));
}

pub fn change_speed(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_duration: ResMut<PuzzleSolvingTicker>,
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
        (KeyCode::Digit0, 5)]);

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
            if let Some(player) = player { warehouse.player = player; }
            if let Some(objects) = objects { warehouse.objects.extend(objects); }
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
    player_query: Query<(Entity, &RenderPlayer, &Transform), Without<RenderObject>>,
    mut light_query: Query<(Entity, &mut PointLight), With<RenderPlayerLight>>, 
    objects_query: Query<(Entity, &RenderObject, &Transform)>,
    mut next_puzzle_state: ResMut<NextState<PuzzleState>>,
    mut warehouse: ResMut<Warehouse>,
    mut puzzle_ticker: ResMut<PuzzleSolvingTicker>,
) {
    puzzle_ticker.timer.tick(time.delta());

    if !next_puzzle_state.is_added() && puzzle_ticker.timer.finished() && !warehouse.movements.is_empty() {
        let mut anim = puzzle_ticker.timer.duration().as_millis();
        if anim > 10 { anim = (anim * 7) / 10; }

        let step = warehouse.movements.remove(0);

        if warehouse.movements.is_empty() { next_puzzle_state.set(PuzzleState::Scoring) } 

        puzzle_ticker.update_duration();

        let (player, moved_objects) = take_step(&warehouse.player, &step, &warehouse.objects, &warehouse.walls);

        let (player_entity, _, p_transform) = player_query.single();
        let pos = warehouse.player + step.delta_position();

        if let Some(player) = player {
            warehouse.player = player;
        } else {
            let (color_entity, mut light) = light_query.single_mut();
            light.color = Color::srgb(1.0, 0.0, 0.0);
            commands.entity(color_entity).insert( TurnOffTheLight::new((anim/2) as u64));
        }

        commands.entity(player_entity).insert(SmoothObject::new(*p_transform, player_transform(&pos, &warehouse), anim as u64, player.is_some()));

        if let Some(objects) = moved_objects {
            for (idx, pos) in objects {
                warehouse.objects.insert(idx, pos);
                if let Some((object_entity, _ , t)) = objects_query.iter().find(|(_, o, _)| o.index == idx) {
                    commands.entity(object_entity).insert(SmoothObject::new(*t, object_transform(&pos, &warehouse), anim as u64, true)); 
                }
            }
        } 
    }
}
