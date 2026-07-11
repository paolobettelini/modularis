use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use block_render_api::{BlockFace, RenderShape};
use client_chunk_mesh_api::{
    ChunkMeshApi, ChunkMeshData, ChunkMeshNeighborhood, ChunkMeshPart, ChunkMeshService,
};
use client_chunk_vertex_lighting_api::{
    ChunkVertexLightingPipeline, ChunkVertexLightingSnapshot, ClientChunkVertexLightingApi,
    VertexOcclusion,
};
use std::marker::PhantomData;
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, LocalBlockPos};

pub struct NaiveCubeChunkMesher<B>(PhantomData<B>);

impl<B: BlockManagerApi> NaiveCubeChunkMesher<B> {
    pub fn init<L: ClientChunkVertexLightingApi>(
        bevy: &mut BevyMod,
        _blocks: &mut B,
        _lighting_api: &mut L,
    ) -> Self {
        let lighting = bevy
            .app
            .world()
            .resource::<ChunkVertexLightingPipeline>()
            .clone();
        bevy.app
            .insert_resource(ChunkMeshService::new(move |neighborhood| {
                mesh_chunk_with_lighting::<B>(neighborhood, &lighting.snapshot())
            }));
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl<B: BlockManagerApi> ChunkMeshApi for NaiveCubeChunkMesher<B> {
    fn mesh_chunk(neighborhood: &ChunkMeshNeighborhood) -> ChunkMeshData {
        mesh_chunk_with_lighting::<B>(neighborhood, &ChunkVertexLightingSnapshot::default())
    }
}

fn mesh_chunk_with_lighting<B: BlockManagerApi>(
    neighborhood: &ChunkMeshNeighborhood,
    lighting: &ChunkVertexLightingSnapshot,
) -> ChunkMeshData {
    if uniform_chunk_is_fully_hidden::<B>(neighborhood) {
        return ChunkMeshData::default();
    }
    let mut mesh = ChunkMeshData::default();
    let chunk = neighborhood.center();
    for (local, block_instance) in chunk.iter() {
        let block = block_instance.block;
        let render = B::render_info(block);
        if B::is_air(block) || render.shape != RenderShape::Cube {
            continue;
        }

        for face in FACES {
            if !face_visible::<B>(neighborhood, local, face.neighbor) {
                continue;
            }
            let texture = render.textures.map(|textures| textures.texture(face.kind));
            let part = mesh_part(&mut mesh, texture);
            add_face::<B>(part, neighborhood, local, face, lighting);
        }
    }
    mesh
}

fn uniform_chunk_is_fully_hidden<B: BlockManagerApi>(neighborhood: &ChunkMeshNeighborhood) -> bool {
    let Some(center_block) = neighborhood.center().uniform_block() else {
        return false;
    };
    if B::is_air(center_block.block) {
        return true;
    }
    if !B::is_opaque(center_block.block) {
        return false;
    }
    let center = neighborhood.center().position();
    FACES.iter().all(|face| {
        let position = voxel_math_api::ChunkPos::new(
            center.x + face.neighbor[0],
            center.y + face.neighbor[1],
            center.z + face.neighbor[2],
        );
        neighborhood
            .chunk(position)
            .and_then(|chunk| chunk.uniform_block())
            .is_some_and(|block| B::is_opaque(block.block))
    })
}

fn mesh_part<'a>(
    mesh: &'a mut ChunkMeshData,
    texture: Option<&'static str>,
) -> &'a mut ChunkMeshPart {
    let index = mesh
        .parts
        .iter()
        .position(|part| part.texture == texture)
        .unwrap_or_else(|| {
            mesh.parts.push(ChunkMeshPart {
                texture,
                ..Default::default()
            });
            mesh.parts.len() - 1
        });
    &mut mesh.parts[index]
}

#[derive(Clone, Copy)]
struct Face {
    kind: BlockFace,
    neighbor: [i32; 3],
    normal: [f32; 3],
    vertices: [[f32; 3]; 4],
}

