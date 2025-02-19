use bevy::prelude::*;

use crate::warehouse::structs::warehouse::Warehouse;

pub fn add_floor(
    mut commands: Commands,
    warehouse: Res<Warehouse>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(warehouse.width as f32, warehouse.height as f32))),
        MeshMaterial3d(materials.add(
                StandardMaterial {
                    base_color: Color::WHITE,
                    metallic: 1.0,
                    reflectance: 0.7,
                    
                    ..default()
                })),
    ));
}
