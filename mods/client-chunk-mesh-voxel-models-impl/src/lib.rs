use bevy_mod::BevyMod;
use block_manager_api::{BlockId, BlockManagerApi};
use block_render_api::BlockFace;
use client_chunk_mesh_api::{
    ChunkMeshApi, ChunkMeshData, ChunkMeshNeighborhood, ChunkMeshPart, ChunkMeshService,
};
use client_chunk_vertex_lighting_api::{
    ChunkVertexLightingPipeline, ChunkVertexLightingSnapshot, ClientChunkVertexLightingApi,
    VertexOcclusion,
};
use std::{collections::HashMap, marker::PhantomData, sync::Arc};
use tokio::task::JoinHandle;
use voxel_math_api::{BlockPos, LocalBlockPos};
use voxel_model_api::{SharedBakedModel, VoxelModelApi, VoxelModelService};
use voxel_models_lib::{BakedQuad, Direction};

pub struct VoxelModelChunkMesher<B>(PhantomData<B>);

impl<B: BlockManagerApi> VoxelModelChunkMesher<B> {
    pub fn init<L: ClientChunkVertexLightingApi, M: VoxelModelApi>(
        bevy: &mut BevyMod,
        _blocks: &mut B,
        _lighting_api: &mut L,
        _models_api: &mut M,
    ) -> Self {
        let model_service = bevy.app.world().resource::<VoxelModelService>().clone();
        let baked_models = Arc::new(load_block_models::<B>(&model_service));
        let lighting = bevy
            .app
            .world()
            .resource::<ChunkVertexLightingPipeline>()
            .clone();
        bevy.app
            .insert_resource(ChunkMeshService::new(move |neighborhood| {
                mesh_chunk::<B>(neighborhood, baked_models.as_ref(), &lighting.snapshot())
            }));
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl<B: BlockManagerApi> ChunkMeshApi for VoxelModelChunkMesher<B> {}

fn load_block_models<B: BlockManagerApi>(
    service: &VoxelModelService,
) -> HashMap<BlockId, SharedBakedModel> {
    let mut models = HashMap::new();
    for block in B::all() {
        let Some(model_id) = B::render_info(*block).model else {
            continue;
        };
        match service.bake(model_id) {
            Ok(model) => {
                models.insert(*block, model);
            }
            Err(error) => {
                eprintln!("failed to load block model '{model_id}': {error}");
            }
        }
    }
    models
}

fn mesh_chunk<B: BlockManagerApi>(
    neighborhood: &ChunkMeshNeighborhood,
    models: &HashMap<BlockId, SharedBakedModel>,
    lighting: &ChunkVertexLightingSnapshot,
) -> ChunkMeshData {
    if uniform_chunk_is_fully_hidden::<B>(neighborhood) {
        return ChunkMeshData::default();
    }

    let mut mesh = ChunkMeshData::default();
    for (local, instance) in neighborhood.center().iter() {
        if B::is_air(instance.block) {
            continue;
        }
        let Some(quads) = models.get(&instance.block) else {
            continue;
        };
        for quad in quads.iter() {
            if !quad_visible::<B>(neighborhood, local, quad) {
                continue;
            }
            let texture = VoxelModelService::texture_asset_path(&quad.texture);
            let part = mesh_part(&mut mesh, texture);
            add_quad::<B>(part, neighborhood, local, quad, lighting);
        }
    }
    mesh
}

fn mesh_part<'a>(mesh: &'a mut ChunkMeshData, texture: String) -> &'a mut ChunkMeshPart {
    let index = mesh
        .parts
        .iter()
        .position(|part| part.texture.as_deref() == Some(texture.as_str()))
        .unwrap_or_else(|| {
            mesh.parts.push(ChunkMeshPart {
                texture: Some(texture),
                ..Default::default()
            });
            mesh.parts.len() - 1
        });
    &mut mesh.parts[index]
}

