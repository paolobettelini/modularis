use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use chunk_api::Chunk;
use coherent_noise_api::PerlinNoise2d;
use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
use server_biome_api::{
    BIOME_FEATURE_PHASES, BiomeDefinition, BiomeTerrain, ServerBiomeApi, ServerBiomeFeature,
    ServerBiomeRegistry,
};
use server_biome_selection_api::{
    BiomeSelectionRequest, ServerBiomeSelectionApi, ServerBiomeSelectorResource,
};
use server_chunk_provider_api::{
    ChunkGenerationRequest, ChunkProviderId, ServerChunkProvider, ServerChunkProviderRegistry,
};
use server_chunk_provider_registry_mod::ServerChunkProviderRegistryMod;
use server_primary_chunk_provider_api::ServerPrimaryChunkProviderApi;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, OnceLock};
use tokio::task::JoinHandle;
use voxel_math_api::{CHUNK_SIZE, LocalBlockPos};

const TERRAIN_SEED: u64 = 0x5041_5443_4857_4f52;
const FEATURE_HORIZONTAL_MARGIN: i32 = 2;

pub struct ServerChunkProviderBiomesMod;

impl ServerChunkProviderBiomesMod {
    pub fn init<B: BlockManagerApi, A: ServerBiomeApi, S: ServerBiomeSelectionApi>(
        bevy: &mut BevyMod,
        _provider_registry: &mut ServerChunkProviderRegistryMod,
        _blocks: &mut B,
        _biomes: &mut A,
        _selection: &mut S,
    ) -> Self {
        let biomes = bevy.app.world().resource::<ServerBiomeRegistry>().clone();
        let selector = bevy
            .app
            .world()
            .resource::<ServerBiomeSelectorResource>()
            .clone();
        bevy.app
            .world()
            .resource::<ServerChunkProviderRegistry>()
            .register(
                ChunkProviderId::primary(),
                BiomeTerrainProvider {
                    biomes,
                    selector,
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
        let definitions = self.biomes.definitions();
        if definitions.is_empty() {
            return None;
        }
        Some(WorldSampler::new(request, &definitions, &self.selector, &self.biomes).build_chunk())
    }
}

#[derive(Debug, Clone, Copy)]
struct ColumnSample {
    terrain: BiomeTerrain,
    surface: i32,
}

struct ActiveFeature {
    biome: BiomeId,
    feature: Arc<dyn ServerBiomeFeature>,
}

struct WorldSampler<'a> {
    request: &'a ChunkGenerationRequest,
    definitions: &'a [BiomeDefinition],
    selector: &'a ServerBiomeSelectorResource,
    registry: &'a ServerBiomeRegistry,
    world_seed: u64,
    biome_cache: RefCell<HashMap<(i32, i32), BiomeId>>,
    height_cache: RefCell<HashMap<(i32, i32), i32>>,
}

impl<'a> WorldSampler<'a> {
    fn new(
        request: &'a ChunkGenerationRequest,
        definitions: &'a [BiomeDefinition],
        selector: &'a ServerBiomeSelectorResource,
        registry: &'a ServerBiomeRegistry,
    ) -> Self {
        Self {
            request,
            definitions,
            selector,
            registry,
            world_seed: TERRAIN_SEED ^ stable_string_hash(&request.instance.0),
            biome_cache: RefCell::new(HashMap::new()),
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
                if let Some(biome) = self.biome_at(world_x, world_z) {
                    active_biomes.insert(biome);
                }
                let surface = self.surface_height(world_x, world_z);
                minimum_surface = minimum_surface.min(surface);
                maximum_surface = maximum_surface.max(surface);
            }
        }

        let active_features = self.active_features(&active_biomes);
        let chunk_bottom = origin.y;
        let chunk_top = origin.y + CHUNK_SIZE - 1;
        let features_intersect = active_features.iter().any(|active| {
            active.feature.vertical_range().intersects(
                chunk_bottom,
                chunk_top,
                minimum_surface,
                maximum_surface,
            )
        });

        if chunk_bottom > maximum_surface && !features_intersect {
            return Chunk::filled(position, BlockId::Air);
        }

