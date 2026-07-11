use bevy::{
    asset::RenderAssetUsages, mesh::Indices, prelude::*, render::render_resource::PrimitiveTopology,
};
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use client_chunk_cache_api::{
    ClientChunkAvailable, ClientChunkCache, ClientChunkCacheApi, ClientChunkChanged,
};
use client_chunk_mesh_api::{ChunkMeshApi, ChunkMeshNeighborhood, ChunkMeshService};
use client_chunk_render_api::{ChunkRenderApi, RenderedChunks};
use client_chunk_streaming_api::{ChunkStreamingApi, ChunkUnload};
use client_game_state_api::{GameState, GameStateApi};
use std::collections::{HashMap, HashSet};
use tokio::task::JoinHandle;

#[derive(Resource, Default)]
struct ChunkMaterials(HashMap<String, Handle<StandardMaterial>>);

pub struct ChunkRenderBevyImpl;

impl ChunkRenderBevyImpl {
    pub fn init<
        C: ClientChunkCacheApi,
        S: ChunkStreamingApi,
        M: ChunkMeshApi,
        B: BlockManagerApi,
        G: GameStateApi,
    >(
        bevy: &mut BevyMod,
        _cache: &mut C,
        _streaming: &mut S,
        _mesher: &mut M,
        _blocks: &mut B,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .init_resource::<RenderedChunks>()
            .init_resource::<ChunkMaterials>()
            .add_systems(
                Update,
                (render_needed_chunks, unload_chunks)
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), cleanup_rendered_chunks);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ChunkRenderApi for ChunkRenderBevyImpl {}

fn render_needed_chunks(
    mut commands: Commands,
    mut available: MessageReader<ClientChunkAvailable>,
    mut changed: MessageReader<ClientChunkChanged>,
    cache: Res<ClientChunkCache>,
    mesher: Res<ChunkMeshService>,
    mut rendered: ResMut<RenderedChunks>,
    mut chunk_materials: ResMut<ChunkMaterials>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut requests = HashSet::new();
    for request in available.read() {
        add_chunk_and_neighbors(&mut requests, request.position);
    }
    for request in changed.read() {
        add_chunk_and_neighbors(&mut requests, request.position);
    }

    for position in requests {
        despawn_chunk(&mut commands, &mut rendered, position);

        let Some(chunk) = cache.chunk(position) else {
            continue;
        };
        let neighborhood = ChunkMeshNeighborhood::new(
            chunk,
            neighboring_chunk_positions(position)
                .into_iter()
                .filter_map(|neighbor| cache.chunk(neighbor)),
        );
        let mesh_data = (mesher.mesh_chunk)(&neighborhood);
        if mesh_data.is_empty() {
            continue;
        }

        let origin = position.world_origin();
        let transform = Transform::from_xyz(origin.x as f32, origin.y as f32, origin.z as f32);
        let mut entities = Vec::new();
        for part in mesh_data.parts {
            if part.is_empty() {
                continue;
            }

            let material_key = material_key(part.texture);
            let material = chunk_materials
                .0
                .entry(material_key)
                .or_insert_with(|| {
                    materials.add(StandardMaterial {
                        base_color: Color::WHITE,
                        base_color_texture: part.texture.map(|path| asset_server.load(path)),
                        perceptual_roughness: 1.0,
                        cull_mode: None,
                        ..default()
                    })
                })
                .clone();

            entities.push(
                commands
                    .spawn((
                        Mesh3d(meshes.add(build_bevy_mesh(part))),
                        MeshMaterial3d(material),
                        transform,
                        DespawnOnExit(GameState::InGame),
                    ))
                    .id(),
            );
        }
        if !entities.is_empty() {
            rendered.entities.insert(position, entities);
        }
    }
}

fn add_chunk_and_neighbors(
    requests: &mut HashSet<voxel_math_api::ChunkPos>,
    position: voxel_math_api::ChunkPos,
) {
    requests.insert(position);
    requests.extend(neighboring_chunk_positions(position));
}

fn neighboring_chunk_positions(
    position: voxel_math_api::ChunkPos,
) -> Vec<voxel_math_api::ChunkPos> {
    let mut neighbors = Vec::with_capacity(26);
    for y in -1..=1 {
        for z in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                neighbors.push(voxel_math_api::ChunkPos::new(
                    position.x + x,
                    position.y + y,
                    position.z + z,
                ));
            }
        }
    }
    neighbors
}

fn unload_chunks(
    mut commands: Commands,
    mut unloads: MessageReader<ChunkUnload>,
    mut rendered: ResMut<RenderedChunks>,
) {
    for unload in unloads.read() {
        despawn_chunk(&mut commands, &mut rendered, unload.position);
    }
}

fn despawn_chunk(
    commands: &mut Commands,
    rendered: &mut RenderedChunks,
    position: voxel_math_api::ChunkPos,
) {
    if let Some(entities) = rendered.entities.remove(&position) {
        for entity in entities {
            commands.entity(entity).try_despawn();
        }
    }
}

fn cleanup_rendered_chunks(mut rendered: ResMut<RenderedChunks>) {
    rendered.entities.clear();
}

fn material_key(texture: Option<&str>) -> String {
    match texture {
        Some(texture) => format!("texture:{texture}"),
        None => "untextured:white".to_string(),
    }
}

fn build_bevy_mesh(data: client_chunk_mesh_api::ChunkMeshPart) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_indices(Indices::U32(data.indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, data.positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, data.colors)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs)
}
