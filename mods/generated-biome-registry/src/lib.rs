#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum BiomeId {
    Desert,
    Forest,
    Plains,
    RockyPeaks,
    Tundra,
}

pub const ALL_BIOMES: &[BiomeId] = &[
    BiomeId::Desert,
    BiomeId::Forest,
    BiomeId::Plains,
    BiomeId::RockyPeaks,
    BiomeId::Tundra,
];

pub fn all_biomes() -> &'static [BiomeId] { ALL_BIOMES }

pub fn from_str(id: &str) -> Option<BiomeId> {
    match id {
        "demo:desert" => Some(BiomeId::Desert),
        "demo:forest" => Some(BiomeId::Forest),
        "demo:plains" => Some(BiomeId::Plains),
        "demo:rocky-peaks" => Some(BiomeId::RockyPeaks),
        "demo:tundra" => Some(BiomeId::Tundra),
        _ => None,
    }
}

pub fn id(biome: BiomeId) -> &'static str {
    match biome {
        BiomeId::Desert => "demo:desert",
        BiomeId::Forest => "demo:forest",
        BiomeId::Plains => "demo:plains",
        BiomeId::RockyPeaks => "demo:rocky-peaks",
        BiomeId::Tundra => "demo:tundra",
    }
}
