use bevy::prelude::Vec3;

use thecrown_protocol::GameMode;

#[derive(Debug, Clone, Copy)]
pub enum InstanceScalePolicy {
    RandomPerAdmission { min: f32, max: f32 },
    Fixed(f32),
}

#[derive(Debug, Clone, Copy)]
pub struct InstancePlayerPolicy {
    pub gravity: Vec3,
    pub scale: InstanceScalePolicy,
    pub flight_enabled: bool,
}

impl InstancePlayerPolicy {
    pub fn for_mode(mode: GameMode) -> Self {
        match mode {
            GameMode::Hub => Self {
                gravity: Vec3::new(0.0, -5.0, 0.0),
                scale: InstanceScalePolicy::RandomPerAdmission { min: 0.4, max: 5.0 },
                flight_enabled: false,
            },
            GameMode::Parkour => Self {
                gravity: Vec3::new(0.0, -20.0, 0.0),
                scale: InstanceScalePolicy::Fixed(1.0),
                flight_enabled: false,
            },
        }
    }

    pub fn scale_for_admission(self, admission_nonce: u64) -> f32 {
        match self.scale {
            InstanceScalePolicy::RandomPerAdmission { min, max } => {
                let random = splitmix64(admission_nonce);
                let normalized = random as f64 / u64::MAX as f64;
                min + (max - min) * normalized as f32
            }
            InstanceScalePolicy::Fixed(scale) => scale,
        }
    }
}

fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hub_scale_is_sampled_again_for_each_admission() {
        let policy = InstancePlayerPolicy::for_mode(GameMode::Hub);
        assert_ne!(policy.scale_for_admission(10), policy.scale_for_admission(11));
    }

    #[test]
    fn parkour_scale_is_always_one() {
        let policy = InstancePlayerPolicy::for_mode(GameMode::Parkour);
        assert_eq!(policy.scale_for_admission(10), 1.0);
        assert_eq!(policy.scale_for_admission(11), 1.0);
    }
}