        let maximum_subsurface_depth = active_biomes
            .iter()
            .filter_map(|biome| self.definition(*biome))
            .map(|definition| definition.terrain.subsurface_depth as i32)
            .max()
            .unwrap_or(0);
        let common_underground = active_biomes
            .iter()
            .filter_map(|biome| self.definition(*biome))
            .map(|definition| definition.terrain.underground)
            .reduce(|left, right| (left == right).then_some(left).unwrap_or(left));
        let underground_is_common = common_underground.is_some_and(|block| {
            active_biomes.iter().all(|biome| {
                self.definition(*biome)
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
        let biome_at = |x, z| self.biome_at(x, z);
        for phase in BIOME_FEATURE_PHASES {
            for active in active_features
                .iter()
                .filter(|active| active.feature.phase() == *phase)
            {
                if !active.feature.vertical_range().intersects(
                    chunk_bottom,
                    chunk_top,
                    minimum_surface,
                    maximum_surface,
                ) {
                    continue;
                }
                let definition = self
                    .definition(active.biome)
                    .expect("an active biome must have a registered definition");
                let mut context = server_biome_api::BiomeFeatureContext::new(
                    self.request,
                    &mut chunk,
                    active.biome,
                    definition,
                    self.world_seed,
                    &surface_height_at,
                    &biome_at,
                );
                active.feature.generate(&mut context);
            }
        }
        chunk
    }

    fn active_features(&self, biomes: &BTreeSet<BiomeId>) -> Vec<ActiveFeature> {
        let mut result = Vec::new();
        for biome in biomes {
            let Some(definition) = self.definition(*biome) else {
                continue;
            };
            for id in &definition.features {
                let feature = self.registry.feature(id).unwrap_or_else(|| {
                    panic!("biome {:?} references missing feature '{}'", biome, id)
                });
                result.push(ActiveFeature {
                    biome: *biome,
                    feature,
                });
            }
        }
        result.sort_by(|left, right| {
            left.feature
                .phase()
                .cmp(&right.feature.phase())
                .then_with(|| left.biome.cmp(&right.biome))
        });
        result
    }

    fn column(&self, x: i32, z: i32) -> ColumnSample {
        let biome = self
            .biome_at(x, z)
            .expect("the selected biome registry must not be empty");
        let definition = self
            .definition(biome)
            .expect("a selector must return a registered biome");
        ColumnSample {
            terrain: definition.terrain,
            surface: self.surface_height(x, z),
        }
    }

    fn biome_at(&self, x: i32, z: i32) -> Option<BiomeId> {
        if let Some(biome) = self.biome_cache.borrow().get(&(x, z)).copied() {
            return Some(biome);
        }
        let biome = self.selector.select(
            &BiomeSelectionRequest {
                generation: self.request,
                x,
                z,
            },
            self.definitions,
        )?;
        self.biome_cache.borrow_mut().insert((x, z), biome);
        Some(biome)
    }

    fn definition(&self, biome: BiomeId) -> Option<&BiomeDefinition> {
        self.definitions
            .binary_search_by_key(&biome, |definition| definition.id)
            .ok()
            .map(|index| &self.definitions[index])
    }

    fn surface_height(&self, x: i32, z: i32) -> i32 {
        if let Some(height) = self.height_cache.borrow().get(&(x, z)).copied() {
            return height;
        }

        let (base_height, height_variation, detail_variation) =
            self.blended_terrain_parameters(x, z);
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

    fn blended_terrain_parameters(&self, x: i32, z: i32) -> (f32, f32, f32) {
        const SAMPLES: &[(i32, f32)] = &[(-8, 1.0), (0, 2.0), (8, 1.0)];
        let mut base_height = 0.0;
        let mut height_variation = 0.0;
        let mut detail_variation = 0.0;
        let mut total_weight = 0.0;
        for (offset_x, weight_x) in SAMPLES {
            for (offset_z, weight_z) in SAMPLES {
                let weight = weight_x * weight_z;
                let Some(biome) = self.biome_at(x + offset_x, z + offset_z) else {
                    continue;
                };
                let Some(definition) = self.definition(biome) else {
                    continue;
                };
                base_height += definition.terrain.base_height * weight;
                height_variation += definition.terrain.height_variation * weight;
                detail_variation += definition.terrain.detail_variation * weight;
                total_weight += weight;
            }
        }
        if total_weight == 0.0 {
            return (1.0, 0.0, 0.0);
        }
        (
            base_height / total_weight,
            height_variation / total_weight,
            detail_variation / total_weight,
        )
    }
}

fn stable_string_hash(value: &str) -> u64 {
    value.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ byte as u64).wrapping_mul(0x1000_0000_01b3)
    })
}

fn smoothstep(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use generated_biome_registry::BiomeId;
    use server_biome_api::{BiomeClimate, BiomeVisuals};
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
        let definitions = provider.biomes.definitions();
        let request = request(ChunkPos::new(0, 0, 0));
        let sampler =
            WorldSampler::new(&request, &definitions, &provider.selector, &provider.biomes);
        assert_eq!(sampler.surface_height(0, 0), 1);
    }
}
