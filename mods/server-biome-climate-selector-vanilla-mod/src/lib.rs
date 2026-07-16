use bevy_mod::BevyMod;
use coherent_noise_api::PerlinNoise2d;
use generated_biome_registry::BiomeId;
use server_biome_api::{BiomeDefinition, ServerBiomeApi};
use server_biome_plains_vanilla_mod::ServerBiomePlainsVanillaMod;
use server_biome_selection_api::{
    BiomeSelectionRequest, ServerBiomeSelectionApi, ServerBiomeSelector,
    ServerBiomeSelectorResource,
};
use tokio::task::JoinHandle;

const CLIMATE_SEED: u32 = 0x4249_4f4d;

pub struct ServerBiomeClimateSelectorVanillaMod;

impl ServerBiomeClimateSelectorVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _plains: &mut ServerBiomePlainsVanillaMod,
    ) -> Self {
        bevy.app
            .insert_resource(ServerBiomeSelectorResource::new(ClimateNoiseBiomeSelector));
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerBiomeSelectionApi for ServerBiomeClimateSelectorVanillaMod {}

struct ClimateNoiseBiomeSelector;

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
        if distance_squared <= 48 * 48
            && definitions
                .iter()
                .any(|definition| definition.id == BiomeId::Plains)
        {
            return Some(BiomeId::Plains);
        }

        let instance_seed = stable_string_hash(&request.generation.instance.0) as u32;
        let temperature = normalized_noise(
            CLIMATE_SEED ^ instance_seed,
            request.x as f32 * 0.0042,
            request.z as f32 * 0.0042,
        );
        let humidity = normalized_noise(
            CLIMATE_SEED ^ instance_seed ^ 0x9e37_79b9,
            request.x as f32 * 0.0038 + 117.0,
            request.z as f32 * 0.0038 - 71.0,
        );
        let continentalness = normalized_noise(
            CLIMATE_SEED ^ instance_seed ^ 0x85eb_ca6b,
            request.x as f32 * 0.0027 - 203.0,
            request.z as f32 * 0.0027 + 149.0,
        );

        definitions
            .iter()
            .min_by(|left, right| {
                climate_distance(left, temperature, humidity, continentalness)
                    .total_cmp(&climate_distance(
                        right,
                        temperature,
                        humidity,
                        continentalness,
                    ))
                    .then_with(|| left.id.cmp(&right.id))
            })
            .map(|definition| definition.id)
    }
}

fn climate_distance(
    definition: &BiomeDefinition,
    temperature: f32,
    humidity: f32,
    continentalness: f32,
) -> f32 {
    let temperature_delta = temperature - definition.climate.temperature;
    let humidity_delta = humidity - definition.climate.humidity;
    let continentalness_delta = continentalness - definition.climate.continentalness;
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
}
