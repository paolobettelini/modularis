use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_player_controller_api::{
    Grounded, Player, PlayerControllerApi, PlayerControllerSet, PlayerVelocity,
};
use collision_api::{CharacterQuery, CollisionApi, CollisionService};
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_hitbox_api::{PlayerHitbox, PlayerHitboxApi};
use player_sneak_api::{LocalPlayerSneak, PlayerSneakApi};
use tokio::task::JoinHandle;

const SUPPORT_PROBE_DISTANCE: f32 = 0.025;
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

    for (transform, grounded, mut velocity) in &mut players {
        if !grounded.0 || !has_support(&collision, *hitbox, transform.translation, down) {
            continue;
        }

        // Predict the same full motion as the controller. Capsule contact can
        // turn gravity into outward motion at an edge, even without WASD.
        let safe = safe_supported_delta(
            &collision,
            *hitbox,
            transform.translation,
            velocity.0 * delta_seconds,
            down,
        );
        velocity.0 = safe / delta_seconds;
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
        if resolved_support(collision, hitbox, start, requested * fraction, down) {
            safe_fraction = fraction;
            continue;
        }

        let mut low = safe_fraction;
        let mut high = fraction;
        for _ in 0..BINARY_SEARCH_STEPS {
            let middle = (low + high) * 0.5;
            if resolved_support(collision, hitbox, start, requested * middle, down) {
                low = middle;
            } else {
                high = middle;
            }
        }
        return requested * low;
    }
    requested
}

fn resolved_support(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    start: Vec3,
    movement: Vec3,
    down: Vec3,
) -> bool {
    let mut query = CharacterQuery::new(start, movement, -down, hitbox.radius, hitbox.height);
    query.was_grounded = true;
    let result = collision.resolve_character(query);
    // Inspect the final resolved position, not the old support attached to an
    // intermediate contact. This also supports legacy collision providers.
    has_support(collision, hitbox, result.position, down)
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
        SUPPORT_PROBE_DISTANCE * (hitbox.height / 1.8),
        hitbox.radius,
        hitbox.height,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use collision_api::CollisionResult;

    struct Floor(collision_api::CollisionBox);
    impl collision_api::CharacterGeometry for Floor {
        fn query(&self, _: collision_api::Aabb) -> Vec<collision_api::CollisionBox> { vec![self.0] }
    }
    impl collision_api::CharacterCollisionBackend for Floor {
        fn resolve(&self, q: CharacterQuery) -> collision_api::CharacterResult { character_collision_lib::resolve(q, self) }
        fn support(&self, q: CharacterQuery) -> Option<collision_api::CharacterContact> { character_collision_lib::support(q, self) }
    }

    #[test]
    fn stays_supported_at_edges_with_arbitrary_gravity_and_tilted_frame() {
        for rotation in [Quat::IDENTITY, Quat::from_euler(EulerRot::XYZ, 0.7, 0.4, -0.8)] {
            let up = rotation * Vec3::Y;
            let frame_rotation = rotation * Quat::from_rotation_z(0.3);
            let normal = frame_rotation * Vec3::Y;
            let hitbox = PlayerHitbox::default();
            let service = CollisionService::new(|_,_,_| false, |_,_,_,_| unreachable!())
                .with_character_backend(Floor(collision_api::CollisionBox {
                    center: Vec3::ZERO, rotation: frame_rotation,
                    half_extents: Vec3::new(1.0, 0.2, 1.0), surface: 42,
                }));
            let mut position = normal * (0.2 + hitbox.radius + 0.001) - up * hitbox.radius;
            for _ in 0..120 {
                let requested = rotation * Vec3::new(0.04, -0.02, 0.0);
                let safe = safe_supported_delta(&service, hitbox, position, requested, -up);
                let mut q = CharacterQuery::new(position, safe, up, hitbox.radius, hitbox.height);
                q.was_grounded = true;
                position = service.resolve_character(q).position;
                assert!(service.character_support(CharacterQuery { position, ..q }).is_some());
            }
        }
    }

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
