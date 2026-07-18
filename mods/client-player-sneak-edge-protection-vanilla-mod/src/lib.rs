use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_player_controller_api::{
    Grounded, Player, PlayerControllerApi, PlayerControllerSet, PlayerVelocity,
};
use collision_api::{CollisionApi, CollisionService};
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_hitbox_api::{PlayerHitbox, PlayerHitboxApi};
use player_sneak_api::{LocalPlayerSneak, PlayerSneakApi};
use tokio::task::JoinHandle;

const SUPPORT_PROBE_DISTANCE: f32 = 0.05;
const PATH_SAMPLES: usize = 8;
const BINARY_SEARCH_STEPS: usize = 8;

pub struct ClientPlayerSneakEdgeProtectionVanillaMod;

impl ClientPlayerSneakEdgeProtectionVanillaMod {
    pub fn init<
        G: GameStateApi,
        P: PlayerControllerApi,
        C: CollisionApi,
        H: PlayerHitboxApi,
        V: PlayerGravityApi,
        S: PlayerSneakApi,
    >(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _controller: &mut P,
        _collision: &mut C,
        _hitbox: &mut H,
        _gravity: &mut V,
        _sneak: &mut S,
    ) -> Self {
        bevy.app.add_systems(
            FixedUpdate,
            constrain_sneaking_movement
                .in_set(PlayerControllerSet::MovementConstraints)
                .run_if(in_state(GameState::InGame)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn constrain_sneaking_movement(
    time: Res<Time<Fixed>>,
    sneak: Res<LocalPlayerSneak>,
    gravity: Res<Gravity>,
    hitbox: Res<PlayerHitbox>,
    collision: Res<CollisionService>,
    mut players: Query<(&Transform, &Grounded, &mut PlayerVelocity), With<Player>>,
) {
    if !sneak.active || gravity.0.length_squared() == 0.0 {
        return;
    }
    let delta_seconds = time.delta_secs();
    if delta_seconds <= f32::EPSILON {
        return;
    }

    let down = gravity.direction();
    let up = gravity.up();
    let alignment = gravity.alignment();
    let first_axis = (alignment * Vec3::X).normalize_or_zero();
    let second_axis = (alignment * Vec3::Z).normalize_or_zero();

    for (transform, grounded, mut velocity) in &mut players {
        if !grounded.0 || !has_support(&collision, *hitbox, transform.translation, down) {
            continue;
        }

        let vertical_velocity = up * velocity.0.dot(up);
        let requested_delta = (velocity.0 - vertical_velocity) * delta_seconds;
        let first_delta = first_axis * requested_delta.dot(first_axis);
        let second_delta = second_axis * requested_delta.dot(second_axis);

        let safe_first = safe_supported_delta(
            &collision,
            *hitbox,
            transform.translation,
            first_delta,
            down,
        );
        let after_first = transform.translation + safe_first;
        let safe_second =
            safe_supported_delta(&collision, *hitbox, after_first, second_delta, down);
        velocity.0 = vertical_velocity + (safe_first + safe_second) / delta_seconds;
    }
}

fn safe_supported_delta(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    start: Vec3,
    requested: Vec3,
    down: Vec3,
) -> Vec3 {
    if requested.length_squared() <= f32::EPSILON {
        return Vec3::ZERO;
    }

    let mut safe_fraction = 0.0;
    for sample in 1..=PATH_SAMPLES {
        let fraction = sample as f32 / PATH_SAMPLES as f32;
        if has_support(collision, hitbox, start + requested * fraction, down) {
            safe_fraction = fraction;
            continue;
        }

        let mut low = safe_fraction;
        let mut high = fraction;
        for _ in 0..BINARY_SEARCH_STEPS {
            let middle = (low + high) * 0.5;
            if has_support(collision, hitbox, start + requested * middle, down) {
                low = middle;
            } else {
                high = middle;
            }
        }
        return requested * low;
    }
    requested
}

fn has_support(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    position: Vec3,
    down: Vec3,
) -> bool {
    collision.has_support(
        position,
        down,
        SUPPORT_PROBE_DISTANCE,
        hitbox.radius,
        hitbox.height,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use collision_api::CollisionResult;

    #[test]
    fn clamps_motion_at_the_first_unsupported_sample() {
        let service = CollisionService::new(
            |_, _, _| false,
            |position, movement, _, _| {
                let supported = position.x <= 0.5 && movement.y < 0.0;
                CollisionResult {
                    position: if supported {
                        position
                    } else {
                        position + movement
                    },
                    grounded: supported,
                    hit_x: false,
                    hit_y: supported,
                    hit_z: false,
                }
            },
        );

        let safe = safe_supported_delta(
            &service,
            PlayerHitbox::default(),
            Vec3::ZERO,
            Vec3::X,
            Vec3::NEG_Y,
        );
        assert!(safe.x <= 0.501);
        assert!(safe.x >= 0.49);
    }
}
