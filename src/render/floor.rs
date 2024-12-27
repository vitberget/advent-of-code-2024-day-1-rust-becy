use bevy::prelude::*;

use crate::warehouse::structs::Warehouse;

pub fn add_floor(
    mut commands: Commands,
    warehouse: Res<Warehouse>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(warehouse.width as f32, warehouse.height as f32))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        // Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
}
