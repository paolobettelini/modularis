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

pub const NETHER_PROVIDER_ID: &str = "demo:nether-terrain";
const NETHER_SEED_NAMESPACE: &str = "demo:nether-terrain";
const FEATURE_HORIZONTAL_MARGIN: i32 = 2;

pub struct ServerChunkProviderNetherMod;

impl ServerChunkProviderNetherMod {
    pub fn init<
        B: BlockManagerApi,
        A: ServerBiomeApi,
        S: ServerBiomeSelectionApi,
        W: ServerWorldSeedApi,
    >(
        bevy: &mut BevyMod,
        _registry_mod: &mut ServerChunkProviderRegistryMod,
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
                ChunkProviderId::new(NETHER_PROVIDER_ID),
                NetherTerrainProvider {
                    biomes,
                    selector,
                    world_seed,
                    validated: Arc::new(OnceLock::new()),
                },
            )
            .expect("the Nether chunk provider id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct NetherTerrainProvider {
    biomes: ServerBiomeRegistry,
    selector: ServerBiomeSelectorResource,
    world_seed: ServerWorldSeed,
    validated: Arc<OnceLock<()>>,
}

impl ServerChunkProvider for NetherTerrainProvider {
    fn generate(&self, request: &ChunkGenerationRequest) -> Option<Chunk> {
        self.validated
            .get_or_init(|| validate_registry(&self.biomes));
        let biomes =
            ServerBiomeSampler::new(request, &self.biomes, &self.selector, Dimension::Nether);
        assert!(
            !biomes.is_empty(),
            "the Nether provider requires at least one Nether biome"
        );
        Some(
            NetherWorldSampler::new(
                request,
                biomes,
                &self.biomes,
                self.world_seed
                    .derive(NETHER_SEED_NAMESPACE, &request.instance),
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
struct ColumnSample {
    terrain: BiomeTerrain,
    surface: i32,
}

struct NetherWorldSampler<'a> {
    request: &'a ChunkGenerationRequest,
    biomes: ServerBiomeSampler<'a>,
    registry: &'a ServerBiomeRegistry,
    world_seed: u64,
    height_cache: RefCell<HashMap<(i32, i32), i32>>,
}

impl<'a> NetherWorldSampler<'a> {
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

        let mut chunk = Chunk::filled(position, BlockId::Air);
        for local_y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let local = LocalBlockPos::new(x, local_y, z).unwrap();
                    let world_y = origin.y + local_y;
                    let column = columns[(x + z * CHUNK_SIZE) as usize];
                    let block = base_block_at(world_y, column);
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
                .expect("Nether biome must exist"),
            surface: self.surface_height(x, z),
        }
    }

    fn surface_height(&self, x: i32, z: i32) -> i32 {
        if let Some(height) = self.height_cache.borrow().get(&(x, z)).copied() {
            return height;
        }
        const SAMPLES: &[(i32, f32)] = &[(-8, 1.0), (0, 2.0), (8, 1.0)];
        let (base, variation, detail_variation) = self
            .biomes
            .blended_terrain_parameters(x, z, SAMPLES)
            .unwrap_or((4.0, 2.0, 1.0));
        let large = PerlinNoise2d::new(self.world_seed as u32 ^ 0x4e45_5448)
            .sample(x as f32 * 0.04, z as f32 * 0.04);
        let detail = PerlinNoise2d::new(self.world_seed as u32 ^ 0xd079_2df1)
            .sample(x as f32 * 0.11 - 22.0, z as f32 * 0.11 + 9.0);
        let distant = (base + large * variation + detail * detail_variation).max(1.0);
        let distance = ((x as f32).powi(2) + (z as f32).powi(2)).sqrt();
        let blend = smoothstep(((distance - 8.0) / 40.0).clamp(0.0, 1.0));
        let height = (1.0 + (distant - 1.0) * blend).round() as i32;
        self.height_cache.borrow_mut().insert((x, z), height);
        height
    }
}

fn base_block_at(world_y: i32, column: ColumnSample) -> BlockId {
    if world_y > column.surface {
        BlockId::Air
    } else if world_y == column.surface {
        column.terrain.surface
    } else if world_y >= column.surface - column.terrain.subsurface_depth as i32 {
        column.terrain.subsurface
    } else {
        column.terrain.underground
    }
}

fn smoothstep(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_zero_is_not_a_special_bedrock_plane() {
        let column = ColumnSample {
            terrain: BiomeTerrain {
                base_height: 8.0,
                height_variation: 0.0,
                detail_variation: 0.0,
                surface: BlockId::Netherrack,
                subsurface: BlockId::Netherrack,
                underground: BlockId::Netherrack,
                subsurface_depth: 3,
            },
            surface: 8,
        };

        assert_eq!(base_block_at(0, column), BlockId::Netherrack);
        assert_ne!(base_block_at(0, column), BlockId::Bedrock);
    }
}
