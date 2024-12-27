use bevy::prelude::*;

use crate::warehouse::structs::{Warehouse, WarehousePosition};

#[derive(Component)]
pub struct RenderObject { pub index: usize }

pub fn add_objects(
    mut commands: Commands,
    warehouse: Res<Warehouse>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(0.5));
    let material = materials.add(Color::srgb(0.5, 0.9, 0.3));

    for (idx, pos) in warehouse.objects.iter() {
        commands.spawn((
            RenderObject{ index: *idx },
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            object_transform(pos, &warehouse)
        ));
    };
}

pub fn object_transform(pos: &WarehousePosition, warehouse: &Warehouse) -> Transform {
    Transform::from_xyz(
        pos.x as f32 - warehouse.width as f32 / 2.0, 
        pos.y as f32 - warehouse.width as f32 / 2.0, 
        0.5)
}
