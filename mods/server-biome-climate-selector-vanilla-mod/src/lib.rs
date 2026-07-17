use bevy_mod::BevyMod;
use coherent_noise_api::PerlinNoise2d;
use generated_biome_registry::BiomeId;
use server_biome_api::{BiomeClimate, BiomeDefinition, ServerBiomeApi};
use server_biome_plains_vanilla_mod::ServerBiomePlainsVanillaMod;
use server_biome_selection_api::{
    BiomeSelectionRequest, ServerBiomeSelectionApi, ServerBiomeSelector,
    ServerBiomeSelectorResource,
};
use server_world_seed_api::{ServerWorldSeed, ServerWorldSeedApi};
use tokio::task::JoinHandle;

const CLIMATE_SEED_NAMESPACE: &str = "demo:biome-climate";
const BIOME_SCALE_MULTIPLIER: f32 = 4.0;
const SPAWN_PLAINS_RADIUS: i64 = 8;
const REGION_DETAIL_STRENGTH: f32 = 0.18;
const CLIMATE_INFLUENCE: f32 = 0.10;

pub struct ServerBiomeClimateSelectorVanillaMod;

impl ServerBiomeClimateSelectorVanillaMod {
    pub fn init<B: ServerBiomeApi, W: ServerWorldSeedApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _plains: &mut ServerBiomePlainsVanillaMod,
        _world_seed: &mut W,
    ) -> Self {
        let world_seed = *bevy.app.world().resource::<ServerWorldSeed>();
        bevy.app.insert_resource(ServerBiomeSelectorResource::new(
            ClimateNoiseBiomeSelector { world_seed },
        ));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerBiomeSelectionApi for ServerBiomeClimateSelectorVanillaMod {}

struct ClimateNoiseBiomeSelector {
    world_seed: ServerWorldSeed,
}

impl ServerBiomeSelector for ClimateNoiseBiomeSelector {
    fn select(
        &self,
        request: &BiomeSelectionRequest<'_>,
        definitions: &[BiomeDefinition],
    ) -> Option<BiomeId> {
        if definitions.is_empty() {
            return None;
        }

        let distance_squared =
            request.x as i64 * request.x as i64 + request.z as i64 * request.z as i64;
        if distance_squared <= SPAWN_PLAINS_RADIUS * SPAWN_PLAINS_RADIUS
            && definitions
                .iter()
                .any(|definition| definition.id == BiomeId::Plains)
        {
            return Some(BiomeId::Plains);
        }

        let instance_seed =
            self.world_seed
                .derive(CLIMATE_SEED_NAMESPACE, &request.generation.instance) as u32;
        let temperature = normalized_noise(
            instance_seed ^ 0x4249_4f4d,
            request.x as f32 * 0.0042 * BIOME_SCALE_MULTIPLIER,
            request.z as f32 * 0.0042 * BIOME_SCALE_MULTIPLIER,
        );
        let humidity = normalized_noise(
            instance_seed ^ 0xdc7e_36f4,
            request.x as f32 * 0.0038 * BIOME_SCALE_MULTIPLIER + 117.0,
            request.z as f32 * 0.0038 * BIOME_SCALE_MULTIPLIER - 71.0,
        );
        let continentalness = normalized_noise(
            instance_seed ^ 0xc7a2_8526,
            request.x as f32 * 0.0027 * BIOME_SCALE_MULTIPLIER - 203.0,
            request.z as f32 * 0.0027 * BIOME_SCALE_MULTIPLIER + 149.0,
        );

        definitions
            .iter()
            .max_by(|left, right| {
                biome_score(
                    left.id,
                    &left.climate,
                    instance_seed,
                    request.x,
                    request.z,
                    temperature,
                    humidity,
                    continentalness,
                )
                .total_cmp(&biome_score(
                    right.id,
                    &right.climate,
                    instance_seed,
                    request.x,
                    request.z,
                    temperature,
                    humidity,
                    continentalness,
                ))
                .then_with(|| right.id.cmp(&left.id))
            })
            .map(|definition| definition.id)
    }
}

#[allow(clippy::too_many_arguments)]
fn biome_score(
    biome: BiomeId,
    climate: &BiomeClimate,
    instance_seed: u32,
    x: i32,
    z: i32,
    temperature: f32,
    humidity: f32,
    continentalness: f32,
) -> f32 {
    let biome_seed = stable_string_hash(generated_biome_registry::id(biome)) as u32;
    let region_x = x as f32 * 0.0042 * BIOME_SCALE_MULTIPLIER;
    let region_z = z as f32 * 0.0042 * BIOME_SCALE_MULTIPLIER;
    let broad =
        PerlinNoise2d::new(instance_seed ^ biome_seed ^ 0x4249_4f4d).sample(region_x, region_z);
    let detail = PerlinNoise2d::new(instance_seed ^ biome_seed.rotate_left(13) ^ 0x65ad_a460)
        .sample(region_x * 2.07 + 31.0, region_z * 2.07 - 47.0);
    let climate_penalty = climate_distance(climate, temperature, humidity, continentalness);
    broad + detail * REGION_DETAIL_STRENGTH - climate_penalty * CLIMATE_INFLUENCE
}

fn climate_distance(
    climate: &BiomeClimate,
    temperature: f32,
    humidity: f32,
    continentalness: f32,
) -> f32 {
    let temperature_delta = temperature - climate.temperature;
    let humidity_delta = humidity - climate.humidity;
    let continentalness_delta = continentalness - climate.continentalness;
    temperature_delta * temperature_delta * 1.25
        + humidity_delta * humidity_delta
        + continentalness_delta * continentalness_delta * 0.85
}

fn normalized_noise(seed: u32, x: f32, z: f32) -> f32 {
    (PerlinNoise2d::new(seed).sample(x, z) * 0.5 + 0.5).clamp(0.0, 1.0)
}

fn stable_string_hash(value: &str) -> u64 {
    value.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ byte as u64).wrapping_mul(0x1000_0000_01b3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const OVERWORLD_CLIMATES: &[(BiomeId, f32, f32, f32)] = &[
        (BiomeId::Plains, 0.58, 0.52, 0.45),
        (BiomeId::Forest, 0.56, 0.82, 0.48),
        (BiomeId::BirchForest, 0.48, 0.70, 0.50),
        (BiomeId::Desert, 0.92, 0.12, 0.42),
        (BiomeId::Badlands, 0.88, 0.08, 0.72),
        (BiomeId::Tundra, 0.08, 0.48, 0.44),
        (BiomeId::RockyPeaks, 0.32, 0.28, 0.90),
    ];
    const NETHER_CLIMATES: &[(BiomeId, f32, f32, f32)] = &[
        (BiomeId::NetherWastes, 0.65, 0.35, 0.50),
        (BiomeId::SoulSandValley, 0.32, 0.10, 0.42),
        (BiomeId::CrimsonForest, 0.82, 0.78, 0.44),
        (BiomeId::WarpedForest, 0.28, 0.88, 0.46),
        (BiomeId::BasaltDeltas, 0.72, 0.12, 0.90),
    ];
    const AETHER_CLIMATES: &[(BiomeId, f32, f32, f32)] = &[
        (BiomeId::AetherHighlands, 0.56, 0.58, 0.55),
        (BiomeId::AetherGoldenGrove, 0.78, 0.76, 0.44),
        (BiomeId::AetherCrystalPeaks, 0.18, 0.30, 0.92),
    ];

    #[test]
    fn instance_seed_is_stable_and_not_empty() {
        assert_eq!(
            stable_string_hash("demo:overworld"),
            stable_string_hash("demo:overworld")
        );
        assert_ne!(
            stable_string_hash("demo:overworld"),
            stable_string_hash("demo:other")
        );
    }

    #[test]
    fn every_dimension_has_a_balanced_biome_share_near_spawn() {
        assert_balanced("demo:overworld", OVERWORLD_CLIMATES);
        assert_balanced("demo:nether", NETHER_CLIMATES);
        assert_balanced("demo:aether", AETHER_CLIMATES);
    }

    fn assert_balanced(instance: &str, climates: &[(BiomeId, f32, f32, f32)]) {
        let instance_seed = ServerWorldSeed::new(42).derive(
            CLIMATE_SEED_NAMESPACE,
            &world_instance_api::WorldInstanceId::new(instance),
        ) as u32;
        let mut counts = vec![0_usize; climates.len()];
        let mut samples = 0_usize;

        for z in (-256..=256).step_by(4) {
            for x in (-256..=256).step_by(4) {
                let temperature = normalized_noise(
                    instance_seed ^ 0x4249_4f4d,
                    x as f32 * 0.0042 * BIOME_SCALE_MULTIPLIER,
                    z as f32 * 0.0042 * BIOME_SCALE_MULTIPLIER,
                );
                let humidity = normalized_noise(
                    instance_seed ^ 0xdc7e_36f4,
                    x as f32 * 0.0038 * BIOME_SCALE_MULTIPLIER + 117.0,
                    z as f32 * 0.0038 * BIOME_SCALE_MULTIPLIER - 71.0,
                );
                let continentalness = normalized_noise(
                    instance_seed ^ 0xc7a2_8526,
                    x as f32 * 0.0027 * BIOME_SCALE_MULTIPLIER - 203.0,
                    z as f32 * 0.0027 * BIOME_SCALE_MULTIPLIER + 149.0,
                );
                let selected = climates
                    .iter()
                    .enumerate()
                    .max_by(|(_, (left_id, lt, lh, lc)), (_, (right_id, rt, rh, rc))| {
                        biome_score(
                            *left_id,
                            &climate(*lt, *lh, *lc),
                            instance_seed,
                            x,
                            z,
                            temperature,
                            humidity,
                            continentalness,
                        )
                        .total_cmp(&biome_score(
                            *right_id,
                            &climate(*rt, *rh, *rc),
                            instance_seed,
                            x,
                            z,
                            temperature,
                            humidity,
                            continentalness,
                        ))
                    })
                    .map(|(index, _)| index)
                    .unwrap();
                counts[selected] += 1;
                samples += 1;
            }
        }

        for (index, count) in counts.into_iter().enumerate() {
            assert!(
                count * climates.len() * 2 >= samples,
                "{:?} only received {count}/{samples} samples",
                climates[index].0
            );
            assert!(
                count * climates.len() <= samples * 2,
                "{:?} received too many samples: {count}/{samples}",
                climates[index].0
            );
        }
    }

    fn climate(temperature: f32, humidity: f32, continentalness: f32) -> BiomeClimate {
        BiomeClimate {
            temperature,
            humidity,
            continentalness,
            has_precipitation: false,
            downfall: 0.0,
        }
    }
}