const FACES: [Face; 6] = [
    Face {
        kind: BlockFace::East,
        neighbor: [1, 0, 0],
        normal: [1.0, 0.0, 0.0],
        vertices: [
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [1.0, 1.0, 1.0],
            [1.0, 0.0, 1.0],
        ],
    },
    Face {
        kind: BlockFace::West,
        neighbor: [-1, 0, 0],
        normal: [-1.0, 0.0, 0.0],
        vertices: [
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0],
        ],
    },
    Face {
        kind: BlockFace::Top,
        neighbor: [0, 1, 0],
        normal: [0.0, 1.0, 0.0],
        vertices: [
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, 0.0],
        ],
    },
    Face {
        kind: BlockFace::Bottom,
        neighbor: [0, -1, 0],
        normal: [0.0, -1.0, 0.0],
        vertices: [
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 1.0],
        ],
    },
    Face {
        kind: BlockFace::South,
        neighbor: [0, 0, 1],
        normal: [0.0, 0.0, 1.0],
        vertices: [
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    },
    Face {
        kind: BlockFace::North,
        neighbor: [0, 0, -1],
        normal: [0.0, 0.0, -1.0],
        vertices: [
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
        ],
    },
];

fn face_visible<B: BlockManagerApi>(
    neighborhood: &ChunkMeshNeighborhood,
    local: LocalBlockPos,
    offset: [i32; 3],
) -> bool {
    let world = local.to_world(neighborhood.center().position());
    let neighbor = BlockPos::new(
        world.x + offset[0],
        world.y + offset[1],
        world.z + offset[2],
    );
    neighborhood
        .block(neighbor)
        .is_none_or(|block| !B::is_opaque(block.block))
}

