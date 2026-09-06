use bevy::prelude::*;
use generated_block_registry::BlockId;
use std::collections::VecDeque;
use voxel_frame_api::{VoxelBlockAddress, VoxelFrameId, VoxelFrameTransform};
use voxel_math_api::BlockPos;

mod obstacles;
pub use obstacles::{
    ParkourFramePlan, ParkourObstacleBehavior, centered_block_pose,
    nominal_jump_difficulty, score_difficulty,
};

#[derive(Debug, Clone)]
pub struct ParkourConfig {
    pub start: BlockPos,
    pub initial_block_count: usize,
    pub fall_reset_distance: f32,
    pub block_palette: Vec<BlockId>,
}

impl Default for ParkourConfig {
    fn default() -> Self {
        Self {
            start: BlockPos::new(0, 32, 0),
            initial_block_count: 10,
            fall_reset_distance: 32.0,
            block_palette: vec![
                BlockId::Dirt,
                BlockId::Stone,
                BlockId::Bedrock,
                BlockId::CraftingTable,
                BlockId::DiamondBlock,
                BlockId::DiamondOre,
                BlockId::EndStone,
                BlockId::Glowstone,
                BlockId::Netherrack,
                BlockId::Obsidian,
                BlockId::Sand,
                BlockId::Snow,
                BlockId::Gravel,
                BlockId::PackedIce,
                BlockId::OakLog,
                BlockId::OakLeaves,
                BlockId::BirchLog,
                BlockId::BirchLeaves,
                BlockId::RedSand,
                BlockId::Terracotta,
                BlockId::SoulSand,
                BlockId::SoulSoil,
                BlockId::Basalt,
                BlockId::Blackstone,
                BlockId::AcaciaLeaves,
                BlockId::AcaciaLog,
                BlockId::AcaciaPlanks,
                BlockId::AmethystBlock,
                BlockId::AncientDebris,
                BlockId::Andesite,
                BlockId::AzaleaLeaves,
                BlockId::BambooBlock,
                BlockId::BambooMosaic,
                BlockId::BambooPlanks,
                BlockId::BlackConcrete,
                BlockId::BlackConcretePowder,
                BlockId::BlackGlazedTerracotta,
                BlockId::BlackStainedGlass,
                BlockId::BlackTerracotta,
                BlockId::BlackWool,
                BlockId::BlueConcrete,
                BlockId::BlueConcretePowder,
                BlockId::BlueGlazedTerracotta,
                BlockId::BlueIce,
                BlockId::BlueStainedGlass,
                BlockId::BlueTerracotta,
                BlockId::BlueWool,
                BlockId::BoneBlock,
                BlockId::BrainCoralBlock,
                BlockId::Bricks,
                BlockId::BrownConcrete,
                BlockId::BrownConcretePowder,
                BlockId::BrownGlazedTerracotta,
                BlockId::BrownMushroomBlock,
                BlockId::BrownStainedGlass,
                BlockId::BrownTerracotta,
                BlockId::BrownWool,
                BlockId::BubbleCoralBlock,
                BlockId::CherryLeaves,
                BlockId::CherryLog,
                BlockId::CherryPlanks,
                BlockId::ChiseledCinnabar,
                BlockId::ChiseledCopper,
                BlockId::ChiseledDeepslate,
                BlockId::ChiseledNetherBricks,
                BlockId::ChiseledPolishedBlackstone,
                BlockId::ChiseledQuartzBlock,
                BlockId::ChiseledRedSandstone,
                BlockId::ChiseledResinBricks,
                BlockId::ChiseledSandstone,
                BlockId::ChiseledStoneBricks,
                BlockId::ChiseledSulfur,
                BlockId::ChiseledTuff,
                BlockId::ChiseledTuffBricks,
                BlockId::Cinnabar,
                BlockId::CinnabarBricks,
                BlockId::Clay,
                BlockId::CoalBlock,
                BlockId::CoalOre,
                BlockId::CoarseDirt,
                BlockId::CobbledDeepslate,
                BlockId::Cobblestone,
                BlockId::CopperBlock,
                BlockId::CopperGrate,
                BlockId::CopperOre,
                BlockId::CrackedDeepslateBricks,
                BlockId::CrackedDeepslateTiles,
                BlockId::CrackedNetherBricks,
                BlockId::CrackedPolishedBlackstoneBricks,
                BlockId::CrackedStoneBricks,
                BlockId::CrimsonPlanks,
                BlockId::CrimsonStem,
                BlockId::CryingObsidian,
                BlockId::CutCopper,
                BlockId::CutRedSandstone,
                BlockId::CutSandstone,
                BlockId::CyanConcrete,
                BlockId::CyanConcretePowder,
                BlockId::CyanGlazedTerracotta,
                BlockId::CyanStainedGlass,
                BlockId::CyanTerracotta,
                BlockId::CyanWool,
                BlockId::DarkOakLeaves,
                BlockId::DarkOakLog,
                BlockId::DarkOakPlanks,
                BlockId::DarkPrismarine,
                BlockId::DeadBrainCoralBlock,
                BlockId::DeadBubbleCoralBlock,
                BlockId::DeadFireCoralBlock,
                BlockId::DeadHornCoralBlock,
                BlockId::DeadTubeCoralBlock,
                BlockId::Deepslate,
                BlockId::DeepslateBricks,
                BlockId::DeepslateCoalOre,
                BlockId::DeepslateCopperOre,
                BlockId::DeepslateDiamondOre,
                BlockId::DeepslateEmeraldOre,
                BlockId::DeepslateGoldOre,
                BlockId::DeepslateIronOre,
                BlockId::DeepslateLapisOre,
                BlockId::DeepslateRedstoneOre,
                BlockId::DeepslateTiles,
                BlockId::Diorite,
                BlockId::DriedKelp,
                BlockId::DripstoneBlock,
                BlockId::EmeraldBlock,
                BlockId::EmeraldOre,
                BlockId::EndStoneBricks,
                BlockId::ExposedChiseledCopper,
                BlockId::ExposedCopper,
                BlockId::ExposedCopperGrate,
                BlockId::ExposedCutCopper,
                BlockId::FireCoralBlock,
                BlockId::FloweringAzaleaLeaves,
                BlockId::GildedBlackstone,
                BlockId::Glass,
                BlockId::GoldBlock,
                BlockId::GoldOre,
                BlockId::Granite,
                BlockId::GrayConcrete,
                BlockId::GrayConcretePowder,
                BlockId::GrayGlazedTerracotta,
                BlockId::GrayStainedGlass,
                BlockId::GrayTerracotta,
                BlockId::GrayWool,
                BlockId::GreenConcrete,
                BlockId::GreenConcretePowder,
                BlockId::GreenGlazedTerracotta,
                BlockId::GreenStainedGlass,
                BlockId::GreenTerracotta,
                BlockId::GreenWool,
                BlockId::HayBlock,
                BlockId::HoneycombBlock,
                BlockId::HornCoralBlock,
                BlockId::Ice,
                BlockId::IronBlock,
                BlockId::IronOre,
                BlockId::JungleLeaves,
                BlockId::JungleLog,
                BlockId::JunglePlanks,
                BlockId::LapisBlock,
                BlockId::LapisOre,
                BlockId::LightBlueConcrete,
                BlockId::LightBlueConcretePowder,
                BlockId::LightBlueGlazedTerracotta,
                BlockId::LightBlueStainedGlass,
                BlockId::LightBlueTerracotta,
                BlockId::LightBlueWool,
                BlockId::LightGrayConcrete,
                BlockId::LightGrayConcretePowder,
                BlockId::LightGrayGlazedTerracotta,
                BlockId::LightGrayStainedGlass,
                BlockId::LightGrayTerracotta,
                BlockId::LightGrayWool,
                BlockId::LimeConcrete,
                BlockId::LimeConcretePowder,
                BlockId::LimeGlazedTerracotta,
                BlockId::LimeStainedGlass,
                BlockId::LimeTerracotta,
                BlockId::LimeWool,
                BlockId::MagentaConcrete,
                BlockId::MagentaConcretePowder,
                BlockId::MagentaGlazedTerracotta,
                BlockId::MagentaStainedGlass,
                BlockId::MagentaTerracotta,
                BlockId::MagentaWool,
                BlockId::MangroveLeaves,
                BlockId::MangroveLog,
                BlockId::MangrovePlanks,
                BlockId::Melon,
                BlockId::MossyCobblestone,
                BlockId::MossyStoneBricks,
                BlockId::Mud,
                BlockId::MudBricks,
                BlockId::MuddyMangroveRoots,
                BlockId::MushroomBlockInside,
                BlockId::MushroomStem,
                BlockId::Mycelium,
                BlockId::NetherBricks,
                BlockId::NetherGoldOre,
                BlockId::NetherQuartzOre,
                BlockId::NetherWartBlock,
                BlockId::NetheriteBlock,
                BlockId::OakPlanks,
                BlockId::OchreFroglight,
                BlockId::OrangeConcrete,
                BlockId::OrangeConcretePowder,
                BlockId::OrangeGlazedTerracotta,
                BlockId::OrangeStainedGlass,
                BlockId::OrangeTerracotta,
                BlockId::OrangeWool,
                BlockId::OxidizedChiseledCopper,
                BlockId::OxidizedCopper,
                BlockId::OxidizedCopperGrate,
                BlockId::OxidizedCutCopper,
                BlockId::PackedMud,
                BlockId::PaleMossBlock,
                BlockId::PaleOakLeaves,
                BlockId::PaleOakLog,
                BlockId::PaleOakPlanks,
                BlockId::PearlescentFroglight,
                BlockId::PinkConcrete,
                BlockId::PinkConcretePowder,
                BlockId::PinkGlazedTerracotta,
                BlockId::PinkStainedGlass,
                BlockId::PinkTerracotta,
                BlockId::PinkWool,
                BlockId::Podzol,
                BlockId::PolishedAndesite,
                BlockId::PolishedBasalt,
                BlockId::PolishedBlackstone,
                BlockId::PolishedBlackstoneBricks,
                BlockId::PolishedCinnabar,
                BlockId::PolishedDeepslate,
                BlockId::PolishedDiorite,
                BlockId::PolishedGranite,
                BlockId::PolishedSulfur,
                BlockId::PolishedTuff,
                BlockId::PotentSulfur,
                BlockId::Prismarine,
                BlockId::PrismarineBricks,
                BlockId::Pumpkin,
                BlockId::PurpleConcrete,
                BlockId::PurpleConcretePowder,
                BlockId::PurpleGlazedTerracotta,
                BlockId::PurpleStainedGlass,
                BlockId::PurpleTerracotta,
                BlockId::PurpleWool,
                BlockId::PurpurBlock,
                BlockId::PurpurPillar,
                BlockId::QuartzBlock,
                BlockId::QuartzBricks,
                BlockId::QuartzPillar,
                BlockId::RawCopperBlock,
                BlockId::RawGoldBlock,
                BlockId::RawIronBlock,
                BlockId::RedConcrete,
                BlockId::RedConcretePowder,
                BlockId::RedGlazedTerracotta,
                BlockId::RedMushroomBlock,
                BlockId::RedNetherBricks,
                BlockId::RedSandstone,
                BlockId::RedStainedGlass,
                BlockId::RedTerracotta,
                BlockId::RedWool,
                BlockId::RedstoneOre,
                BlockId::ReinforcedDeepslate,
                BlockId::ResinBlock,
                BlockId::ResinBricks,
                BlockId::RootedDirt,
                BlockId::Sandstone,
                BlockId::Sculk,
                BlockId::SeaLantern,
                BlockId::Shroomlight,
                BlockId::SmoothBasalt,
                BlockId::SmoothQuartz,
                BlockId::SmoothRedSandstone,
                BlockId::SmoothSandstone,
                BlockId::SmoothStone,
                BlockId::SpruceLeaves,
                BlockId::SpruceLog,
                BlockId::SprucePlanks,
                BlockId::StoneBricks,
                BlockId::StrippedAcaciaLog,
                BlockId::StrippedBambooBlock,
                BlockId::StrippedBirchLog,
                BlockId::StrippedCherryLog,
                BlockId::StrippedCrimsonStem,
                BlockId::StrippedDarkOakLog,
                BlockId::StrippedJungleLog,
                BlockId::StrippedMangroveLog,
                BlockId::StrippedOakLog,
                BlockId::StrippedPaleOakLog,
                BlockId::StrippedSpruceLog,
                BlockId::StrippedWarpedStem,
                BlockId::Sulfur,
                BlockId::SulfurBricks,
                BlockId::TintedGlass,
                BlockId::TubeCoralBlock,
                BlockId::Tuff,
                BlockId::TuffBricks,
                BlockId::VerdantFroglight,
                BlockId::WarpedPlanks,
                BlockId::WarpedStem,
                BlockId::WarpedWartBlock,
                BlockId::WeatheredChiseledCopper,
                BlockId::WeatheredCopper,
                BlockId::WeatheredCopperGrate,
                BlockId::WeatheredCutCopper,
                BlockId::WhiteConcrete,
                BlockId::WhiteConcretePowder,
                BlockId::WhiteGlazedTerracotta,
                BlockId::WhiteStainedGlass,
                BlockId::WhiteTerracotta,
                BlockId::WhiteWool,
                BlockId::YellowConcrete,
                BlockId::YellowConcretePowder,
                BlockId::YellowGlazedTerracotta,
                BlockId::YellowStainedGlass,
                BlockId::YellowTerracotta,
                BlockId::YellowWool,
                BlockId::CrimsonNylium,
                BlockId::WarpedNylium,
                BlockId::Moss,
                BlockId::Calcite
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParkourBlock {
    /// Nominal path position. Frame animation never feeds back into route
    /// generation, so the original generator remains deterministic and valid.
    pub position: BlockPos,
    pub block: BlockId,
    pub frame: ParkourFramePlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParkourBlockEdit {
    pub position: VoxelBlockAddress,
    pub block: BlockId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParkourFrameEdit {
    Spawn(ParkourFramePlan),
    Remove(VoxelFrameId),
}

#[derive(Debug, Clone)]
pub struct ParkourUpdate {
    pub edits: Vec<ParkourBlockEdit>,
    pub frame_edits: Vec<ParkourFrameEdit>,
    pub teleport: Option<[f32; 3]>,
    pub score_changed: bool,
    pub score: i32,
    pub combo: i32,
    /// Final score of a run that ended in this update. Persistence adapters
    /// can use this boundary without coupling themselves to score progress.
    pub completed_score: Option<i32>,
}

impl ParkourUpdate {
    fn unchanged(score: i32, combo: i32) -> Self {
        Self {
            edits: Vec::new(),
            frame_edits: Vec::new(),
            teleport: None,
            score_changed: false,
            score,
            combo,
            completed_score: None,
        }
    }
}

/// Pure parkour state attached by the server orchestrator to a player scope.
///
/// This library computes game rules only. It does not know about networking,
/// audiences, chunk providers, player registries or scope lifecycle.
#[derive(Component, Debug, Clone)]
pub struct ParkourRun {
    blocks: VecDeque<ParkourBlock>,
    score: i32,
    combo: i32,
    last_progress_seconds: f64,
    awaiting_respawn: bool,
    rng: DeterministicRng,
    obstacle_planner: obstacles::ParkourObstaclePlanner,
}

impl ParkourRun {
    pub fn new(seed: u64) -> Self {
        Self {
            blocks: VecDeque::new(),
            score: 0,
            combo: 0,
            last_progress_seconds: 0.0,
            awaiting_respawn: false,
            rng: DeterministicRng::new(seed),
            obstacle_planner: obstacles::ParkourObstaclePlanner::new(seed),
        }
    }

    pub fn blocks(&self) -> &VecDeque<ParkourBlock> {
        &self.blocks
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn combo(&self) -> i32 {
        self.combo
    }

    pub fn reset(&mut self, config: &ParkourConfig, now_seconds: f64) -> ParkourUpdate {
        let mut edits = Vec::with_capacity(self.blocks.len() + config.initial_block_count);
        let mut frame_edits = Vec::with_capacity(self.blocks.len() + config.initial_block_count);
        for block in self.blocks.drain(..) {
            edits.push(ParkourBlockEdit {
                position: VoxelBlockAddress::new(block.frame.id, BlockPos::new(0, 0, 0)),
                block: BlockId::Air,
            });
            frame_edits.push(ParkourFrameEdit::Remove(block.frame.id));
        }
        self.score = 0;
        self.combo = 0;
        self.last_progress_seconds = now_seconds;
        self.awaiting_respawn = false;

        let first = self.make_block(config, config.start, config.start, true);
        edits.push(ParkourBlockEdit {
            position: VoxelBlockAddress::new(first.frame.id, BlockPos::new(0, 0, 0)),
            block: first.block,
        });
        frame_edits.push(ParkourFrameEdit::Spawn(first.frame.clone()));
        self.blocks.push_back(first);
        for _ in 1..config.initial_block_count.max(1) {
            let (edit, frame_edit) = self.append_next(config);
            edits.push(edit);
            frame_edits.push(frame_edit);
        }

        ParkourUpdate {
            edits,
            frame_edits,
            teleport: Some([
                config.start.x as f32 + 0.5,
                config.start.y as f32 + 10.0,
                config.start.z as f32 + 0.5,
            ]),
            score_changed: true,
            score: self.score,
            combo: self.combo,
            completed_score: None,
        }
    }

    pub fn observe_position(
        &mut self,
        config: &ParkourConfig,
        player_position: Vec3,
        now_seconds: f64,
    ) -> ParkourUpdate {
        self.observe_position_internal(config, player_position, now_seconds, None)
    }

    /// Frame-aware checkpoint recognition. The adapter supplies authoritative
    /// poses from the player's routed world; no network or registry policy is
    /// embedded in the parkour rules.
    pub fn observe_position_with_frames(
        &mut self,
        config: &ParkourConfig,
        player_position: Vec3,
        now_seconds: f64,
        mut frame_transform: impl FnMut(VoxelFrameId) -> Option<VoxelFrameTransform>,
    ) -> ParkourUpdate {
        self.observe_position_internal(
            config,
            player_position,
            now_seconds,
            Some(&mut frame_transform),
        )
    }

    fn observe_position_internal(
        &mut self,
        config: &ParkourConfig,
        player_position: Vec3,
        now_seconds: f64,
        mut frame_transform: Option<&mut dyn FnMut(VoxelFrameId) -> Option<VoxelFrameTransform>>,
    ) -> ParkourUpdate {
        if self.awaiting_respawn {
            // Movement packets produced before the teleport is applied may
            // continue to arrive for a few ticks. Do not reset and publish the
            // same zero score again until the player reaches the respawn area.
            if player_position.y < config.start.y as f32 + 1.0 {
                return ParkourUpdate::unchanged(self.score, self.combo);
            }
            self.awaiting_respawn = false;
        }

        if player_position.y < config.start.y as f32 - config.fall_reset_distance {
            let completed_score = self.score;
            let mut update = self.reset(config, now_seconds);
            update.completed_score = Some(completed_score);
            self.awaiting_respawn = true;
            return update;
        }

        let Some(index) = self.blocks.iter().position(|block| {
            if let Some(lookup) = frame_transform.as_mut() {
                let pose = lookup(block.frame.id).unwrap_or(block.frame.initial);
                player_is_on_frame_block(player_position, pose)
            } else {
                let under_player = BlockPos::new(
                    player_position.x.floor() as i32,
                    (player_position.y - 0.08).floor() as i32,
                    player_position.z.floor() as i32,
                );
                block.position == under_player
            }
        })
        else {
            return ParkourUpdate::unchanged(self.score, self.combo);
        };
        if index == 0 {
            return ParkourUpdate::unchanged(self.score, self.combo);
        }

        let elapsed = (now_seconds - self.last_progress_seconds).max(0.0);
        let maximum = index as f64 / 2.0_f64.powf(self.combo as f64 / 45.0);
        if elapsed < maximum {
            self.combo += index as i32;
        } else {
            // A successful landing starts a new combo even when the previous
            // combo expired. Therefore advancing by one block reports combo 1
            // instead of combo 0.
            self.combo = index as i32;
        }

        let mut edits = Vec::with_capacity(index * 2);
        let mut frame_edits = Vec::with_capacity(index * 2);
        for _ in 0..index {
            if let Some(removed) = self.blocks.pop_front() {
                edits.push(ParkourBlockEdit {
                    position: VoxelBlockAddress::new(
                        removed.frame.id,
                        BlockPos::new(0, 0, 0),
                    ),
                    block: BlockId::Air,
                });
                frame_edits.push(ParkourFrameEdit::Remove(removed.frame.id));
                self.score += 1;
            }
            let (edit, frame_edit) = self.append_next(config);
            edits.push(edit);
            frame_edits.push(frame_edit);
        }
        self.last_progress_seconds = now_seconds;
        ParkourUpdate {
            edits,
            frame_edits,
            teleport: None,
            score_changed: true,
            score: self.score,
            combo: self.combo,
            completed_score: None,
        }
    }

    fn append_next(&mut self, config: &ParkourConfig) -> (ParkourBlockEdit, ParkourFrameEdit) {
        let previous = self
            .blocks
            .back()
            .map(|block| block.position)
            .unwrap_or(config.start);
        let y = self.rng.range_i32(-1, 1);
        let z = if y == 1 {
            self.rng.range_i32(1, 2)
        } else {
            self.rng.range_i32(2, 4)
        };
        let position = BlockPos::new(
            previous.x + self.rng.range_i32(-3, 3),
            previous.y + y,
            previous.z + z,
        );
        // Keep all nominal path RNG calls before the independent obstacle
        // planner. Adding frame behavior therefore cannot change route shape.
        let block = self.make_block(config, previous, position, false);
        let frame = block.frame.clone();
        self.blocks.push_back(block);
        (
            ParkourBlockEdit {
                position: VoxelBlockAddress::new(frame.id, BlockPos::new(0, 0, 0)),
                block: self.blocks.back().expect("parkour block was inserted").block,
            },
            ParkourFrameEdit::Spawn(frame),
        )
    }

    fn make_block(
        &mut self,
        config: &ParkourConfig,
        previous: BlockPos,
        position: BlockPos,
        force_normal: bool,
    ) -> ParkourBlock {
        let block = self.random_block(config);
        let frame = self
            .obstacle_planner
            .plan(previous, position, self.score, force_normal);
        ParkourBlock {
            position,
            block,
            frame,
        }
    }

    fn random_block(&mut self, config: &ParkourConfig) -> BlockId {
        if config.block_palette.is_empty() {
            return BlockId::Stone;
        }
        config.block_palette[self.rng.index(config.block_palette.len())]
    }
}

#[derive(Debug, Clone, Copy)]
struct DeterministicRng(u64);

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }

    fn next(&mut self) -> u64 {
        let mut value = self.0;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.0 = value;
        value
    }

    fn index(&mut self, length: usize) -> usize {
        (self.next() % length as u64) as usize
    }

    fn range_i32(&mut self, minimum: i32, maximum: i32) -> i32 {
        minimum + (self.next() % (maximum - minimum + 1) as u64) as i32
    }

    fn next_f32(&mut self) -> f32 {
        let value = self.next() >> 40;
        value as f32 / ((1_u32 << 24) - 1) as f32
    }

    fn range_f32(&mut self, minimum: f32, maximum: f32) -> f32 {
        minimum + (maximum - minimum) * self.next_f32()
    }
}

fn player_is_on_frame_block(player_position: Vec3, transform: VoxelFrameTransform) -> bool {
    let local = transform.world_to_local(player_position.as_dvec3());
    // Player positions represent the foot center. A small horizontal margin
    // tolerates solver skin and edge landings, while the narrow local-height
    // band avoids recognizing side/underside contacts as checkpoints.
    (-0.18..=1.18).contains(&local.x)
        && (-0.18..=1.18).contains(&local.z)
        && (0.82..=1.30).contains(&local.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_builds_a_private_course_and_returns_a_spawn() {
        let mut run = ParkourRun::new(42);
        let config = ParkourConfig::default();
        let update = run.reset(&config, 1.0);
        assert_eq!(run.blocks().len(), config.initial_block_count);
        assert_eq!(update.teleport, Some([0.5, 42.0, 0.5]));
        assert_eq!(update.edits.len(), config.initial_block_count);
    }

    #[test]
    fn landing_ahead_advances_the_course() {
        let mut run = ParkourRun::new(42);
        let config = ParkourConfig::default();
        run.reset(&config, 1.0);
        let target = run.blocks()[1].position;
        let update = run.observe_position(
            &config,
            Vec3::new(
                target.x as f32 + 0.5,
                target.y as f32 + 1.0,
                target.z as f32 + 0.5,
            ),
            1.2,
        );
        assert!(update.score_changed);
        assert_eq!(update.score, 1);
        assert_eq!(update.combo, 1);
        assert_eq!(run.blocks().len(), config.initial_block_count);
    }

    #[test]
    fn landing_uses_the_current_frame_pose_instead_of_the_nominal_block() {
        let mut run = ParkourRun::new(42);
        let config = ParkourConfig::default();
        run.reset(&config, 1.0);
        let target = run.blocks()[1].clone();
        let center = bevy::math::DVec3::new(
            target.position.x as f64 + 2.0,
            target.position.y as f64 + 0.5,
            target.position.z as f64 + 0.5,
        );
        let pose = centered_block_pose(
            center,
            bevy::math::DQuat::from_rotation_z(8.0_f64.to_radians()),
        );
        let foot = pose
            .local_to_world(bevy::math::DVec3::new(0.5, 1.0, 0.5))
            .as_vec3();
        let update = run.observe_position_with_frames(&config, foot, 1.2, |frame| {
            (frame == target.frame.id).then_some(pose)
        });
        assert!(update.score_changed);
        assert_eq!(update.score, 1);
    }

    #[test]
    fn falling_only_resets_once_until_the_respawn_arrives() {
        let mut run = ParkourRun::new(42);
        let config = ParkourConfig::default();
        run.reset(&config, 1.0);
        let fallen = Vec3::new(0.5, -1.0, 0.5);

        let first = run.observe_position(&config, fallen, 2.0);
        assert!(first.score_changed);
        assert!(first.teleport.is_some());
        assert_eq!(first.completed_score, Some(0));

        let duplicate = run.observe_position(&config, fallen, 2.1);
        assert!(!duplicate.score_changed);
        assert!(duplicate.teleport.is_none());

        run.observe_position(
            &config,
            Vec3::new(0.5, config.start.y as f32 + 10.0, 0.5),
            2.2,
        );
        let next_fall = run.observe_position(&config, fallen, 3.0);
        assert!(next_fall.score_changed);
        assert!(next_fall.teleport.is_some());
    }

    #[test]
    fn a_finished_run_reports_its_score_only_when_the_player_falls() {
        let mut run = ParkourRun::new(42);
        let config = ParkourConfig::default();
        run.reset(&config, 1.0);
        let target = run.blocks()[1].position;
        let checkpoint = run.observe_position(
            &config,
            Vec3::new(
                target.x as f32 + 0.5,
                target.y as f32 + 1.0,
                target.z as f32 + 0.5,
            ),
            1.2,
        );
        assert_eq!(checkpoint.score, 1);
        assert_eq!(checkpoint.completed_score, None);

        let finished = run.observe_position(&config, Vec3::new(0.5, -1.0, 0.5), 2.0);
        assert_eq!(finished.completed_score, Some(1));
        assert_eq!(finished.score, 0);
    }
}
