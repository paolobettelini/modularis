#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum BiomeId {
    AetherCrystalPeaks,
    AetherGoldenGrove,
    AetherHighlands,
    Badlands,
    BasaltDeltas,
    BirchForest,
    CrimsonForest,
    Desert,
    Forest,
    NetherWastes,
    Plains,
    RockyPeaks,
    SoulSandValley,
    Tundra,
    WarpedForest,
}

pub const ALL_BIOMES: &[BiomeId] = &[
    BiomeId::AetherCrystalPeaks,
    BiomeId::AetherGoldenGrove,
    BiomeId::AetherHighlands,
    BiomeId::Badlands,
    BiomeId::BasaltDeltas,
    BiomeId::BirchForest,
    BiomeId::CrimsonForest,
    BiomeId::Desert,
    BiomeId::Forest,
    BiomeId::NetherWastes,
    BiomeId::Plains,
    BiomeId::RockyPeaks,
    BiomeId::SoulSandValley,
    BiomeId::Tundra,
    BiomeId::WarpedForest,
];

pub fn all_biomes() -> &'static [BiomeId] { ALL_BIOMES }

pub fn from_str(id: &str) -> Option<BiomeId> {
    match id {
        "demo:aether-crystal-peaks" => Some(BiomeId::AetherCrystalPeaks),
        "demo:aether-golden-grove" => Some(BiomeId::AetherGoldenGrove),
        "demo:aether-highlands" => Some(BiomeId::AetherHighlands),
        "demo:badlands" => Some(BiomeId::Badlands),
        "demo:basalt-deltas" => Some(BiomeId::BasaltDeltas),
        "demo:birch-forest" => Some(BiomeId::BirchForest),
        "demo:crimson-forest" => Some(BiomeId::CrimsonForest),
        "demo:desert" => Some(BiomeId::Desert),
        "demo:forest" => Some(BiomeId::Forest),
        "demo:nether-wastes" => Some(BiomeId::NetherWastes),
        "demo:plains" => Some(BiomeId::Plains),
        "demo:rocky-peaks" => Some(BiomeId::RockyPeaks),
        "demo:soul-sand-valley" => Some(BiomeId::SoulSandValley),
        "demo:tundra" => Some(BiomeId::Tundra),
        "demo:warped-forest" => Some(BiomeId::WarpedForest),
        _ => None,
    }
}

pub fn id(biome: BiomeId) -> &'static str {
    match biome {
        BiomeId::AetherCrystalPeaks => "demo:aether-crystal-peaks",
        BiomeId::AetherGoldenGrove => "demo:aether-golden-grove",
        BiomeId::AetherHighlands => "demo:aether-highlands",
        BiomeId::Badlands => "demo:badlands",
        BiomeId::BasaltDeltas => "demo:basalt-deltas",
        BiomeId::BirchForest => "demo:birch-forest",
        BiomeId::CrimsonForest => "demo:crimson-forest",
        BiomeId::Desert => "demo:desert",
        BiomeId::Forest => "demo:forest",
        BiomeId::NetherWastes => "demo:nether-wastes",
        BiomeId::Plains => "demo:plains",
        BiomeId::RockyPeaks => "demo:rocky-peaks",
        BiomeId::SoulSandValley => "demo:soul-sand-valley",
        BiomeId::Tundra => "demo:tundra",
        BiomeId::WarpedForest => "demo:warped-forest",
    }
}