fn add_face<B: BlockManagerApi>(
    mesh: &mut ChunkMeshPart,
    neighborhood: &ChunkMeshNeighborhood,
    local: LocalBlockPos,
    face: Face,
    lighting: &ChunkVertexLightingSnapshot,
) {
    let base = mesh.positions.len() as u32;
    let offset = [local.x as f32, local.y as f32, local.z as f32];
    let uvs = [[0.0, 1.0], [0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];
    let brightness = face.vertices.map(|vertex| {
        lighting.brightness(
            face.kind,
            vertex_occlusion::<B>(neighborhood, local, face, vertex),
        )
    });
    for ((vertex, uv), brightness) in face.vertices.into_iter().zip(uvs).zip(brightness) {
        mesh.positions.push([
            vertex[0] + offset[0],
            vertex[1] + offset[1],
            vertex[2] + offset[2],
        ]);
        mesh.normals.push(face.normal);
        mesh.colors.push([brightness, brightness, brightness, 1.0]);
        mesh.uvs.push(uv);
    }
    if brightness[0] + brightness[2] > brightness[1] + brightness[3] {
        mesh.indices
            .extend_from_slice(&[base, base + 1, base + 3, base + 1, base + 2, base + 3]);
    } else {
        mesh.indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
}

fn vertex_occlusion<B: BlockManagerApi>(
    neighborhood: &ChunkMeshNeighborhood,
    local: LocalBlockPos,
    face: Face,
    vertex: [f32; 3],
) -> VertexOcclusion {
    let world = local.to_world(neighborhood.center().position());
    let (axis_a, axis_b) = match face.kind {
        BlockFace::East | BlockFace::West => (1, 2),
        BlockFace::Top | BlockFace::Bottom => (0, 2),
        BlockFace::South | BlockFace::North => (0, 1),
    };
    let side_a = axis_offset(axis_a, vertex_sign(vertex[axis_a]));
    let side_b = axis_offset(axis_b, vertex_sign(vertex[axis_b]));
    let outside = face.neighbor;
    VertexOcclusion {
        side_a: opaque_at::<B>(
            neighborhood,
            offset_block(world, add_offsets(outside, side_a)),
        ),
        side_b: opaque_at::<B>(
            neighborhood,
            offset_block(world, add_offsets(outside, side_b)),
        ),
        corner: opaque_at::<B>(
            neighborhood,
            offset_block(world, add_offsets(add_offsets(outside, side_a), side_b)),
        ),
    }
}

fn vertex_sign(coordinate: f32) -> i32 {
    if coordinate > 0.5 { 1 } else { -1 }
}

fn axis_offset(axis: usize, amount: i32) -> [i32; 3] {
    let mut result = [0; 3];
    result[axis] = amount;
    result
}

fn add_offsets(left: [i32; 3], right: [i32; 3]) -> [i32; 3] {
    [left[0] + right[0], left[1] + right[1], left[2] + right[2]]
}

fn offset_block(block: BlockPos, offset: [i32; 3]) -> BlockPos {
    BlockPos::new(
        block.x + offset[0],
        block.y + offset[1],
        block.z + offset[2],
    )
}

fn opaque_at<B: BlockManagerApi>(neighborhood: &ChunkMeshNeighborhood, position: BlockPos) -> bool {
    neighborhood
        .block(position)
        .is_some_and(|block| B::is_opaque(block.block))
}

#[cfg(test)]
mod tests {
    use super::*;
    use block_api::BlockInfo;
    use block_manager_api::BlockId;
    use block_render_api::BlockRenderInfo;
    use chunk_api::Chunk;
    use client_chunk_mesh_api::ChunkMeshNeighborhood;
    use voxel_math_api::ChunkPos;

    struct TestBlocks;

    static AIR_INFO: BlockInfo = BlockInfo {
        id: "test:air",
        is_air: true,
        solid: false,
        opaque: false,
    };
    static STONE_INFO: BlockInfo = BlockInfo {
        id: "test:stone",
        is_air: false,
        solid: true,
        opaque: true,
    };
    static AIR_RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Invisible,
        textures: None,
    };
    static STONE_RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: None,
    };
    static ALL_BLOCKS: [BlockId; 2] = [BlockId::Air, BlockId::Stone];

    impl BlockManagerApi for TestBlocks {
        fn info(block: BlockId) -> &'static BlockInfo {
            match block {
                BlockId::Air => &AIR_INFO,
                _ => &STONE_INFO,
            }
        }

        fn render_info(block: BlockId) -> &'static BlockRenderInfo {
            match block {
                BlockId::Air => &AIR_RENDER,
                _ => &STONE_RENDER,
            }
        }

        fn all() -> &'static [BlockId] {
            &ALL_BLOCKS
        }

        fn from_string(id: &str) -> Option<BlockId> {
            match id {
                "test:air" => Some(BlockId::Air),
                "test:stone" => Some(BlockId::Stone),
                _ => None,
            }
        }

        fn id(block: BlockId) -> &'static str {
            match block {
                BlockId::Air => "test:air",
                _ => "test:stone",
            }
        }
    }

    #[test]
    fn chunk_boundary_face_depends_on_actual_neighbor() {
        let mut center = Chunk::filled(ChunkPos::new(0, 0, 0), BlockId::Air);
        center.set(LocalBlockPos::new(15, 1, 1).unwrap(), BlockId::Stone);

        let without_neighbor = ChunkMeshNeighborhood::new(center.clone(), []);
        let visible = NaiveCubeChunkMesher::<TestBlocks>::mesh_chunk(&without_neighbor);
        assert_eq!(visible.parts[0].indices.len(), 6 * 6);

        let mut neighbor = Chunk::filled(ChunkPos::new(1, 0, 0), BlockId::Air);
        neighbor.set(LocalBlockPos::new(0, 1, 1).unwrap(), BlockId::Stone);
        let with_opaque_neighbor = ChunkMeshNeighborhood::new(center, [neighbor]);
        let hidden = NaiveCubeChunkMesher::<TestBlocks>::mesh_chunk(&with_opaque_neighbor);
        assert_eq!(hidden.parts[0].indices.len(), 5 * 6);
    }

    #[test]
    fn fully_surrounded_uniform_opaque_chunk_needs_no_mesh() {
        let center = Chunk::filled(ChunkPos::new(0, 0, 0), BlockId::Stone);
        let neighbors = [
            ChunkPos::new(1, 0, 0),
            ChunkPos::new(-1, 0, 0),
            ChunkPos::new(0, 1, 0),
            ChunkPos::new(0, -1, 0),
            ChunkPos::new(0, 0, 1),
            ChunkPos::new(0, 0, -1),
        ]
        .map(|position| Chunk::filled(position, BlockId::Stone));
        let neighborhood = ChunkMeshNeighborhood::new(center, neighbors);
        let mesh = NaiveCubeChunkMesher::<TestBlocks>::mesh_chunk(&neighborhood);
        assert!(mesh.is_empty());
    }
}
