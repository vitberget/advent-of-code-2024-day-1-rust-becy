use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

pub fn get_text_mesh(text: &str, pixel_size: f32) -> (Mesh, Vec3) {
    use meshtext::{MeshGenerator, MeshText, TextSection};
    let font_data = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/fonts/DejaVuSans.ttf"
    ));
    let mut generator = MeshGenerator::new(font_data);
    let transform = Mat4::from_scale(Vec3::new(pixel_size, pixel_size, 0.)).to_cols_array();
    let text_mesh: MeshText = generator
        .generate_section(text, true, Some(&transform))
        .unwrap();

    let vertices = text_mesh.vertices;
    let positions: Vec<[f32; 3]> = vertices.chunks(3).map(|c| [c[0], c[1], c[2]]).collect();
    let uvs = vec![[0f32, 0f32]; positions.len()];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.compute_flat_normals();

    (mesh, Vec3::X * (text_mesh.bbox.size().x / -2.) + Vec3::Y * (text_mesh.bbox.size().y / -2.))
}
