use bevy::prelude::*;

use crate::warehouse::structs::{Warehouse, WarehousePosition};

#[derive(Component)]
pub struct RenderPlayer;

#[derive(Component)]
pub struct RenderPlayerLight;

pub fn add_player(
    mut commands: Commands,
    warehouse: Res<Warehouse>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cone::new(0.5, 3.0));
    let material = materials.add(Color::srgb(1.0, 0.2, 0.3));

    commands.spawn((
            RenderPlayer,
            player_transform(&warehouse.player,&warehouse)
    ))
        .with_children(|parent| {
            parent.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
            ));
            parent.spawn((
                    PointLight {
                        shadows_enabled: true,
                        range: 7.0,
                        // radius: 30.0,
                        intensity: 500_000.0,
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.0, 3.5),
                    RenderPlayerLight
            ));
        });
}

pub fn player_transform(position: &WarehousePosition, warehouse: &Warehouse) -> Transform {
    warehouse.get_bevy_transform(position, 1.5)
}
