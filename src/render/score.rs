use std::time::Duration;

use bevy::prelude::*;

use crate::warehouse::structs::Warehouse;
use crate::PuzzleState;

use super::objects::RenderObject;
use super::puzzle::PuzzleSolvingTicker;

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource)]
pub struct Score { pub score: usize }

pub fn setup_score(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut puzzle_ticker: ResMut<PuzzleSolvingTicker>,
) {
    commands.insert_resource(Score { score: 0});
    commands.spawn((
            Text::new(""),
            TextFont {
                font: asset_server.load("fonts/DejaVuSans.ttf"),
                font_size: 42.0,
                ..default()
            },
            ScoreText
    ));
    puzzle_ticker.timer.set_duration(Duration::from_millis(200));
    puzzle_ticker.timer.reset();
}


#[allow(clippy::too_many_arguments)]
pub fn score_trigger(
    mut commands: Commands,
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut warehouse: ResMut<Warehouse>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
    mut next_puzzle_state: ResMut<NextState<PuzzleState>>,
    objects_query: Query<(Entity, &RenderObject)>,
    mut puzzle_ticker: ResMut<PuzzleSolvingTicker>,
) {
    puzzle_ticker.timer.tick(time.delta());

    if puzzle_ticker.timer.finished() && !warehouse.objects.is_empty() {
        let anim = puzzle_ticker.timer.duration().as_millis();
        if anim > 4 { puzzle_ticker.timer.set_duration(Duration::from_millis(anim as u64 - 1)); }

        if let Some((entity, object)) = objects_query.iter().next() {
            if let Some(pos) = warehouse.objects.remove(&object.index) {
                score.score += 100 * pos.y as usize + pos.x as usize;
            }
            commands.entity(entity).despawn();
        }

        for mut text in &mut text_query { **text = format!("{}", score.score); }

        if warehouse.objects.is_empty() { next_puzzle_state.set(PuzzleState::Completed); }
    }
}
