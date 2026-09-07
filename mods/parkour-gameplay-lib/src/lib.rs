use bevy::prelude::*;
use generated_block_registry::BlockId;
use std::collections::VecDeque;
use voxel_frame_api::{VoxelBlockAddress, VoxelFrameId, VoxelFrameTransform};
use voxel_math_api::BlockPos;

mod obstacles;
pub use obstacles::{
    ParkourFramePlan, ParkourObstacleBehavior, centered_block_pose, nominal_jump_difficulty,
    score_difficulty,
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
                BlockId::Calcite,
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

enum ParkourLandingObservation<'a> {
    Position,
    FrameTransforms(&'a mut dyn FnMut(VoxelFrameId) -> Option<VoxelFrameTransform>),
    MovementWithFrames {
        previous_position: Vec3,
        frame_transform: &'a mut dyn FnMut(VoxelFrameId) -> Option<VoxelFrameTransform>,
    },
    SupportedFrame(Option<VoxelFrameId>),
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
        self.observe_position_internal(
            config,
            player_position,
            now_seconds,
            ParkourLandingObservation::Position,
        )
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
            ParkourLandingObservation::FrameTransforms(&mut frame_transform),
        )
    }

    /// Frame-aware landing recognition from the authoritative server movement.
    ///
    /// This deliberately does not depend on Grounded or surface-anchor state:
    /// checkpoint detection is a gameplay query against the current transformed
    /// top face of the known parkour blocks.
    pub fn observe_movement_with_frames(
        &mut self,
        config: &ParkourConfig,
        previous_position: Vec3,
        player_position: Vec3,
        now_seconds: f64,
        mut frame_transform: impl FnMut(VoxelFrameId) -> Option<VoxelFrameTransform>,
    ) -> ParkourUpdate {
        self.observe_position_internal(
            config,
            player_position,
            now_seconds,
            ParkourLandingObservation::MovementWithFrames {
                previous_position,
                frame_transform: &mut frame_transform,
            },
        )
    }

    /// Recognizes a checkpoint from authoritative collision support.
    ///
    /// `Some(frame)` means that frame is really supporting the player's
    /// character according to the server collision solver. `None` means the
    /// authoritative solver observed no frame support; proximity alone must not
    /// award a checkpoint in that case.
    pub fn observe_supported_frame(
        &mut self,
        config: &ParkourConfig,
        player_position: Vec3,
        now_seconds: f64,
        supported_frame: Option<VoxelFrameId>,
    ) -> ParkourUpdate {
        self.observe_position_internal(
            config,
            player_position,
            now_seconds,
            ParkourLandingObservation::SupportedFrame(supported_frame),
        )
    }

    fn observe_position_internal(
        &mut self,
        config: &ParkourConfig,
        player_position: Vec3,
        now_seconds: f64,
        observation: ParkourLandingObservation<'_>,
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

        let index = match observation {
            ParkourLandingObservation::SupportedFrame(supported_frame) => supported_frame
                .and_then(|frame| self.blocks.iter().position(|block| block.frame.id == frame)),
            ParkourLandingObservation::FrameTransforms(lookup) => {
                self.blocks
                    .iter()
                    .enumerate()
                    .skip(1)
                    .find_map(|(index, block)| {
                        let pose = lookup(block.frame.id).unwrap_or(block.frame.initial);
                        player_is_on_frame_block(player_position, pose).then_some(index)
                    })
            }
            ParkourLandingObservation::MovementWithFrames {
                previous_position,
                frame_transform,
            } => self
                .blocks
                .iter()
                .enumerate()
                .skip(1)
                .find_map(|(index, block)| {
                    let pose = frame_transform(block.frame.id).unwrap_or(block.frame.initial);
                    player_landed_on_frame_block(previous_position, player_position, pose)
                        .then_some(index)
                }),
            ParkourLandingObservation::Position => {
                let under_player = BlockPos::new(
                    player_position.x.floor() as i32,
                    (player_position.y - 0.08).floor() as i32,
                    player_position.z.floor() as i32,
                );
                self.blocks
                    .iter()
                    .position(|block| block.position == under_player)
            }
        };

        let Some(index) = index else {
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
                self.score += 400;
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
                block: self
                    .blocks
                    .back()
                    .expect("parkour block was inserted")
                    .block,
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

const PARKOUR_PLAYER_RADIUS: f32 = 0.3;
const PARKOUR_SUPPORT_ABOVE_TOLERANCE: f32 = 0.10;
const PARKOUR_SUPPORT_PENETRATION_TOLERANCE: f32 = 0.04;
const PARKOUR_FACE_EDGE_MARGIN: f64 = 0.18;
const PARKOUR_CHECKPOINT_HORIZONTAL_MARGIN: f64 = PARKOUR_PLAYER_RADIUS as f64 + 0.10;
const PARKOUR_CHECKPOINT_BELOW_TOP: f64 = 0.22;
const PARKOUR_CHECKPOINT_ABOVE_TOP: f64 = 0.42;

fn player_is_on_frame_block(player_position: Vec3, transform: VoxelFrameTransform) -> bool {
    capsule_bottom_sphere_supports_frame(player_position, transform)
        || player_overlaps_frame_checkpoint(player_position, transform)
}

/// Detects the parkour landing from the same geometry that matters to the
/// character controller: the player's bottom capsule sphere against the
/// transformed local +Y face of the one-block checkpoint.
///
/// `CharacterQuery::position` is the foot point. For a tilted surface that foot
/// point is NOT the contact point: the contact is shifted sideways by the
/// sphere radius along the surface normal. Previous versions tested/raycasted
/// from the foot point and therefore systematically misclassified tilted
/// landings.
fn player_landed_on_frame_block(
    previous_position: Vec3,
    player_position: Vec3,
    transform: VoxelFrameTransform,
) -> bool {
    if !previous_position.is_finite() || !player_position.is_finite() {
        return false;
    }

    if capsule_bottom_sphere_supports_frame(player_position, transform) {
        return true;
    }

    // Checkpoint policy is intentionally more tolerant than physical support.
    // A client can send the first accepted packet slightly above/below the
    // solver's narrow support skin, especially while the frame itself moves.
    // Test in frame-local space so translation and tilt remain exact.
    if (player_position - previous_position).y <= 0.35
        && player_overlaps_frame_checkpoint(player_position, transform)
    {
        return true;
    }

    // Swept fallback for the exact landing packet. Usually the resolved end
    // position already satisfies the support test above, but this also catches
    // a surface moving into the character or a packet whose endpoints straddle
    // the contact plane by a tiny amount.
    swept_bottom_sphere_hits_frame(previous_position, player_position, transform)
}

fn player_overlaps_frame_checkpoint(
    foot_position: Vec3,
    transform: VoxelFrameTransform,
) -> bool {
    if !foot_position.is_finite() {
        return false;
    }
    let local = transform.world_to_local(foot_position.as_dvec3());
    let horizontal = -PARKOUR_CHECKPOINT_HORIZONTAL_MARGIN
        ..=1.0 + PARKOUR_CHECKPOINT_HORIZONTAL_MARGIN;

    horizontal.contains(&local.x)
        && horizontal.contains(&local.z)
        && (1.0 - PARKOUR_CHECKPOINT_BELOW_TOP
            ..=1.0 + PARKOUR_CHECKPOINT_ABOVE_TOP)
            .contains(&local.y)
}

fn capsule_bottom_sphere_supports_frame(
    foot_position: Vec3,
    transform: VoxelFrameTransform,
) -> bool {
    let Some(face) = parkour_top_face(transform) else {
        return false;
    };

    let sphere_center = foot_position + Vec3::Y * PARKOUR_PLAYER_RADIUS;
    let plane_distance = (sphere_center - face.center).dot(face.normal);
    let separation = plane_distance - PARKOUR_PLAYER_RADIUS;

    if separation > PARKOUR_SUPPORT_ABOVE_TOLERANCE
        || separation < -PARKOUR_SUPPORT_PENETRATION_TOLERANCE
    {
        return false;
    }

    let contact_world = sphere_center - face.normal * plane_distance;
    contact_is_on_top_face(contact_world, transform)
}

fn swept_bottom_sphere_hits_frame(
    previous_foot: Vec3,
    current_foot: Vec3,
    transform: VoxelFrameTransform,
) -> bool {
    let Some(face) = parkour_top_face(transform) else {
        return false;
    };

    let previous_center = previous_foot + Vec3::Y * PARKOUR_PLAYER_RADIUS;
    let current_center = current_foot + Vec3::Y * PARKOUR_PLAYER_RADIUS;

    let previous_separation =
        (previous_center - face.center).dot(face.normal) - PARKOUR_PLAYER_RADIUS;
    let current_separation =
        (current_center - face.center).dot(face.normal) - PARKOUR_PLAYER_RADIUS;

    // Only a crossing toward the support plane is a landing. This avoids
    // scoring while jumping upward away from an obstacle.
    if previous_separation < current_separation
        || previous_separation < -PARKOUR_SUPPORT_PENETRATION_TOLERANCE
        || current_separation > PARKOUR_SUPPORT_ABOVE_TOLERANCE
    {
        return false;
    }

    let denominator = previous_separation - current_separation;
    if denominator.abs() <= 1e-6 {
        return false;
    }

    let t = (previous_separation / denominator).clamp(0.0, 1.0);
    let sphere_center = previous_center.lerp(current_center, t);
    let plane_distance = (sphere_center - face.center).dot(face.normal);
    let contact_world = sphere_center - face.normal * plane_distance;

    contact_is_on_top_face(contact_world, transform)
}

#[derive(Debug, Clone, Copy)]
struct ParkourTopFace {
    center: Vec3,
    normal: Vec3,
}

fn parkour_top_face(transform: VoxelFrameTransform) -> Option<ParkourTopFace> {
    let normal = transform
        .local_direction_to_world(bevy::math::DVec3::Y)
        .as_vec3()
        .normalize_or_zero();

    // TheCrown parkour gravity is fixed to world -Y and the character solver's
    // default walkable slope threshold is cos(45°).
    if normal.length_squared() <= 1e-8 || normal.dot(Vec3::Y) < 0.70710677 {
        return None;
    }

    Some(ParkourTopFace {
        center: transform
            .local_to_world(bevy::math::DVec3::new(0.5, 1.0, 0.5))
            .as_vec3(),
        normal,
    })
}

fn contact_is_on_top_face(contact_world: Vec3, transform: VoxelFrameTransform) -> bool {
    let local = transform.world_to_local(contact_world.as_dvec3());

    (-PARKOUR_FACE_EDGE_MARGIN..=1.0 + PARKOUR_FACE_EDGE_MARGIN).contains(&local.x)
        && (-PARKOUR_FACE_EDGE_MARGIN..=1.0 + PARKOUR_FACE_EDGE_MARGIN).contains(&local.z)
        && (local.y - 1.0).abs() <= 0.035
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
        assert_eq!(update.score, 400);
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
        assert_eq!(update.score, 400);
    }

    #[test]
    fn authoritative_support_advances_without_position_heuristics() {
        let mut run = ParkourRun::new(42);
        let config = ParkourConfig::default();
        run.reset(&config, 1.0);
        let target = run.blocks()[1].clone();

        // Deliberately not in the narrow local-Y band used by the old
        // frame-position heuristic. Collision support is the authority.
        let position = Vec3::new(
            target.position.x as f32 + 0.5,
            target.position.y as f32 + 0.2,
            target.position.z as f32 + 0.5,
        );
        let update =
            run.observe_supported_frame(&config, position, 1.2, Some(target.frame.id));
        assert!(update.score_changed);
        assert_eq!(update.score, 400);
    }

    #[test]
    fn authoritative_no_support_does_not_score_from_proximity() {
        let mut run = ParkourRun::new(42);
        let config = ParkourConfig::default();
        run.reset(&config, 1.0);
        let target = run.blocks()[1].clone();
        let pose = target.frame.initial;
        let position = pose
            .local_to_world(bevy::math::DVec3::new(0.5, 1.0, 0.5))
            .as_vec3();

        let update = run.observe_supported_frame(&config, position, 1.2, None);
        assert!(!update.score_changed);
        assert_eq!(update.score, 0);
    }

    #[test]
    fn movement_fallback_checks_future_frames_instead_of_the_current_checkpoint() {
        let mut run = ParkourRun::new(42);
        let config = ParkourConfig::default();
        run.reset(&config, 1.0);
        let target = run.blocks()[1].clone();
        let foot = target
            .frame
            .initial
            .local_to_world(bevy::math::DVec3::new(0.5, 1.0, 0.5))
            .as_vec3();

        let update = run.observe_movement_with_frames(
            &config,
            foot + Vec3::Y * 0.8,
            foot,
            1.2,
            |frame| (frame == target.frame.id).then_some(target.frame.initial),
        );

        assert!(update.score_changed);
        assert_eq!(update.score, 400);
    }

    #[test]
    fn capsule_center_landing_on_horizontal_frame_is_recognized() {
        let pose = centered_block_pose(
            bevy::math::DVec3::new(4.5, 8.5, -2.5),
            bevy::math::DQuat::IDENTITY,
        );
        let contact = pose
            .local_to_world(bevy::math::DVec3::new(0.5, 1.0, 0.5))
            .as_vec3();
        let normal = Vec3::Y;
        let sphere_center = contact + normal * PARKOUR_PLAYER_RADIUS;
        let foot = sphere_center - Vec3::Y * PARKOUR_PLAYER_RADIUS;

        assert!(player_landed_on_frame_block(
            foot + Vec3::Y * 0.8,
            foot,
            pose,
        ));
    }

    #[test]
    fn physically_correct_capsule_landing_on_tilted_center_is_recognized() {
        let pose = centered_block_pose(
            bevy::math::DVec3::new(4.5, 8.5, -2.5),
            bevy::math::DQuat::from_rotation_z(30.0_f64.to_radians()),
        );
        let contact = pose
            .local_to_world(bevy::math::DVec3::new(0.5, 1.0, 0.5))
            .as_vec3();
        let normal = pose
            .local_direction_to_world(bevy::math::DVec3::Y)
            .as_vec3()
            .normalize();

        // This is the actual foot-point geometry of the lower capsule sphere:
        // sphere center is radius units along the tilted surface normal, while
        // CharacterQuery::position is radius units below it along world up.
        let sphere_center = contact + normal * PARKOUR_PLAYER_RADIUS;
        let foot = sphere_center - Vec3::Y * PARKOUR_PLAYER_RADIUS;

        assert!(player_landed_on_frame_block(
            foot + Vec3::Y * 0.8,
            foot,
            pose,
        ));
    }

    #[test]
    fn tilted_landing_near_face_center_does_not_require_edge_contact() {
        let pose = centered_block_pose(
            bevy::math::DVec3::new(4.5, 8.5, -2.5),
            bevy::math::DQuat::from_rotation_x(22.0_f64.to_radians()),
        );
        let contact = pose
            .local_to_world(bevy::math::DVec3::new(0.45, 1.0, 0.55))
            .as_vec3();
        let normal = pose
            .local_direction_to_world(bevy::math::DVec3::Y)
            .as_vec3()
            .normalize();
        let foot = contact + normal * PARKOUR_PLAYER_RADIUS
            - Vec3::Y * PARKOUR_PLAYER_RADIUS;

        assert!(capsule_bottom_sphere_supports_frame(foot, pose));
    }

    #[test]
    fn checkpoint_volume_accepts_center_and_edge_packets_on_a_moved_tilted_frame() {
        let pose = centered_block_pose(
            bevy::math::DVec3::new(17.5, 11.5, -8.5),
            bevy::math::DQuat::from_rotation_z(28.0_f64.to_radians()),
        );
        let center = pose
            .local_to_world(bevy::math::DVec3::new(0.5, 1.24, 0.5))
            .as_vec3();
        let edge = pose
            .local_to_world(bevy::math::DVec3::new(-0.25, 1.08, 0.5))
            .as_vec3();

        assert!(player_overlaps_frame_checkpoint(center, pose));
        assert!(player_overlaps_frame_checkpoint(edge, pose));
    }

    #[test]
    fn capsule_beside_tilted_block_is_not_a_landing() {
        let pose = centered_block_pose(
            bevy::math::DVec3::new(4.5, 8.5, -2.5),
            bevy::math::DQuat::from_rotation_z(25.0_f64.to_radians()),
        );
        let contact = pose
            .local_to_world(bevy::math::DVec3::new(1.65, 1.0, 0.5))
            .as_vec3();
        let normal = pose
            .local_direction_to_world(bevy::math::DVec3::Y)
            .as_vec3()
            .normalize();
        let foot = contact + normal * PARKOUR_PLAYER_RADIUS
            - Vec3::Y * PARKOUR_PLAYER_RADIUS;

        assert!(!capsule_bottom_sphere_supports_frame(foot, pose));
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
        assert_eq!(checkpoint.score, 400);
        assert_eq!(checkpoint.completed_score, None);

        let finished = run.observe_position(&config, Vec3::new(0.5, -1.0, 0.5), 2.0);
        assert_eq!(finished.completed_score, Some(400));
        assert_eq!(finished.score, 0);
    }
}
