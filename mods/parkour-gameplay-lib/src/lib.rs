use bevy::prelude::*;
use generated_block_registry::BlockId;
use std::collections::VecDeque;
use voxel_math_api::BlockPos;

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
                BlockId::Stone,
                BlockId::Dirt,
                BlockId::Moss,
                BlockId::DiamondBlock,
                BlockId::Glowstone,
                BlockId::PackedIce,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParkourBlock {
    pub position: BlockPos,
    pub block: BlockId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParkourBlockEdit {
    pub position: BlockPos,
    pub block: BlockId,
}

#[derive(Debug, Clone)]
pub struct ParkourUpdate {
    pub edits: Vec<ParkourBlockEdit>,
    pub teleport: Option<[f32; 3]>,
    pub score_changed: bool,
    pub score: i32,
    pub combo: i32,
}

impl ParkourUpdate {
    fn unchanged(score: i32, combo: i32) -> Self {
        Self {
            edits: Vec::new(),
            teleport: None,
            score_changed: false,
            score,
            combo,
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
        let mut edits = self
            .blocks
            .drain(..)
            .map(|block| ParkourBlockEdit {
                position: block.position,
                block: BlockId::Air,
            })
            .collect::<Vec<_>>();
        self.score = 0;
        self.combo = 0;
        self.last_progress_seconds = now_seconds;
        self.awaiting_respawn = false;

        let first = ParkourBlock {
            position: config.start,
            block: self.random_block(config),
        };
        self.blocks.push_back(first);
        edits.push(ParkourBlockEdit {
            position: first.position,
            block: first.block,
        });
        for _ in 1..config.initial_block_count.max(1) {
            edits.push(self.append_next(config));
        }

        ParkourUpdate {
            edits,
            teleport: Some([
                config.start.x as f32 + 0.5,
                config.start.y as f32 + 10.0,
                config.start.z as f32 + 0.5,
            ]),
            score_changed: true,
            score: self.score,
            combo: self.combo,
        }
    }

    pub fn observe_position(
        &mut self,
        config: &ParkourConfig,
        player_position: Vec3,
        now_seconds: f64,
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
            let update = self.reset(config, now_seconds);
            self.awaiting_respawn = true;
            return update;
        }

        let under_player = BlockPos::new(
            player_position.x.floor() as i32,
            (player_position.y - 0.08).floor() as i32,
            player_position.z.floor() as i32,
        );
        let Some(index) = self
            .blocks
            .iter()
            .position(|block| block.position == under_player)
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
        for _ in 0..index {
            if let Some(removed) = self.blocks.pop_front() {
                edits.push(ParkourBlockEdit {
                    position: removed.position,
                    block: BlockId::Air,
                });
                self.score += 1;
            }
            edits.push(self.append_next(config));
        }
        self.last_progress_seconds = now_seconds;
        ParkourUpdate {
            edits,
            teleport: None,
            score_changed: true,
            score: self.score,
            combo: self.combo,
        }
    }

    fn append_next(&mut self, config: &ParkourConfig) -> ParkourBlockEdit {
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
        let block = ParkourBlock {
            position,
            block: self.random_block(config),
        };
        self.blocks.push_back(block);
        ParkourBlockEdit {
            position,
            block: block.block,
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
    fn falling_only_resets_once_until_the_respawn_arrives() {
        let mut run = ParkourRun::new(42);
        let config = ParkourConfig::default();
        run.reset(&config, 1.0);
        let fallen = Vec3::new(0.5, -1.0, 0.5);

        let first = run.observe_position(&config, fallen, 2.0);
        assert!(first.score_changed);
        assert!(first.teleport.is_some());

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
}
