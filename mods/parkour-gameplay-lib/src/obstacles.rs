use bevy::math::{DQuat, DVec3};
use voxel_frame_api::{VoxelFrameId, VoxelFrameTransform};
use voxel_frame_movement_lib::{Easing, FrameMovement, RepeatMode};
use voxel_math_api::BlockPos;

use crate::DeterministicRng;

const FRAME_SEED_SALT: u64 = 0xa076_1d64_78bd_642f;
const CENTER: DVec3 = DVec3::splat(0.5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParkourObstacleBehavior {
    Normal,
    StaticTilt,
    Slider,
    Rotating,
    SliderTilted,
    SliderRotating,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParkourFramePlan {
    pub id: VoxelFrameId,
    pub behavior: ParkourObstacleBehavior,
    pub initial: VoxelFrameTransform,
    pub target: Option<VoxelFrameTransform>,
    pub movement: FrameMovement,
    /// Elapsed animation time at creation. This deterministically offsets the
    /// initial phase without changing the nominal course RNG stream.
    pub initial_elapsed_seconds: f64,
    pub estimated_difficulty: f32,
}

impl ParkourFramePlan {
    pub fn is_animated(&self) -> bool {
        self.target.is_some() && matches!(self.movement, FrameMovement::Animated { .. })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ParkourObstaclePlanner {
    seed: u64,
    serial: u64,
    rng: DeterministicRng,
    difficult_streak: u8,
    recovery_in: u8,
    previous_was_very_difficult: bool,
}

impl ParkourObstaclePlanner {
    pub(crate) fn new(seed: u64) -> Self {
        let mut rng = DeterministicRng::new(seed ^ FRAME_SEED_SALT);
        let recovery_in = rng.range_i32(4, 7) as u8;
        Self {
            seed,
            serial: 0,
            rng,
            difficult_streak: 0,
            recovery_in,
            previous_was_very_difficult: false,
        }
    }

    pub(crate) fn plan(
        &mut self,
        previous: BlockPos,
        current: BlockPos,
        score: i32,
        force_normal: bool,
    ) -> ParkourFramePlan {
        let id = deterministic_frame_id(self.seed, self.serial);
        self.serial = self.serial.wrapping_add(1);

        let progression = score_difficulty(score);
        let jump_difficulty = nominal_jump_difficulty(previous, current);
        let total_budget = 0.48 + 1.12 * progression;
        let accessory_budget = (total_budget - jump_difficulty).max(0.0);
        let forced_recovery = force_normal
            || self.recovery_in == 0
            || self.difficult_streak >= 3
            || self.previous_was_very_difficult;

        if forced_recovery {
            self.recovery_in = self.rng.range_i32(4, 7) as u8;
        } else {
            self.recovery_in = self.recovery_in.saturating_sub(1);
        }

        let mut behavior = if forced_recovery {
            ParkourObstacleBehavior::Normal
        } else {
            self.choose_behavior(progression, accessory_budget)
        };

        let mut static_tilt = 0.0_f32;
        let mut slide_amplitude = 0.0_f32;
        let mut rotation_amplitude = 0.0_f32;

        if matches!(
            behavior,
            ParkourObstacleBehavior::StaticTilt | ParkourObstacleBehavior::SliderTilted
        ) {
            let maximum = 6.0 + 24.0 * progression;
            static_tilt = self.rng.range_f32(3.0, maximum).to_radians();
        }
        if matches!(
            behavior,
            ParkourObstacleBehavior::Slider
                | ParkourObstacleBehavior::SliderTilted
                | ParkourObstacleBehavior::SliderRotating
        ) {
            slide_amplitude = 0.5
                + 1.5 * progression * self.rng.range_f32(0.65, 1.0);
        }
        if matches!(
            behavior,
            ParkourObstacleBehavior::Rotating | ParkourObstacleBehavior::SliderRotating
        ) {
            rotation_amplitude = self
                .rng
                .range_f32(3.0, 4.0 + 8.0 * progression)
                .to_radians();
        }

        let slide_axis = self.slide_axis(progression);
        if slide_axis == DVec3::Y {
            // Vertical motion is intentionally rare and stays much smaller
            // than horizontal travel even at maximum progression.
            slide_amplitude = slide_amplitude.min(0.5 + 0.5 * progression);
        }

        // Transform complexity spends only the budget left by the nominal
        // jump. This prevents a long, high jump from also receiving maximum
        // tilt, travel and rotation merely because the score is high.
        let requested_accessory = static_tilt.to_degrees() / 30.0 * 0.28
            + slide_amplitude / 2.0 * 0.42
            + rotation_amplitude.to_degrees() / 12.0 * 0.30;
        let budget_scale = if requested_accessory > 0.0 {
            (accessory_budget / requested_accessory).clamp(0.0, 1.0)
        } else {
            1.0
        };
        static_tilt *= budget_scale;
        slide_amplitude *= budget_scale;
        rotation_amplitude *= budget_scale;

        if (matches!(
            behavior,
            ParkourObstacleBehavior::Slider
                | ParkourObstacleBehavior::SliderTilted
                | ParkourObstacleBehavior::SliderRotating
        ) && slide_amplitude < 0.2)
            || (matches!(
                behavior,
                ParkourObstacleBehavior::Rotating | ParkourObstacleBehavior::SliderRotating
            ) && rotation_amplitude.to_degrees() < 1.0)
        {
            behavior = ParkourObstacleBehavior::Normal;
            static_tilt = 0.0;
            slide_amplitude = 0.0;
            rotation_amplitude = 0.0;
        }

        let tilt_axis = self.horizontal_axis();
        let rotation_axis = self.horizontal_axis();
        let static_rotation = DQuat::from_axis_angle(tilt_axis, static_tilt as f64);
        let dynamic_start = DQuat::from_axis_angle(rotation_axis, -(rotation_amplitude as f64));
        let dynamic_target = DQuat::from_axis_angle(rotation_axis, rotation_amplitude as f64);
        let start_rotation = (static_rotation * dynamic_start).normalize();
        let target_rotation = (static_rotation * dynamic_target).normalize();
        let nominal_center = DVec3::new(
            current.x as f64 + 0.5,
            current.y as f64 + 0.5,
            current.z as f64 + 0.5,
        );
        let start_offset = slide_axis * -(slide_amplitude as f64);
        let target_offset = slide_axis * slide_amplitude as f64;
        let initial = centered_block_pose(nominal_center + start_offset, start_rotation);

        let animated = matches!(
            behavior,
            ParkourObstacleBehavior::Slider
                | ParkourObstacleBehavior::Rotating
                | ParkourObstacleBehavior::SliderTilted
                | ParkourObstacleBehavior::SliderRotating
        );
        let (target, movement, initial_elapsed_seconds) = if animated {
            let minimum_seconds = 2.5 - progression;
            let maximum_seconds = 4.0 - progression;
            let duration_seconds = self.rng.range_f32(minimum_seconds, maximum_seconds);
            let movement = FrameMovement::Animated {
                duration_ms: (duration_seconds * 1000.0).round() as u32,
                translation_easing: ease_in_out(),
                rotation_easing: ease_in_out(),
                repeat: RepeatMode::PingPong,
            };
            // PingPong has a two-leg period. Keeping this separate from the
            // route RNG makes seed -> nominal path stable after this feature.
            let phase = self.rng.next_f32() as f64;
            (
                Some(centered_block_pose(
                    nominal_center + target_offset,
                    target_rotation,
                )),
                movement,
                phase * duration_seconds as f64 * 2.0,
            )
        } else {
            (None, FrameMovement::Instant, 0.0)
        };

        let estimated_difficulty = jump_difficulty + requested_accessory * budget_scale;
        let hard_threshold = total_budget * 0.72;
        let very_hard_threshold = total_budget * 0.90;
        if estimated_difficulty >= hard_threshold && behavior != ParkourObstacleBehavior::Normal {
            self.difficult_streak = self.difficult_streak.saturating_add(1);
        } else {
            self.difficult_streak = 0;
        }
        self.previous_was_very_difficult = estimated_difficulty >= very_hard_threshold
            && behavior != ParkourObstacleBehavior::Normal;

        ParkourFramePlan {
            id,
            behavior,
            initial,
            target,
            movement,
            initial_elapsed_seconds,
            estimated_difficulty,
        }
    }

    fn choose_behavior(
        &mut self,
        difficulty: f32,
        accessory_budget: f32,
    ) -> ParkourObstacleBehavior {
        let d2 = difficulty * difficulty;
        let candidates = [
            (ParkourObstacleBehavior::Normal, 0.96 - 0.71 * difficulty, 0.0),
            (ParkourObstacleBehavior::StaticTilt, 0.04 + 0.12 * difficulty, 0.04),
            (ParkourObstacleBehavior::Slider, 0.18 * difficulty, 0.18),
            (ParkourObstacleBehavior::Rotating, 0.13 * d2.sqrt(), 0.14),
            (ParkourObstacleBehavior::SliderTilted, 0.14 * d2, 0.30),
            (ParkourObstacleBehavior::SliderRotating, 0.14 * d2, 0.34),
        ];
        let total = candidates
            .iter()
            .filter(|(_, _, minimum)| accessory_budget >= *minimum)
            .map(|(_, weight, _)| weight.max(0.0))
            .sum::<f32>();
        let mut selected = self.rng.next_f32() * total.max(f32::EPSILON);
        for (behavior, weight, minimum) in candidates {
            if accessory_budget < minimum {
                continue;
            }
            selected -= weight.max(0.0);
            if selected <= 0.0 {
                return behavior;
            }
        }
        ParkourObstacleBehavior::Normal
    }

    fn horizontal_axis(&mut self) -> DVec3 {
        let sign = if self.rng.next() & 1 == 0 { 1.0 } else { -1.0 };
        if self.rng.next() & 1 == 0 {
            DVec3::X * sign
        } else {
            DVec3::Z * sign
        }
    }

    fn slide_axis(&mut self, difficulty: f32) -> DVec3 {
        if self.rng.next_f32() < 0.08 * difficulty {
            return DVec3::Y;
        }
        if self.rng.next() & 1 == 0 {
            DVec3::X
        } else {
            DVec3::Z
        }
    }
}

pub fn score_difficulty(score: i32) -> f32 {
    let linear = ((score as f32 - 15.0) / (150.0 - 15.0)).clamp(0.0, 1.0);
    linear * linear * (3.0 - 2.0 * linear)
}

pub fn nominal_jump_difficulty(previous: BlockPos, current: BlockPos) -> f32 {
    let dx = (current.x - previous.x) as f32;
    let dy = (current.y - previous.y) as f32;
    let dz = (current.z - previous.z) as f32;
    let horizontal = (dx * dx + dz * dz).sqrt();
    let distance = (horizontal * horizontal + dy * dy).sqrt();
    ((horizontal - 1.0).max(0.0) / 4.0 * 0.46
        + dy.max(0.0) * 0.22
        + (distance - 2.0).max(0.0) / 4.0 * 0.20)
        .clamp(0.0, 1.1)
}

/// Builds a grid-frame pose whose local block at (0,0,0) rotates around its
/// center. The frame API uses corner-based voxel coordinates, so translation
/// compensates for the rotated local half-block.
pub fn centered_block_pose(center: DVec3, rotation: DQuat) -> VoxelFrameTransform {
    VoxelFrameTransform::new(
        (center - rotation * CENTER).to_array(),
        rotation.to_array(),
    )
    .expect("finite parkour obstacle transform")
}

fn ease_in_out() -> Easing {
    Easing::CubicBezier {
        x1: 0.42,
        y1: 0.0,
        x2: 0.58,
        y2: 1.0,
    }
}

fn deterministic_frame_id(seed: u64, serial: u64) -> VoxelFrameId {
    let high = mix64(seed ^ serial.wrapping_mul(0x9e37_79b9_7f4a_7c15));
    let low = mix64(seed.rotate_left(29) ^ serial ^ FRAME_SEED_SALT);
    let id = ((high as u128) << 64) | low as u128;
    VoxelFrameId::from_u128(id.max(1))
}

fn mix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difficulty_is_smooth_and_clamped() {
        assert_eq!(score_difficulty(15), 0.0);
        assert_eq!(score_difficulty(150), 1.0);
        assert_eq!(score_difficulty(10_000), 1.0);
        assert!(score_difficulty(80) > 0.0 && score_difficulty(80) < 1.0);
    }

    #[test]
    fn rotation_keeps_the_block_center_fixed() {
        let center = DVec3::new(4.5, 8.5, -2.5);
        let rotation = DQuat::from_rotation_z(0.42);
        let pose = centered_block_pose(center, rotation);
        assert!(pose.local_to_world(CENTER).distance(center) < 1e-10);
    }
}
