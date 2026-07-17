use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi, InGameOverlayState};
use client_player_controller_api::{
    Grounded, Player, PlayerControllerApi, PlayerControllerSet, PlayerPlanarMovementIntent,
    PlayerVelocity,
};
use player_gravity_api::{Gravity, PlayerGravityApi};
use tokio::task::JoinHandle;

const MINECRAFT_AIR_ACCELERATION_PER_TICK: f32 = 0.4;

#[derive(Resource, Debug, Clone, Copy)]
pub struct VanillaInertiaConfig {
    pub reference_speed: f32,
    pub air_drag_per_tick: f32,
    pub ground_drag_per_tick: f32,
}

impl Default for VanillaInertiaConfig {
    fn default() -> Self {
        Self {
            reference_speed: MINECRAFT_AIR_ACCELERATION_PER_TICK / (1.0 - 0.91),
            air_drag_per_tick: 0.91,
            // Preserve deliberate air inertia while making normal ground
            // movement stop and reverse much faster.
            ground_drag_per_tick: 0.25,
        }
    }
}

pub struct ClientPlayerInertiaVanillaMod;

impl ClientPlayerInertiaVanillaMod {
    pub fn init<P: PlayerControllerApi, G: PlayerGravityApi, S: GameStateApi>(
        bevy: &mut BevyMod,
        _player: &mut P,
        _gravity: &mut G,
        _game_state: &mut S,
    ) -> Self {
        bevy.app
            .init_resource::<VanillaInertiaConfig>()
            .add_systems(
                FixedUpdate,
                accelerate_planar_velocity
                    .in_set(PlayerControllerSet::ApplyMovementIntent)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                apply_planar_drag
                    .in_set(PlayerControllerSet::PostMovement)
                    .run_if(in_state(GameState::InGame)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn accelerate_planar_velocity(
    overlay: Res<State<InGameOverlayState>>,
    intent: Res<PlayerPlanarMovementIntent>,
    config: Res<VanillaInertiaConfig>,
    mut players: Query<(&mut PlayerVelocity, &Grounded), With<Player>>,
) {
    if *overlay.get() != InGameOverlayState::Playing || intent.direction.length_squared() == 0.0 {
        return;
    }
    let requested_speed = (intent.target_speed * intent.speed_multiplier).max(0.0);
    let speed_ratio = requested_speed / config.reference_speed.max(f32::EPSILON);
    for (mut velocity, grounded) in &mut players {
        let drag = if grounded.0 {
            config.ground_drag_per_tick
        } else {
            config.air_drag_per_tick
        }
        .clamp(0.0, 1.0);
        let acceleration = acceleration_per_tick(requested_speed, grounded.0, drag, speed_ratio);
        velocity.0 += intent.direction * acceleration;
    }
}

fn acceleration_per_tick(requested_speed: f32, grounded: bool, drag: f32, speed_ratio: f32) -> f32 {
    if grounded {
        requested_speed * (1.0 - drag)
    } else {
        MINECRAFT_AIR_ACCELERATION_PER_TICK * speed_ratio
    }
}

fn apply_planar_drag(
    config: Res<VanillaInertiaConfig>,
    gravity: Res<Gravity>,
    mut players: Query<(&mut PlayerVelocity, &Grounded), With<Player>>,
) {
    let up = gravity.up();
    for (mut velocity, grounded) in &mut players {
        let drag = if grounded.0 {
            config.ground_drag_per_tick
        } else {
            config.air_drag_per_tick
        }
        .clamp(0.0, 1.0);
        let vertical = up * velocity.0.dot(up);
        velocity.0 = vertical + (velocity.0 - vertical) * drag;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vanilla_acceleration_and_drag_converge_to_requested_speed() {
        let config = VanillaInertiaConfig::default();
        for grounded in [true, false] {
            let drag = if grounded {
                config.ground_drag_per_tick
            } else {
                config.air_drag_per_tick
            };
            let requested = 5.0;
            let ratio = requested / config.reference_speed;
            let acceleration = acceleration_per_tick(requested, grounded, drag, ratio);
            let mut velocity_after_drag = 0.0;
            let mut velocity_during_movement = 0.0;
            for _ in 0..200 {
                velocity_during_movement = velocity_after_drag + acceleration;
                velocity_after_drag = velocity_during_movement * drag;
            }
            assert!((velocity_during_movement - requested).abs() < 0.001);
        }
    }
}
