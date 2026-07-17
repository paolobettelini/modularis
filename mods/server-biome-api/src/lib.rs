use bevy::prelude::Resource;
use block_instance_api::BlockInstance;
use chunk_api::Chunk;
pub use generated_biome_registry::BiomeId;
use generated_block_registry::BlockId;
pub use generated_dimension_registry::Dimension;
use server_chunk_provider_api::ChunkGenerationRequest;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use voxel_math_api::{BlockPos, ChunkPos};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BiomeClimate {
    /// Target value used by climate selectors. Values normally stay in 0..=1.
    pub temperature: f32,
    /// Target value used by climate selectors. Values normally stay in 0..=1.
    pub humidity: f32,
    /// Target inland/mountain value. Values normally stay in 0..=1.
    pub continentalness: f32,
    pub has_precipitation: bool,
    pub downfall: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BiomeVisuals {
    pub sky_color: [f32; 3],
    pub fog_color: [f32; 3],
    pub water_color: [f32; 3],
    pub grass_tint: [f32; 3],
    pub foliage_tint: [f32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BiomeTerrain {
    pub base_height: f32,
    pub height_variation: f32,
    pub detail_variation: f32,
    pub surface: BlockId,
    pub subsurface: BlockId,
    pub underground: BlockId,
    pub subsurface_depth: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BiomeFeatureId(pub String);

impl BiomeFeatureId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for BiomeFeatureId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BiomeDefinition {
    pub id: BiomeId,
    /// Dimension in which this definition may be selected.
    pub dimension: Dimension,
    pub name: &'static str,
    pub climate: BiomeClimate,
    pub terrain: BiomeTerrain,
    pub visuals: BiomeVisuals,
    /// Feature ids are interpreted in phase order by the selected chunk provider.
    pub features: Vec<BiomeFeatureId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BiomeFeaturePhase {
    Carving,
    Underground,
    Surface,
    Decoration,
    Finalization,
}

pub const BIOME_FEATURE_PHASES: &[BiomeFeaturePhase] = &[
    BiomeFeaturePhase::Carving,
    BiomeFeaturePhase::Underground,
    BiomeFeaturePhase::Surface,
    BiomeFeaturePhase::Decoration,
    BiomeFeaturePhase::Finalization,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureVerticalRange {
    Any,
    Absolute { min: i32, max: i32 },
    RelativeToSurface { min: i32, max: i32 },
}

impl FeatureVerticalRange {
    pub fn intersects(
        self,
        chunk_bottom: i32,
        chunk_top: i32,
        minimum_surface: i32,
        maximum_surface: i32,
    ) -> bool {
        let (minimum, maximum) = match self {
            Self::Any => return true,
            Self::Absolute { min, max } => (min, max),
            Self::RelativeToSurface { min, max } => (minimum_surface + min, maximum_surface + max),
        };
        chunk_bottom <= maximum && chunk_top >= minimum
    }
}

pub trait ServerBiomeFeature: Send + Sync + 'static {
    fn phase(&self) -> BiomeFeaturePhase;

    /// A conservative bound used to skip empty or uniform vertical chunks.
    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::Any
    }

    fn generate(&self, context: &mut BiomeFeatureContext<'_>);
}

pub struct BiomeFeatureContext<'a> {
    request: &'a ChunkGenerationRequest,
    chunk: &'a mut Chunk,
    target_biome: BiomeId,
    definition: &'a BiomeDefinition,
    world_seed: u64,
    surface_height_at: &'a dyn Fn(i32, i32) -> i32,
    biome_at: &'a dyn Fn(i32, i32) -> Option<BiomeId>,
}

impl<'a> BiomeFeatureContext<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        request: &'a ChunkGenerationRequest,
        chunk: &'a mut Chunk,
        target_biome: BiomeId,
        definition: &'a BiomeDefinition,
        world_seed: u64,
        surface_height_at: &'a dyn Fn(i32, i32) -> i32,
        biome_at: &'a dyn Fn(i32, i32) -> Option<BiomeId>,
    ) -> Self {
        Self {
            request,
            chunk,
            target_biome,
            definition,
            world_seed,
            surface_height_at,
            biome_at,
        }
    }

    pub fn request(&self) -> &ChunkGenerationRequest {
        self.request
    }

    pub fn chunk_position(&self) -> ChunkPos {
        self.chunk.position()
    }

    pub fn target_biome(&self) -> BiomeId {
        self.target_biome
    }

    pub fn world_seed(&self) -> u64 {
        self.world_seed
    }

    pub fn definition(&self) -> &BiomeDefinition {
        self.definition
    }

    pub fn surface_height(&self, x: i32, z: i32) -> i32 {
        (self.surface_height_at)(x, z)
    }

    pub fn biome_at(&self, x: i32, z: i32) -> Option<BiomeId> {
        (self.biome_at)(x, z)
    }

    pub fn is_target_biome(&self, x: i32, z: i32) -> bool {
        self.biome_at(x, z) == Some(self.target_biome)
    }

    pub fn block(&self, position: BlockPos) -> Option<BlockInstance> {
        (position.chunk() == self.chunk.position()).then(|| self.chunk.get(position.local()))
    }

    /// Writes only inside the chunk currently being generated. Features may inspect
    /// neighboring anchor positions and call this method; writes are clipped so the
    /// same deterministic feature can be evaluated independently for every chunk.
    pub fn set_block(&mut self, position: BlockPos, block: impl Into<BlockInstance>) -> bool {
        if position.chunk() != self.chunk.position() {
            return false;
        }
        self.chunk.set(position.local(), block);
        true
    }

    pub fn hash(&self, position: BlockPos, salt: u64) -> u64 {
        stable_position_hash(self.world_seed ^ salt, position)
    }
}

fn stable_position_hash(seed: u64, position: BlockPos) -> u64 {
    let mut value = seed
        ^ (position.x as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (position.y as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9)
        ^ (position.z as u64).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[derive(Default)]
struct BiomeRegistryState {
    definitions: BTreeMap<BiomeId, BiomeDefinition>,
    features: BTreeMap<BiomeFeatureId, Arc<dyn ServerBiomeFeature>>,
}

#[derive(Resource, Clone, Default)]
pub struct ServerBiomeRegistry(Arc<RwLock<BiomeRegistryState>>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BiomeRegistrationError {
    DuplicateBiome(BiomeId),
    DuplicateFeature(BiomeFeatureId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingBiomeFeature {
    pub biome: BiomeId,
    pub feature: BiomeFeatureId,
}

impl ServerBiomeRegistry {
    pub fn register_biome(
        &self,
        definition: BiomeDefinition,
    ) -> Result<(), BiomeRegistrationError> {
        let mut state = self.0.write().expect("server biome registry lock poisoned");
        if state.definitions.contains_key(&definition.id) {
            return Err(BiomeRegistrationError::DuplicateBiome(definition.id));
        }
        state.definitions.insert(definition.id, definition);
        Ok(())
    }

    pub fn register_feature(
        &self,
        id: BiomeFeatureId,
        feature: impl ServerBiomeFeature,
    ) -> Result<(), BiomeRegistrationError> {
        let mut state = self.0.write().expect("server biome registry lock poisoned");
        if state.features.contains_key(&id) {
            return Err(BiomeRegistrationError::DuplicateFeature(id));
        }
        state.features.insert(id, Arc::new(feature));
        Ok(())
    }

    pub fn definition(&self, id: BiomeId) -> Option<BiomeDefinition> {
        self.0
            .read()
            .expect("server biome registry lock poisoned")
            .definitions
            .get(&id)
            .cloned()
    }

    pub fn definitions(&self) -> Vec<BiomeDefinition> {
        self.0
            .read()
            .expect("server biome registry lock poisoned")
            .definitions
            .values()
            .cloned()
            .collect()
    }

    pub fn definitions_for_dimension(&self, dimension: Dimension) -> Vec<BiomeDefinition> {
        self.0
            .read()
            .expect("server biome registry lock poisoned")
            .definitions
            .values()
            .filter(|definition| definition.dimension == dimension)
            .cloned()
            .collect()
    }

    pub fn feature(&self, id: &BiomeFeatureId) -> Option<Arc<dyn ServerBiomeFeature>> {
        self.0
            .read()
            .expect("server biome registry lock poisoned")
            .features
            .get(id)
            .cloned()
    }

    pub fn missing_features(&self) -> Vec<MissingBiomeFeature> {
        let state = self.0.read().expect("server biome registry lock poisoned");
        state
            .definitions
            .values()
            .flat_map(|definition| {
                definition.features.iter().filter_map(|feature| {
                    (!state.features.contains_key(feature)).then(|| MissingBiomeFeature {
                        biome: definition.id,
                        feature: feature.clone(),
                    })
                })
            })
            .collect()
    }
}

pub trait ServerBiomeApi: Send + Sync + 'static {}

#[cfg(test)]
mod tests {
    use super::*;

    struct NoopFeature;

    impl ServerBiomeFeature for NoopFeature {
        fn phase(&self) -> BiomeFeaturePhase {
            BiomeFeaturePhase::Decoration
        }

        fn generate(&self, _context: &mut BiomeFeatureContext<'_>) {}
    }

    fn definition(features: Vec<BiomeFeatureId>) -> BiomeDefinition {
        BiomeDefinition {
            id: BiomeId::Plains,
            dimension: Dimension::Overworld,
            name: "Plains",
            climate: BiomeClimate {
                temperature: 0.5,
                humidity: 0.5,
                continentalness: 0.5,
                has_precipitation: true,
                downfall: 0.4,
            },
            terrain: BiomeTerrain {
                base_height: 4.0,
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
            features,
        }
    }

    #[test]
    fn missing_feature_validation_is_data_driven() {
        let registry = ServerBiomeRegistry::default();
        let feature_id = BiomeFeatureId::new("test:feature");
        registry
            .register_biome(definition(vec![feature_id.clone()]))
            .unwrap();
        assert_eq!(registry.missing_features().len(), 1);
        registry.register_feature(feature_id, NoopFeature).unwrap();
        assert!(registry.missing_features().is_empty());
    }

    #[test]
    fn relative_feature_ranges_skip_unrelated_vertical_chunks() {
        let range = FeatureVerticalRange::RelativeToSurface { min: 1, max: 7 };
        assert!(range.intersects(16, 31, 10, 20));
        assert!(!range.intersects(64, 79, 10, 20));
    }

    #[test]
    fn definitions_are_filtered_by_dimension() {
        let registry = ServerBiomeRegistry::default();
        registry.register_biome(definition(Vec::new())).unwrap();
        let mut nether = definition(Vec::new());
        nether.id = BiomeId::NetherWastes;
        nether.dimension = Dimension::Nether;
        registry.register_biome(nether).unwrap();

        assert_eq!(
            registry.definitions_for_dimension(Dimension::Overworld)[0].id,
            BiomeId::Plains
        );
        assert_eq!(
            registry.definitions_for_dimension(Dimension::Nether)[0].id,
            BiomeId::NetherWastes
        );
        assert!(
            registry
                .definitions_for_dimension(Dimension::Aether)
                .is_empty()
        );
    }
}
