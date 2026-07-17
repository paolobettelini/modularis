use chunk_api::Chunk;
use generated_biome_registry::BiomeId;
use server_biome_api::{
    BIOME_FEATURE_PHASES, BiomeDefinition, BiomeTerrain, Dimension, ServerBiomeFeature,
    ServerBiomeRegistry,
};
use server_biome_selection_api::{BiomeSelectionRequest, ServerBiomeSelectorResource};
use server_chunk_provider_api::ChunkGenerationRequest;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

/// Shared, dimension-aware biome lookup used by independent chunk providers.
///
/// The sampler owns no terrain policy: Overworld, Nether, Aether, saved-world,
/// or custom providers decide how a selected biome becomes blocks.
pub struct ServerBiomeSampler<'a> {
    request: &'a ChunkGenerationRequest,
    definitions: Vec<BiomeDefinition>,
    selector: &'a ServerBiomeSelectorResource,
    cache: RefCell<HashMap<(i32, i32), BiomeId>>,
}

impl<'a> ServerBiomeSampler<'a> {
    pub fn new(
        request: &'a ChunkGenerationRequest,
        registry: &ServerBiomeRegistry,
        selector: &'a ServerBiomeSelectorResource,
        dimension: Dimension,
    ) -> Self {
        Self {
            request,
            definitions: registry.definitions_for_dimension(dimension),
            selector,
            cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    pub fn definitions(&self) -> &[BiomeDefinition] {
        &self.definitions
    }

    pub fn biome_at(&self, x: i32, z: i32) -> Option<BiomeId> {
        if let Some(biome) = self.cache.borrow().get(&(x, z)).copied() {
            return Some(biome);
        }
        let biome = self.selector.select(
            &BiomeSelectionRequest {
                generation: self.request,
                x,
                z,
            },
            &self.definitions,
        )?;
        self.cache.borrow_mut().insert((x, z), biome);
        Some(biome)
    }

    pub fn definition(&self, biome: BiomeId) -> Option<&BiomeDefinition> {
        self.definitions
            .binary_search_by_key(&biome, |definition| definition.id)
            .ok()
            .map(|index| &self.definitions[index])
    }

    pub fn terrain_at(&self, x: i32, z: i32) -> Option<BiomeTerrain> {
        self.definition(self.biome_at(x, z)?)
            .map(|definition| definition.terrain)
    }

    pub fn blended_terrain_parameters(
        &self,
        x: i32,
        z: i32,
        samples: &[(i32, f32)],
    ) -> Option<(f32, f32, f32)> {
        let mut base_height = 0.0;
        let mut height_variation = 0.0;
        let mut detail_variation = 0.0;
        let mut total_weight = 0.0;
        for (offset_x, weight_x) in samples {
            for (offset_z, weight_z) in samples {
                let weight = weight_x * weight_z;
                let definition = self.definition(self.biome_at(x + offset_x, z + offset_z)?)?;
                base_height += definition.terrain.base_height * weight;
                height_variation += definition.terrain.height_variation * weight;
                detail_variation += definition.terrain.detail_variation * weight;
                total_weight += weight;
            }
        }
        (total_weight > 0.0).then_some((
            base_height / total_weight,
            height_variation / total_weight,
            detail_variation / total_weight,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn apply_features(
        &self,
        registry: &ServerBiomeRegistry,
        chunk: &mut Chunk,
        world_seed: u64,
        active_biomes: &BTreeSet<BiomeId>,
        minimum_surface: i32,
        maximum_surface: i32,
        surface_height_at: &dyn Fn(i32, i32) -> i32,
        biome_at: &dyn Fn(i32, i32) -> Option<BiomeId>,
    ) {
        let chunk_bottom = chunk.position().world_origin().y;
        let chunk_top = chunk_bottom + voxel_math_api::CHUNK_SIZE - 1;
        let active_features = active_features(registry, &self.definitions, active_biomes);
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
                let Some(definition) = self.definition(active.biome) else {
                    continue;
                };
                let mut context = server_biome_api::BiomeFeatureContext::new(
                    self.request,
                    chunk,
                    active.biome,
                    definition,
                    world_seed,
                    surface_height_at,
                    biome_at,
                );
                active.feature.generate(&mut context);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn features_intersect(
        &self,
        registry: &ServerBiomeRegistry,
        active_biomes: &BTreeSet<BiomeId>,
        chunk_bottom: i32,
        chunk_top: i32,
        minimum_surface: i32,
        maximum_surface: i32,
    ) -> bool {
        active_features(registry, &self.definitions, active_biomes)
            .iter()
            .any(|active| {
                active.feature.vertical_range().intersects(
                    chunk_bottom,
                    chunk_top,
                    minimum_surface,
                    maximum_surface,
                )
            })
    }
}

struct ActiveFeature {
    biome: BiomeId,
    feature: Arc<dyn ServerBiomeFeature>,
}

fn active_features(
    registry: &ServerBiomeRegistry,
    definitions: &[BiomeDefinition],
    biomes: &BTreeSet<BiomeId>,
) -> Vec<ActiveFeature> {
    let mut result = Vec::new();
    for biome in biomes {
        let Ok(index) = definitions.binary_search_by_key(biome, |definition| definition.id) else {
            continue;
        };
        for id in &definitions[index].features {
            let feature = registry
                .feature(id)
                .unwrap_or_else(|| panic!("biome {:?} references missing feature '{}'", biome, id));
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
