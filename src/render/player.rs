use bevy::prelude::*;

use crate::warehouse::structs::Warehouse;

pub fn add_player(
    mut commands: Commands,
    warehouse: Res<Warehouse>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cone::new(0.5, 3.0));
    let material = materials.add(Color::srgb(1.0, 0.2, 0.3));

    commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(
                warehouse.player.x as f32 - warehouse.width as f32 / 2.0, 
                warehouse.player.y as f32 - warehouse.width as f32 / 2.0, 
                1.5).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));

    commands.spawn((
            PointLight {
                shadows_enabled: true,
                range: 7.0,
                // radius: 30.0,
                intensity: 1_000_000.0,
                ..default()
            },
            Transform::from_xyz(
                warehouse.player.x as f32 - warehouse.width as f32 / 2.0, 
                warehouse.player.y as f32 - warehouse.width as f32 / 2.0, 
                5.5)

    ));
}
