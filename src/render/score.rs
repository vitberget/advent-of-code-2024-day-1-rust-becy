use bevy::prelude::*;

use crate::warehouse::structs::Warehouse;

use super::objects::{self, RenderObject};
use super::text::get_text_mesh;
use super::puzzle::PuzzleSolvingTicker;

#[derive(Component)]
pub struct ScoreTest;

#[derive(Resource)]
pub struct Score { pub score: usize }

pub fn setup_score(mut commands: Commands) {
    commands.insert_resource(Score { score: 0});
}


#[allow(clippy::too_many_arguments)]
pub fn score_trigger(
    mut commands: Commands,
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut warehouse: ResMut<Warehouse>,
    scores_query: Query<(Entity, &ScoreTest)>,
    objects_query: Query<(Entity, &RenderObject)>,
    mut puzzle_ticker: ResMut<PuzzleSolvingTicker>,
) {

    puzzle_ticker.timer.tick(time.delta());
    if puzzle_ticker.timer.finished() && warehouse.movements.is_empty() && !warehouse.objects.is_empty() {
        for (entity, _) in &scores_query { commands.entity(entity).despawn(); }
        if let Some((entity, object)) = objects_query.iter().next() {
           if let Some(pos) = warehouse.objects.remove(&object.index) {
               println!("pos {:?}", pos);
                score.score += 100 * pos.y as usize + pos.x as usize;
           }
           commands.entity(entity).despawn();
        }

        let (mesh, offset) = get_text_mesh(format!("{}", score.score).as_str(), 4.0);

        commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
                Transform::from_xyz(0.0, offset.x, 5.4).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ScoreTest
        ));
    }
}
