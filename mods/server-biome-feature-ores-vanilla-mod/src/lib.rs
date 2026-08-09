use bevy_mod::BevyMod;
use generated_block_registry::BlockId;
use server_biome_api::{
    BiomeFeatureContext, BiomeFeatureId, BiomeFeaturePhase, FeatureVerticalRange, ServerBiomeApi,
    ServerBiomeFeature, ServerBiomeRegistry,
};
use tokio::task::JoinHandle;
use voxel_math_api::{CHUNK_SIZE, LocalBlockPos};

// Kept for compatibility with existing biome definition mods. The implementation
// now generates the complete vanilla-style Overworld ore palette.
pub const ORES_FEATURE_ID: &str = "demo:diamond-ores";

pub fn ores_feature_id() -> BiomeFeatureId {
    BiomeFeatureId::new(ORES_FEATURE_ID)
}

pub struct ServerBiomeFeatureOresVanillaMod;

impl ServerBiomeFeatureOresVanillaMod {
    pub fn init<B: ServerBiomeApi>(
        bevy: &mut BevyMod,
        _biomes: &mut B,
        _coal_ore: &mut block_coal_ore::BlockCoalOreMod,
        _iron_ore: &mut block_iron_ore::BlockIronOreMod,
        _copper_ore: &mut block_copper_ore::BlockCopperOreMod,
        _gold_ore: &mut block_gold_ore::BlockGoldOreMod,
        _lapis_ore: &mut block_lapis_ore::BlockLapisOreMod,
        _redstone_ore: &mut block_redstone_ore::BlockRedstoneOreMod,
        _emerald_ore: &mut block_emerald_ore::BlockEmeraldOreMod,
        _deepslate: &mut block_deepslate::BlockDeepslateMod,
        _deepslate_coal_ore: &mut block_deepslate_coal_ore::BlockDeepslateCoalOreMod,
        _deepslate_iron_ore: &mut block_deepslate_iron_ore::BlockDeepslateIronOreMod,
        _deepslate_copper_ore: &mut block_deepslate_copper_ore::BlockDeepslateCopperOreMod,
        _deepslate_gold_ore: &mut block_deepslate_gold_ore::BlockDeepslateGoldOreMod,
        _deepslate_lapis_ore: &mut block_deepslate_lapis_ore::BlockDeepslateLapisOreMod,
        _deepslate_redstone_ore: &mut block_deepslate_redstone_ore::BlockDeepslateRedstoneOreMod,
        _deepslate_diamond_ore: &mut block_deepslate_diamond_ore::BlockDeepslateDiamondOreMod,
        _deepslate_emerald_ore: &mut block_deepslate_emerald_ore::BlockDeepslateEmeraldOreMod,
    ) -> Self {
        bevy.app
            .world()
            .resource::<ServerBiomeRegistry>()
            .register_feature(ores_feature_id(), OresFeature)
            .expect("the vanilla overworld ore feature id must be unique");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

struct OresFeature;

impl ServerBiomeFeature for OresFeature {
    fn phase(&self) -> BiomeFeaturePhase {
        BiomeFeaturePhase::Underground
    }

    fn vertical_range(&self) -> FeatureVerticalRange {
        FeatureVerticalRange::Absolute { min: -64, max: 96 }
    }

    fn generate(&self, context: &mut BiomeFeatureContext<'_>) {
        let chunk = context.chunk_position();
        let underground = context.definition().terrain.underground;
        for local_y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let world = LocalBlockPos::new(x, local_y, z).unwrap().to_world(chunk);
                    if !context.is_target_biome(world.x, world.z)
                        || !(-64..=96).contains(&world.y)
                        || context
                            .block(world)
                            .is_none_or(|block| block.block != underground)
                    {
                        continue;
                    }

                    let hash = context.hash(world, 0x4f56_4552_574f_5245);
                    let roll = hash % 4096;
                    let deep = world.y < 0 || underground == BlockId::Deepslate;
                    let ore = if world.y <= -24 && roll < 12 {
                        if deep {
                            BlockId::DeepslateDiamondOre
                        } else {
                            BlockId::DiamondOre
                        }
                    } else if world.y <= 40 && roll < 46 {
                        if deep {
                            BlockId::DeepslateRedstoneOre
                        } else {
                            BlockId::RedstoneOre
                        }
                    } else if (-32..=48).contains(&world.y) && roll < 70 {
                        if deep {
                            BlockId::DeepslateGoldOre
                        } else {
                            BlockId::GoldOre
                        }
                    } else if (-16..=72).contains(&world.y) && roll < 106 {
                        if deep {
                            BlockId::DeepslateLapisOre
                        } else {
                            BlockId::LapisOre
                        }
                    } else if (-16..=80).contains(&world.y) && roll < 178 {
                        if deep {
                            BlockId::DeepslateCopperOre
                        } else {
                            BlockId::CopperOre
                        }
                    } else if (-48..=96).contains(&world.y) && roll < 294 {
                        if deep {
                            BlockId::DeepslateIronOre
                        } else {
                            BlockId::IronOre
                        }
                    } else if world.y >= 0 && roll < 420 {
                        BlockId::CoalOre
                    } else if world.y >= 28 && roll < 428 {
                        BlockId::EmeraldOre
                    } else if deep && roll < 434 {
                        BlockId::DeepslateEmeraldOre
                    } else if deep && roll < 520 {
                        BlockId::DeepslateCoalOre
                    } else {
                        continue;
                    };
                    context.set_block(world, ore);
                }
            }
        }
    }
}
