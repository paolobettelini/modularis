//! Optional Bevy 0.19 adapter.

use bevy_asset::RenderAssetUsages;
use bevy_mesh::{Indices, Mesh, PrimitiveTopology};

use crate::{BakedMeshPart, BakedQuad, ResourceLocation, group_quads_by_texture};

/// Converts baked quads into one Bevy mesh per texture. This layout works well
/// with chunk meshing because each returned mesh can use one material/atlas page.
pub fn quads_to_bevy_meshes(quads: &[BakedQuad]) -> Vec<(ResourceLocation, Mesh)> {
    group_quads_by_texture(quads)
        .into_iter()
        .map(|part| {
            let texture = part.texture.clone();
            (texture, mesh_part_to_bevy(part))
        })
        .collect()
}

pub fn mesh_part_to_bevy(part: BakedMeshPart) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_indices(Indices::U32(part.indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, part.positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, part.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, part.uvs)
}
