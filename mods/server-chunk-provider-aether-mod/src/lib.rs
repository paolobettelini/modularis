use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use chunk_api::Chunk;
use coherent_noise_api::PerlinNoise2d;
use generated_block_registry::BlockId;
use server_biome_api::{BiomeTerrain, Dimension, ServerBiomeApi, ServerBiomeRegistry};
use server_biome_sampling_api::ServerBiomeSampler;
use server_biome_selection_api::{ServerBiomeSelectionApi, ServerBiomeSelectorResource};
use server_chunk_provider_api::{
    ChunkGenerationRequest, ChunkProviderId, ServerChunkProvider, ServerChunkProviderRegistry,
};
use server_chunk_provider_registry_mod::ServerChunkProviderRegistryMod;
use server_world_seed_api::{ServerWorldSeed, ServerWorldSeedApi};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, OnceLock};
use tokio::task::JoinHandle;
use voxel_math_api::{CHUNK_SIZE, LocalBlockPos};

pub const AETHER_PROVIDER_ID: &str = "demo:aether-islands";
const AETHER_SEED_NAMESPACE: &str = "demo:aether-terrain";
const FEATURE_HORIZONTAL_MARGIN: i32 = 2;

pub struct ServerChunkProviderAetherMod;

impl ServerChunkProviderAetherMod {
    pub fn init<
        B: BlockManagerApi,
        A: ServerBiomeApi,
        S: ServerBiomeSelectionApi,
        W: ServerWorldSeedApi,
    >(
        bevy: &mut BevyMod,
        _registry: &mut ServerChunkProviderRegistryMod,
        _blocks: &mut B,
        _biomes: &mut A,
        _selection: &mut S,
        _world_seed: &mut W,
    ) -> Self {
        let biomes = bevy.app.world().resource::<ServerBiomeRegistry>().clone();
        let selector = bevy
            .app
            .world()
            .resource::<ServerBiomeSelectorResource>()
            .clone();
        let world_seed = *bevy.app.world().resource::<ServerWorldSeed>();
        bevy.app
            .world()
            .resource::<ServerChunkProviderRegistry>()
            .register(
                ChunkProviderId::new(AETHER_PROVIDER_ID),
                AetherIslandProvider {
                    biomes,
                    selector,
                    world_seed,
                    validated: Arc::new(OnceLock::new()),
                },
            )
            .expect("the Aether chunk provider id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct AetherIslandProvider {
    biomes: ServerBiomeRegistry,
    selector: ServerBiomeSelectorResource,
    world_seed: ServerWorldSeed,
    validated: Arc<OnceLock<()>>,
}

impl ServerChunkProvider for AetherIslandProvider {
    fn generate(&self, request: &ChunkGenerationRequest) -> Option<Chunk> {
        self.validated
            .get_or_init(|| validate_registry(&self.biomes));
        let biomes =
            ServerBiomeSampler::new(request, &self.biomes, &self.selector, Dimension::Aether);
        assert!(
            !biomes.is_empty(),
            "the Aether provider requires at least one Aether biome"
        );
        Some(
            AetherWorldSampler::new(
                request,
                biomes,
                &self.biomes,
                self.world_seed
                    .derive(AETHER_SEED_NAMESPACE, &request.instance),
            )
            .build_chunk(),
        )
    }
}

fn validate_registry(registry: &ServerBiomeRegistry) {
    let missing = registry.missing_features();
    assert!(
        missing.is_empty(),
        "biome definitions reference unregistered features: {}",
        missing
            .iter()
            .map(|missing| format!("{:?} -> {}", missing.biome, missing.feature))
            .collect::<Vec<_>>()
            .join(", ")
    );
}

#[derive(Clone, Copy)]
struct IslandColumn {
    terrain: BiomeTerrain,
    surface: i32,
    thickness: i32,
}

struct AetherWorldSampler<'a> {
    request: &'a ChunkGenerationRequest,
    biomes: ServerBiomeSampler<'a>,
    registry: &'a ServerBiomeRegistry,
    world_seed: u64,
    island_cache: RefCell<HashMap<(i32, i32), Option<IslandColumn>>>,
}

impl<'a> AetherWorldSampler<'a> {
    fn new(
        request: &'a ChunkGenerationRequest,
        biomes: ServerBiomeSampler<'a>,
        registry: &'a ServerBiomeRegistry,
        world_seed: u64,
    ) -> Self {
        Self {
            request,
            biomes,
            registry,
            world_seed,
            island_cache: RefCell::new(HashMap::new()),
        }
    }

    fn build_chunk(&self) -> Chunk {
        let position = self.request.position;
        let origin = position.world_origin();
        let mut columns = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                columns.push(self.island_column(origin.x + x, origin.z + z));
            }
        }

