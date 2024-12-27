use bevy::prelude::*;

use crate::warehouse::structs::WarehousePosition;

pub mod walls;
pub mod floor;
pub mod objects;
pub mod player;
pub mod puzzle;

#[derive(Component)]
pub struct RenderWarehousePosition(WarehousePosition);

pub fn setup_things( mut commands: Commands,) {
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            range: 1060.0,
            radius: 30.0,
            intensity: 50_000_000.0,
            ..default()
        },
        Transform::from_xyz(-30.0, -30.0, 32.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(52.5, 4.5, 59.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));

}