fn quad_visible<B: BlockManagerApi>(
    neighborhood: &ChunkMeshNeighborhood,
    local: LocalBlockPos,
    quad: &BakedQuad,
) -> bool {
    let Some(direction) = quad.cull_face else {
        return true;
    };
    let world = local.to_world(neighborhood.center().position());
    let offset = direction_offset(direction);
    let neighbor = BlockPos::new(
        world.x + offset[0],
        world.y + offset[1],
        world.z + offset[2],
    );
    neighborhood
        .block(neighbor)
        .is_none_or(|block| !B::is_opaque(block.block))
}

fn add_quad<B: BlockManagerApi>(
    mesh: &mut ChunkMeshPart,
    neighborhood: &ChunkMeshNeighborhood,
    local: LocalBlockPos,
    quad: &BakedQuad,
    lighting: &ChunkVertexLightingSnapshot,
) {
    let base = mesh.positions.len() as u32;
    let offset = [local.x as f32, local.y as f32, local.z as f32];
    let face = direction_from_normal(quad.normal);
    let brightness = quad.positions.map(|vertex| {
        let mut value = if quad.shade {
            lighting.brightness(
                face,
                vertex_occlusion::<B>(neighborhood, local, face, vertex),
            )
        } else {
            1.0
        };
        if let Some(emission) = quad.light_emission {
            value = value.max(emission as f32 / 15.0);
        }
        value
    });

    for ((position, uv), brightness) in quad.positions.iter().zip(quad.uvs).zip(brightness) {
        mesh.positions.push([
            position[0] + offset[0],
            position[1] + offset[1],
            position[2] + offset[2],
        ]);
        mesh.normals.push(quad.normal);
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
    face: BlockFace,
    vertex: [f32; 3],
) -> VertexOcclusion {
    let world = local.to_world(neighborhood.center().position());
    let outside = face_offset(face);
    let (axis_a, axis_b) = match face {
        BlockFace::East | BlockFace::West => (1, 2),
        BlockFace::Top | BlockFace::Bottom => (0, 2),
        BlockFace::South | BlockFace::North => (0, 1),
    };
    let side_a = axis_offset(axis_a, vertex_sign(vertex[axis_a]));
    let side_b = axis_offset(axis_b, vertex_sign(vertex[axis_b]));
    VertexOcclusion {
        side_a: opaque_at::<B>(neighborhood, offset_block(world, add(outside, side_a))),
        side_b: opaque_at::<B>(neighborhood, offset_block(world, add(outside, side_b))),
        corner: opaque_at::<B>(
            neighborhood,
            offset_block(world, add(add(outside, side_a), side_b)),
        ),
    }
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
    Direction::ALL.into_iter().all(|direction| {
        let offset = direction_offset(direction);
        let position = voxel_math_api::ChunkPos::new(
            center.x + offset[0],
            center.y + offset[1],
            center.z + offset[2],
        );
        neighborhood
            .chunk(position)
            .and_then(|chunk| chunk.uniform_block())
            .is_some_and(|block| B::is_opaque(block.block))
    })
}

fn direction_from_normal(normal: [f32; 3]) -> BlockFace {
    let [x, y, z] = normal;
    if x.abs() >= y.abs() && x.abs() >= z.abs() {
        if x >= 0.0 {
            BlockFace::East
        } else {
            BlockFace::West
        }
    } else if y.abs() >= z.abs() {
        if y >= 0.0 {
            BlockFace::Top
        } else {
            BlockFace::Bottom
        }
    } else if z >= 0.0 {
        BlockFace::South
    } else {
        BlockFace::North
    }
}

fn direction_offset(direction: Direction) -> [i32; 3] {
    match direction {
        Direction::Down => [0, -1, 0],
        Direction::Up => [0, 1, 0],
        Direction::North => [0, 0, -1],
        Direction::South => [0, 0, 1],
        Direction::West => [-1, 0, 0],
        Direction::East => [1, 0, 0],
    }
}

fn face_offset(face: BlockFace) -> [i32; 3] {
    match face {
        BlockFace::East => [1, 0, 0],
        BlockFace::West => [-1, 0, 0],
        BlockFace::Top => [0, 1, 0],
        BlockFace::Bottom => [0, -1, 0],
        BlockFace::South => [0, 0, 1],
        BlockFace::North => [0, 0, -1],
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

fn add(left: [i32; 3], right: [i32; 3]) -> [i32; 3] {
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