        let mut active_biomes = BTreeSet::new();
        let mut minimum_surface = i32::MAX;
        let mut maximum_surface = i32::MIN;
        let mut minimum_bottom = i32::MAX;
        for z in -FEATURE_HORIZONTAL_MARGIN..(CHUNK_SIZE + FEATURE_HORIZONTAL_MARGIN) {
            for x in -FEATURE_HORIZONTAL_MARGIN..(CHUNK_SIZE + FEATURE_HORIZONTAL_MARGIN) {
                let world_x = origin.x + x;
                let world_z = origin.z + z;
                let Some(column) = self.island_column(world_x, world_z) else {
                    continue;
                };
                if let Some(biome) = self.biomes.biome_at(world_x, world_z) {
                    active_biomes.insert(biome);
                }
                minimum_surface = minimum_surface.min(column.surface);
                maximum_surface = maximum_surface.max(column.surface);
                minimum_bottom = minimum_bottom.min(column.surface - column.thickness);
            }
        }
        if active_biomes.is_empty() {
            return Chunk::filled(position, BlockId::Air);
        }

        let chunk_bottom = origin.y;
        let chunk_top = origin.y + CHUNK_SIZE - 1;
        let features_intersect = self.biomes.features_intersect(
            self.registry,
            &active_biomes,
            chunk_bottom,
            chunk_top,
            minimum_surface,
            maximum_surface,
        );
        if (chunk_bottom > maximum_surface || chunk_top < minimum_bottom) && !features_intersect {
            return Chunk::filled(position, BlockId::Air);
        }

        let mut chunk = Chunk::filled(position, BlockId::Air);
        for local_y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let Some(column) = columns[(x + z * CHUNK_SIZE) as usize] else {
                        continue;
                    };
                    let local = LocalBlockPos::new(x, local_y, z).unwrap();
                    let depth = column.surface - (origin.y + local_y);
                    let block = match depth {
                        i32::MIN..=-1 => BlockId::Air,
                        0 => column.terrain.surface,
                        depth if depth <= column.terrain.subsurface_depth as i32 => {
                            column.terrain.subsurface
                        }
                        depth if depth <= column.thickness => column.terrain.underground,
                        _ => BlockId::Air,
                    };
                    chunk.set(local, block);
                }
            }
        }

        let surface_height_at = |x, z| {
            self.island_column(x, z)
                .map(|column| column.surface)
                .unwrap_or(0)
        };
        let biome_at = |x, z| {
            self.island_column(x, z)?;
            self.biomes.biome_at(x, z)
        };
        self.biomes.apply_features(
            self.registry,
            &mut chunk,
            self.world_seed,
            &active_biomes,
            minimum_surface,
            maximum_surface,
            &surface_height_at,
            &biome_at,
        );
        chunk
    }

    fn island_column(&self, x: i32, z: i32) -> Option<IslandColumn> {
        if let Some(column) = self.island_cache.borrow().get(&(x, z)).copied() {
            return column;
        }
        let terrain = self.biomes.terrain_at(x, z)?;
        let distance = ((x as f32).powi(2) + (z as f32).powi(2)).sqrt();
        let column = if distance <= 8.0 {
            Some(IslandColumn {
                terrain,
                surface: 9,
                thickness: 4.max(terrain.subsurface_depth as i32 + 1),
            })
        } else {
            let continents = PerlinNoise2d::new(self.world_seed as u32 ^ 0x4145_5448)
                .sample(x as f32 * 0.026, z as f32 * 0.026);
            let breakup = PerlinNoise2d::new(self.world_seed as u32 ^ 0xdf72_2cf1)
                .sample(x as f32 * 0.071 + 31.0, z as f32 * 0.071 - 17.0);
            if continents + breakup * 0.36 < 0.08 {
                None
            } else {
                const SAMPLES: &[(i32, f32)] = &[(-8, 1.0), (0, 2.0), (8, 1.0)];
                let (base, variation, detail_variation) = self
                    .biomes
                    .blended_terrain_parameters(x, z, SAMPLES)
                    .unwrap_or((9.0, 3.0, 1.0));
                let height = PerlinNoise2d::new(self.world_seed as u32 ^ 0x0900_1c0f)
                    .sample(x as f32 * 0.047 - 8.0, z as f32 * 0.047 + 12.0);
                let detail = PerlinNoise2d::new(self.world_seed as u32 ^ 0x150d_1d0b)
                    .sample(x as f32 * 0.083 + 19.0, z as f32 * 0.083 + 3.0);
                Some(IslandColumn {
                    terrain,
                    surface: (base + height * variation + detail * detail_variation).round() as i32,
                    thickness: (terrain.subsurface_depth as i32
                        + 2
                        + (detail.abs() * 3.0).round() as i32)
                        .max(3),
                })
            }
        };
        self.island_cache.borrow_mut().insert((x, z), column);
        column
    }
}
