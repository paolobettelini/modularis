use bevy::{
    asset::RenderAssetUsages,
    camera::primitives::Aabb,
    mesh::{Indices, MeshVertexBufferLayoutRef},
    pbr::{Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{
        AsBindGroup, PrimitiveTopology, RenderPipelineDescriptor, ShaderType,
        SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
};
use bevy_mod::BevyMod;
use client_bevy_default_plugins_mod::ClientBevyDefaultPluginsMod;
use client_chunk_cache_api::{
    ClientChunkAvailable, ClientChunkCache, ClientChunkCacheApi, ClientChunkChanged,
};
use client_chunk_streaming_api::{ChunkStreamingApi, ChunkStreamingFocus, ChunkUnload};
use client_dimension_api::{ClientDimension, ClientDimensionApi};
use client_game_state_api::{GameState, GameStateApi};
use client_grass_interaction_api::{
    ClientGrassInteractionApi, ClientGrassInteractionField, GrassInteractionCollectSet,
};
use client_grass_mesh_api::{ClientGrassMeshApi, GrassMeshData, GrassMeshService};
use client_grass_render_api::{ClientGrassRenderApi, GrassChunkMeshRebuilt};
use client_grass_settings_api::{
    ClientGrassSettings, ClientGrassSettingsApi, ClientGrassSettingsChanged,
};
use client_wind_api::{ClientWind, ClientWindApi};
use std::collections::{HashMap, HashSet};
use tokio::task::JoinHandle;
use voxel_math_api::{CHUNK_SIZE, ChunkPos};

const GRASS_SHADER: &str = "client-grass-render-bevy-impl/shaders/grass.wgsl";
const CHUNKS_PER_FRAME: usize = 2;
const MAX_VERTICAL_CHUNK_DISTANCE: i32 = 4;
const MAX_GRASS_INTERACTIONS: usize = 8;

#[derive(Clone, Copy, Debug, ShaderType)]
struct GrassMaterialUniform {
    wind: Vec4,
    appearance: Vec4,
    base_color: Vec4,
    interaction_header: Vec4,
    interaction_positions: [Vec4; MAX_GRASS_INTERACTIONS],
    interaction_axes: [Vec4; MAX_GRASS_INTERACTIONS],
    interaction_parameters: [Vec4; MAX_GRASS_INTERACTIONS],
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct GrassMaterial {
    #[uniform(0)]
    values: GrassMaterialUniform,
}

impl Material for GrassMaterial {
    fn vertex_shader() -> ShaderRef {
        GRASS_SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        GRASS_SHADER.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        descriptor.vertex.buffers = vec![layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(5),
        ])?];
        Ok(())
    }
}

#[derive(Resource, Default)]
struct GrassMaterialHandle(Option<Handle<GrassMaterial>>);

#[derive(Resource, Default)]
struct GrassChunkState {
    known: HashSet<ChunkPos>,
    pending: HashSet<ChunkPos>,
    entities: HashMap<ChunkPos, Entity>,
}

pub struct ClientGrassRenderBevyImpl;

impl ClientGrassRenderBevyImpl {
    #[allow(clippy::too_many_arguments)]
    pub fn init<
        C: ClientChunkCacheApi,
        S: ChunkStreamingApi,
        G: GameStateApi,
        D: ClientDimensionApi,
        M: ClientGrassMeshApi,
        I: ClientGrassInteractionApi,
        R: ClientGrassSettingsApi,
        W: ClientWindApi,
    >(
        bevy: &mut BevyMod,
        _plugins: &mut ClientBevyDefaultPluginsMod,
        _cache: &mut C,
        _streaming: &mut S,
        _game_state: &mut G,
        _dimension: &mut D,
        _mesher: &mut M,
        _interactions: &mut I,
        _settings: &mut R,
        _wind: &mut W,
    ) -> Self {
        bevy.app
            .add_plugins(MaterialPlugin::<GrassMaterial> {
                // The grass shader uses a compact custom vertex interface. Bevy's
                // standard prepass/shadow shaders expect the standard mesh
                // locations and cannot be specialized with this layout.
                prepass_enabled: false,
                shadows_enabled: false,
                ..default()
            })
            .init_resource::<GrassMaterialHandle>()
            .init_resource::<GrassChunkState>()
            .add_message::<GrassChunkMeshRebuilt>()
            .add_systems(Startup, create_grass_material)
            .add_systems(
                Update,
                (
                    update_grass_material.after(GrassInteractionCollectSet),
                    collect_grass_chunk_work,
                    queue_grass_configuration_changes,
                    process_grass_chunk_work,
                    unload_grass_chunks,
                )
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), clear_grass_state);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientGrassRenderApi for ClientGrassRenderBevyImpl {}

fn create_grass_material(
    mut handle: ResMut<GrassMaterialHandle>,
    settings: Res<ClientGrassSettings>,
    wind: Res<ClientWind>,
    focus: Res<ChunkStreamingFocus>,
    interactions: Res<ClientGrassInteractionField>,
    mut materials: ResMut<Assets<GrassMaterial>>,
) {
    handle.0 = Some(materials.add(material(*settings, *wind, &focus, &interactions)));
}

fn update_grass_material(
    handle: Res<GrassMaterialHandle>,
    settings: Res<ClientGrassSettings>,
    wind: Res<ClientWind>,
    focus: Res<ChunkStreamingFocus>,
    interactions: Res<ClientGrassInteractionField>,
    mut materials: ResMut<Assets<GrassMaterial>>,
) {
    let Some(handle) = &handle.0 else {
        return;
    };
    let Some(current) = materials.get_mut(handle) else {
        return;
    };
    current.values = material(*settings, *wind, &focus, &interactions).values;
}

fn material(
    settings: ClientGrassSettings,
    wind: ClientWind,
    focus: &ChunkStreamingFocus,
    interactions: &ClientGrassInteractionField,
) -> GrassMaterial {
    let mut interaction_positions = [Vec4::ZERO; MAX_GRASS_INTERACTIONS];
    let mut interaction_axes = [Vec4::ZERO; MAX_GRASS_INTERACTIONS];
    let mut interaction_parameters = [Vec4::ZERO; MAX_GRASS_INTERACTIONS];
    let mut interaction_count = 0;
    let interaction_origin = focus
        .center
        .map(|position| {
            let origin = position.world_origin();
            Vec3::new(origin.x as f32, origin.y as f32, origin.z as f32)
                + Vec3::splat(CHUNK_SIZE as f32 * 0.5)
        })
        .unwrap_or(Vec3::ZERO);
    for (_, source) in interactions.sources_nearest_to(interaction_origin, MAX_GRASS_INTERACTIONS) {
        interaction_positions[interaction_count] = source.position.extend(source.radius);
        interaction_axes[interaction_count] =
            source.axis.normalize_or_zero().extend(source.half_length);
        interaction_parameters[interaction_count] = Vec4::new(source.strength, 0.0, 0.0, 0.0);
        interaction_count += 1;
    }
    let deformation = settings.deformation_strength;
    GrassMaterial {
        values: GrassMaterialUniform {
            wind: Vec4::new(wind.direction.x, wind.direction.y, wind.intensity, 0.0),
            appearance: Vec4::new(settings.brightness, 0.76, 1.08, 1.9),
            base_color: Vec4::new(1.0, 1.0, 1.0, settings.hue_jitter_degrees.to_radians()),
            interaction_header: Vec4::new(
                interaction_count as f32,
                0.52 * deformation,
                0.12 * deformation,
                0.0,
            ),
            interaction_positions,
            interaction_axes,
            interaction_parameters,
        },
    }
}

fn collect_grass_chunk_work(
    mut available: MessageReader<ClientChunkAvailable>,
    mut changed: MessageReader<ClientChunkChanged>,
    mut state: ResMut<GrassChunkState>,
) {
    for event in available.read() {
        state.known.insert(event.position);
        state.pending.insert(event.position);
    }
    for event in changed.read() {
        state.known.insert(event.position);
        state.pending.insert(event.position);
    }
}

fn queue_grass_configuration_changes(
    focus: Res<ChunkStreamingFocus>,
    mut changes: MessageReader<ClientGrassSettingsChanged>,
    mut state: ResMut<GrassChunkState>,
) {
    let rebuild = focus.is_changed() || changes.read().any(|change| change.geometry_changed);
    if rebuild {
        let known = state.known.iter().copied().collect::<Vec<_>>();
        state.pending.extend(known);
    }
}

#[allow(clippy::too_many_arguments)]
fn process_grass_chunk_work(
    mut commands: Commands,
    cache: Res<ClientChunkCache>,
    focus: Res<ChunkStreamingFocus>,
    settings: Res<ClientGrassSettings>,
    dimension: Res<ClientDimension>,
    mesher: Res<GrassMeshService>,
    handle: Res<GrassMaterialHandle>,
    mut state: ResMut<GrassChunkState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut rebuilt: MessageWriter<GrassChunkMeshRebuilt>,
) {
    if !settings.enabled {
        for entity in state.entities.drain().map(|(_, entity)| entity) {
            commands.entity(entity).try_despawn();
        }
        state.pending.clear();
        return;
    }
    let (Some(focus), Some(material)) = (focus.center, handle.0.as_ref()) else {
        return;
    };

    let mut pending = state.pending.iter().copied().collect::<Vec<_>>();
    pending.sort_unstable_by_key(|chunk| chunk_distance_squared(*chunk, focus));
    pending.truncate(CHUNKS_PER_FRAME);

    for position in pending {
        state.pending.remove(&position);
        if !in_render_range(position, focus, settings.render_radius) {
            despawn_grass_chunk(&mut commands, &mut state, position);
            continue;
        }
        let Some(chunk) = cache.chunk(position) else {
            // ChunkUnload is responsible for removing an existing mesh. Keeping
            // it here avoids a visual hole during a transient cache/work-order
            // race.
            continue;
        };
        let distance = horizontal_chunk_distance(position, focus);
        let mesh_data = (mesher.mesh_chunk)(&chunk, *settings, distance, dimension.0);
        if mesh_data.is_empty() {
            despawn_grass_chunk(&mut commands, &mut state, position);
            continue;
        }
        let blade_count = mesh_data.blade_count;

        let origin = position.world_origin();
        let replacement = commands
            .spawn((
                Mesh3d(meshes.add(bevy_mesh(mesh_data))),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(origin.x as f32, origin.y as f32, origin.z as f32),
                grass_chunk_bounds(),
                DespawnOnExit(GameState::InGame),
            ))
            .id();
        if let Some(previous) = state.entities.insert(position, replacement) {
            commands.entity(previous).try_despawn();
        }
        rebuilt.write(GrassChunkMeshRebuilt {
            chunk: position,
            blade_count,
        });
    }
}

fn unload_grass_chunks(
    mut commands: Commands,
    mut unloads: MessageReader<ChunkUnload>,
    mut state: ResMut<GrassChunkState>,
) {
    for unload in unloads.read() {
        state.known.remove(&unload.position);
        state.pending.remove(&unload.position);
        if let Some(entity) = state.entities.remove(&unload.position) {
            commands.entity(entity).try_despawn();
        }
    }
}

fn clear_grass_state(mut state: ResMut<GrassChunkState>) {
    state.known.clear();
    state.pending.clear();
    state.entities.clear();
}

fn despawn_grass_chunk(commands: &mut Commands, state: &mut GrassChunkState, position: ChunkPos) {
    if let Some(entity) = state.entities.remove(&position) {
        commands.entity(entity).try_despawn();
    }
}

fn in_render_range(position: ChunkPos, focus: ChunkPos, render_radius: f32) -> bool {
    (position.y - focus.y).abs() <= MAX_VERTICAL_CHUNK_DISTANCE
        && horizontal_chunk_distance(position, focus) <= render_radius
}

fn horizontal_chunk_distance(position: ChunkPos, focus: ChunkPos) -> f32 {
    let dx = (position.x - focus.x) as f32 * CHUNK_SIZE as f32;
    let dz = (position.z - focus.z) as f32 * CHUNK_SIZE as f32;
    (dx * dx + dz * dz).sqrt()
}

fn chunk_distance_squared(position: ChunkPos, focus: ChunkPos) -> i64 {
    let dx = (position.x - focus.x) as i64;
    let dy = (position.y - focus.y) as i64;
    let dz = (position.z - focus.z) as i64;
    dx * dx + dy * dy + dz * dz
}

fn bevy_mesh(data: GrassMeshData) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_indices(Indices::U32(data.indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, data.positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, data.colors)
}

fn grass_chunk_bounds() -> Aabb {
    // Vertex-shader wind can move tips outside the CPU-authored mesh bounds.
    // Keep culling conservative without disabling it for every grass chunk.
    Aabb::from_min_max(
        Vec3::new(-1.0, -0.1, -1.0),
        Vec3::splat(CHUNK_SIZE as f32) + Vec3::new(1.0, 1.6, 1.0),
    )
}
