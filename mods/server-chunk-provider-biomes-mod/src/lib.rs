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
use server_primary_chunk_provider_api::ServerPrimaryChunkProviderApi;
use server_world_seed_api::{ServerWorldSeed, ServerWorldSeedApi};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, OnceLock};
use tokio::task::JoinHandle;
use voxel_math_api::{CHUNK_SIZE, LocalBlockPos};

const TERRAIN_SEED_NAMESPACE: &str = "demo:overworld-terrain";
const FEATURE_HORIZONTAL_MARGIN: i32 = 2;

pub struct ServerChunkProviderBiomesMod;

impl ServerChunkProviderBiomesMod {
    pub fn init<
        B: BlockManagerApi,
        A: ServerBiomeApi,
        S: ServerBiomeSelectionApi,
        W: ServerWorldSeedApi,
    >(
        bevy: &mut BevyMod,
        _provider_registry: &mut ServerChunkProviderRegistryMod,
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
        let world_seed = bevy.app.world().resource::<ServerWorldSeed>().clone();
        bevy.app
            .world()
            .resource::<ServerChunkProviderRegistry>()
            .register(
                ChunkProviderId::primary(),
                BiomeTerrainProvider {
                    biomes,
                    selector,
                    world_seed,
                    validated: Arc::new(OnceLock::new()),
                },
            )
            .expect("the primary server chunk provider must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPrimaryChunkProviderApi for ServerChunkProviderBiomesMod {}

struct BiomeTerrainProvider {
    biomes: ServerBiomeRegistry,
    selector: ServerBiomeSelectorResource,
    world_seed: ServerWorldSeed,
    validated: Arc<OnceLock<()>>,
}

impl ServerChunkProvider for BiomeTerrainProvider {
    fn generate(&self, request: &ChunkGenerationRequest) -> Option<Chunk> {
        self.validated.get_or_init(|| {
            let missing = self.biomes.missing_features();
            assert!(
                missing.is_empty(),
                "biome definitions reference unregistered features: {}",
                missing
                    .iter()
                    .map(|missing| format!("{:?} -> {}", missing.biome, missing.feature))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        });
        let biomes =
            ServerBiomeSampler::new(request, &self.biomes, &self.selector, Dimension::Overworld);
        if biomes.is_empty() {
            return None;
        }
        Some(
            WorldSampler::new(
                request,
                biomes,
                &self.biomes,
                self.world_seed
                    .derive(TERRAIN_SEED_NAMESPACE, &request.instance),
            )
            .build_chunk(),
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct ColumnSample {
    terrain: BiomeTerrain,
    surface: i32,
}

struct WorldSampler<'a> {
    request: &'a ChunkGenerationRequest,
    biomes: ServerBiomeSampler<'a>,
    registry: &'a ServerBiomeRegistry,
    world_seed: u64,
    height_cache: RefCell<HashMap<(i32, i32), i32>>,
}

impl<'a> WorldSampler<'a> {
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
            height_cache: RefCell::new(HashMap::new()),
        }
    }

    fn build_chunk(&self) -> Chunk {
        let position = self.request.position;
        let origin = position.world_origin();
        let mut columns = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                columns.push(self.column(origin.x + x, origin.z + z));
            }
        }

        let mut active_biomes = BTreeSet::new();
        let mut minimum_surface = i32::MAX;
        let mut maximum_surface = i32::MIN;
        for z in -FEATURE_HORIZONTAL_MARGIN..(CHUNK_SIZE + FEATURE_HORIZONTAL_MARGIN) {
            for x in -FEATURE_HORIZONTAL_MARGIN..(CHUNK_SIZE + FEATURE_HORIZONTAL_MARGIN) {
                let world_x = origin.x + x;
                let world_z = origin.z + z;
                if let Some(biome) = self.biomes.biome_at(world_x, world_z) {
                    active_biomes.insert(biome);
                }
                let surface = self.surface_height(world_x, world_z);
                minimum_surface = minimum_surface.min(surface);
                maximum_surface = maximum_surface.max(surface);
            }
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

        if chunk_bottom > maximum_surface && !features_intersect {
            return Chunk::filled(position, BlockId::Air);
        }

        let maximum_subsurface_depth = active_biomes
            .iter()
            .filter_map(|biome| self.biomes.definition(*biome))
            .map(|definition| definition.terrain.subsurface_depth as i32)
            .max()
            .unwrap_or(0);
        let common_underground = active_biomes
            .iter()
            .filter_map(|biome| self.biomes.definition(*biome))
            .map(|definition| definition.terrain.underground)
            .reduce(|left, right| (left == right).then_some(left).unwrap_or(left));
        let underground_is_common = common_underground.is_some_and(|block| {
            active_biomes.iter().all(|biome| {
                self.biomes
                    .definition(*biome)
                    .is_some_and(|definition| definition.terrain.underground == block)
            })
        });
        if chunk_top < minimum_surface - maximum_subsurface_depth
            && !features_intersect
            && underground_is_common
        {
            return Chunk::filled(position, common_underground.unwrap());
        }

        let mut chunk = Chunk::filled(position, BlockId::Air);
        for local_y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let local = LocalBlockPos::new(x, local_y, z).unwrap();
                    let world_y = origin.y + local_y;
                    let column = columns[(x + z * CHUNK_SIZE) as usize];
                    let block = if world_y > column.surface {
                        BlockId::Air
                    } else if world_y == column.surface {
                        column.terrain.surface
                    } else if world_y >= column.surface - column.terrain.subsurface_depth as i32 {
                        column.terrain.subsurface
                    } else {
                        column.terrain.underground
                    };
                    chunk.set(local, block);
                }
            }
        }

        let surface_height_at = |x, z| self.surface_height(x, z);
        let biome_at = |x, z| self.biomes.biome_at(x, z);
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

    fn column(&self, x: i32, z: i32) -> ColumnSample {
        ColumnSample {
            terrain: self
                .biomes
                .terrain_at(x, z)
                .expect("the selected biome registry must not be empty"),
            surface: self.surface_height(x, z),
        }
    }

    fn surface_height(&self, x: i32, z: i32) -> i32 {
        if let Some(height) = self.height_cache.borrow().get(&(x, z)).copied() {
            return height;
        }

        let (base_height, height_variation, detail_variation) = self
            .biomes
            .blended_terrain_parameters(x, z, &[(-8, 1.0), (0, 2.0), (8, 1.0)])
            .unwrap_or((1.0, 0.0, 0.0));
        let seed = self.world_seed as u32;
        let macro_noise =
            PerlinNoise2d::new(seed ^ 0x4d41_4352).sample(x as f32 * 0.018, z as f32 * 0.018);
        let detail_noise = PerlinNoise2d::new(seed ^ 0x4445_5441)
            .sample(x as f32 * 0.071 + 31.7, z as f32 * 0.071 - 18.2);
        let ridge_noise = PerlinNoise2d::new(seed ^ 0x5249_4447)
            .sample(x as f32 * 0.009 - 93.0, z as f32 * 0.009 + 57.0);
        let ridge = 1.0 - ridge_noise.abs();
        let distant = base_height
            + macro_noise * height_variation
            + detail_noise * detail_variation
            + ridge * (height_variation * 0.18);

        // Preserve the shared spawn contract at y=2 and blend into generated terrain.
        let distance = ((x as f64).powi(2) + (z as f64).powi(2)).sqrt() as f32;
        let blend = smoothstep(((distance - 8.0) / 48.0).clamp(0.0, 1.0));
        let height = (1.0 + (distant - 1.0) * blend).round() as i32;
        self.height_cache.borrow_mut().insert((x, z), height);
        height
    }
}

fn smoothstep(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use generated_biome_registry::BiomeId;
    use server_biome_api::{BiomeClimate, BiomeDefinition, BiomeVisuals};
    use server_biome_selection_api::{BiomeSelectionRequest, ServerBiomeSelector};
    use server_chunk_provider_api::ChunkViewer;
    use voxel_math_api::ChunkPos;
    use world_instance_api::WorldInstanceId;

    struct FixedSelector;

    impl ServerBiomeSelector for FixedSelector {
        fn select(
            &self,
            _request: &BiomeSelectionRequest<'_>,
            definitions: &[BiomeDefinition],
        ) -> Option<BiomeId> {
            definitions.first().map(|definition| definition.id)
        }
    }

    fn provider() -> BiomeTerrainProvider {
        let registry = ServerBiomeRegistry::default();
        registry
            .register_biome(BiomeDefinition {
                id: BiomeId::Plains,
                dimension: Dimension::Overworld,
                name: "Test plains",
                climate: BiomeClimate {
                    temperature: 0.5,
                    humidity: 0.5,
                    continentalness: 0.5,
                    has_precipitation: true,
                    downfall: 0.5,
                },
                terrain: BiomeTerrain {
                    base_height: 5.0,
                    height_variation: 2.0,
                    detail_variation: 1.0,
                    surface: BlockId::Grass,
                    subsurface: BlockId::Dirt,
                    underground: BlockId::Stone,
                    subsurface_depth: 3,
                },
                visuals: BiomeVisuals {
                    sky_color: [0.5; 3],
                    fog_color: [0.5; 3],
                    water_color: [0.5; 3],
                    grass_tint: [0.5; 3],
                    foliage_tint: [0.5; 3],
                },
                features: Vec::new(),
            })
            .unwrap();
        BiomeTerrainProvider {
            biomes: registry,
            selector: ServerBiomeSelectorResource::new(FixedSelector),
            world_seed: ServerWorldSeed::new(42),
            validated: Arc::new(OnceLock::new()),
        }
    }

    fn request(position: ChunkPos) -> ChunkGenerationRequest {
        ChunkGenerationRequest {
            viewer: ChunkViewer::Server,
            instance: WorldInstanceId::new("test:overworld"),
            position,
        }
    }

    #[test]
    fn high_and_deep_chunks_use_uniform_palette_fast_paths() {
        let provider = provider();
        assert_eq!(
            provider
                .generate(&request(ChunkPos::new(0, 20, 0)))
                .unwrap()
                .uniform_block()
                .unwrap()
                .block,
            BlockId::Air
        );
        assert_eq!(
            provider
                .generate(&request(ChunkPos::new(0, -20, 0)))
                .unwrap()
                .uniform_block()
                .unwrap()
                .block,
            BlockId::Stone
        );
    }

    #[test]
    fn spawn_surface_remains_at_one() {
        let provider = provider();
        let request = request(ChunkPos::new(0, 0, 0));
        let biomes = ServerBiomeSampler::new(
            &request,
            &provider.biomes,
            &provider.selector,
            Dimension::Overworld,
        );
        let sampler = WorldSampler::new(
            &request,
            biomes,
            &provider.biomes,
            provider
                .world_seed
                .derive(TERRAIN_SEED_NAMESPACE, &request.instance),
        );
        assert_eq!(sampler.surface_height(0, 0), 1);
    }
}
