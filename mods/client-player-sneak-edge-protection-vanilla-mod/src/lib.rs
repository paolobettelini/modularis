use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_player_surface_api::PlayerSurfaceContact;
use client_player_controller_api::{
    Grounded, Player, PlayerControllerApi, PlayerControllerSet, PlayerPlanarMovementIntent,
    PlayerVelocity,
};
use collision_api::{CharacterContact, CharacterQuery, CollisionApi, CollisionService};
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_hitbox_api::{PlayerHitbox, PlayerHitboxApi};
use player_sneak_api::{LocalPlayerSneak, PlayerSneakApi};
use tokio::task::JoinHandle;

const SUPPORT_PROBE_DISTANCE: f32 = 0.025;
const SNEAK_OVERHANG_FRACTION: f32 = 0.90;
const EDGE_BACKOFF: f32 = 0.0005;
const BINARY_SEARCH_STEPS: usize = 9;
const SUPPORT_GRACE_TICKS: u8 = 2;

#[derive(Resource)]
struct SneakEdgeLatch {
    active: bool,
    surface: u128,
    local_face_normal: Vec3,
    local_edge_a: Vec3,
    local_edge_b: Vec3,
    missed_support_ticks: u8,
}

impl Default for SneakEdgeLatch {
    fn default() -> Self {
        Self {
            active: false,
            surface: 0,
            local_face_normal: Vec3::Y,
            local_edge_a: Vec3::X,
            local_edge_b: Vec3::Z,
            missed_support_ticks: 0,
        }
    }
}

impl SneakEdgeLatch {
    fn clear(&mut self) {
        *self = Self::default();
    }
}

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
        bevy.app
            .init_resource::<SneakEdgeLatch>()
            .add_systems(
                FixedUpdate,
                restore_sneak_edge_grounding
                    .in_set(PlayerControllerSet::MovementModifiers)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                constrain_sneaking_movement
                    .in_set(PlayerControllerSet::MovementConstraints)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                maintain_sneak_support_after_movement
                    .in_set(PlayerControllerSet::PostMovement)
                    .run_if(in_state(GameState::InGame)),
            );

        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn restore_sneak_edge_grounding(
    sneak: Res<LocalPlayerSneak>,
    gravity: Res<Gravity>,
    hitbox: Res<PlayerHitbox>,
    collision: Res<CollisionService>,
    mut latch: ResMut<SneakEdgeLatch>,
    mut players: Query<(&Transform, &PlayerVelocity, &mut Grounded), With<Player>>,
) {
    if !sneak.active || gravity.0.length_squared() <= f32::EPSILON {
        latch.clear();
        return;
    }

    let up = gravity.up();

    for (transform, velocity, mut grounded) in &mut players {
        // A real jump must detach immediately from sneak edge protection.
        let current_support = strict_support(&collision, *hitbox, transform.translation, up);
        let support_normal = current_support.map(|hit| hit.normal).unwrap_or_else(||
            if latch.active { latch_world_normal(&collision, &latch) } else { up });
        if velocity.0.dot(support_normal) > 0.05 {
            latch.clear();
            continue;
        }

        if let Some(contact) = current_support {
            arm_latch_from_contact(&collision, &mut latch, contact, up);
            grounded.0 = true;
            continue;
        }

        if !latch.active {
            continue;
        }

        let edges = latch_edge_directions(&collision, &latch, up);

        if has_sneak_support(
            &collision,
            *hitbox,
            transform.translation,
            up,
            edges,
        ) {
            latch.missed_support_ticks = 0;
            grounded.0 = true;
            continue;
        }

        // A transformed frame/capsule corner can lose the strict support probe
        // for one fixed tick because of skin/rounding. Keep the previously
        // armed latch briefly, while the movement constraint below prevents
        // further travel into unsupported space.
        if latch.missed_support_ticks < SUPPORT_GRACE_TICKS {
            latch.missed_support_ticks += 1;
            grounded.0 = true;
        } else {
            latch.clear();
        }
    }
}

fn maintain_sneak_support_after_movement(
    sneak: Res<LocalPlayerSneak>,
    gravity: Res<Gravity>,
    hitbox: Res<PlayerHitbox>,
    collision: Res<CollisionService>,
    latch: Res<SneakEdgeLatch>,
    mut surface: ResMut<PlayerSurfaceContact>,
    mut players: Query<(&Transform, &PlayerVelocity, &mut Grounded), With<Player>>,
) {
    if !sneak.active || !latch.active || gravity.0.length_squared() <= f32::EPSILON {
        return;
    }

    let up = gravity.up();
    let edges = latch_edge_directions(&collision, &latch, up);

    for (transform, velocity, mut grounded) in &mut players {
        // Never resurrect a support latch after JumpForces detached the player.
        if !grounded.0 && velocity.0.dot(latch_world_normal(&collision, &latch)) > 0.05 { continue; }
        // A real strict support returned by move_player is already ideal.
        if grounded.0 && surface.0.is_some() {
            continue;
        }

        if !has_sneak_support(
            &collision,
            *hitbox,
            transform.translation,
            up,
            edges,
        ) && latch.missed_support_ticks >= SUPPORT_GRACE_TICKS
        {
            continue;
        }

        grounded.0 = true;
        surface.0 = Some(CharacterContact {
            normal: latch_world_normal(&collision, &latch),
            point: transform.translation,
            surface: latch.surface,
        });
    }
}

fn constrain_sneaking_movement(
    time: Res<Time<Fixed>>,
    sneak: Res<LocalPlayerSneak>,
    gravity: Res<Gravity>,
    hitbox: Res<PlayerHitbox>,
    collision: Res<CollisionService>,
    movement_intent: Res<PlayerPlanarMovementIntent>,
    latch: Res<SneakEdgeLatch>,
    mut players: Query<(&Transform, &Grounded, &mut PlayerVelocity), With<Player>>,
) {
    if !sneak.active || gravity.0.length_squared() <= f32::EPSILON {
        return;
    }

    let delta_seconds = time.delta_secs();
    if delta_seconds <= f32::EPSILON {
        return;
    }

    let up = gravity.up();
    let edges = if latch.active {
        latch_edge_directions(&collision, &latch, up)
    } else {
        tangent_basis(up)
    };

    for (transform, grounded, mut velocity) in &mut players {
        // JumpForces runs before this set and clears Grounded for a real jump.
        if !grounded.0 || !latch.active {
            continue;
        }

        let normal = latch_world_normal(&collision, &latch);

        // Sneak edge protection is only a constraint. It never carries planar
        // velocity of its own: use the already crouch-scaled movement intent.
        let desired_speed =
            (movement_intent.target_speed * movement_intent.speed_multiplier).max(0.0);
        let raw_direction = movement_intent.direction - up * movement_intent.direction.dot(up);
        let desired_planar_velocity = project_to_plane(raw_direction, normal).normalize_or_zero() * desired_speed;
        let requested_delta = desired_planar_velocity * delta_seconds;

        let safe_delta = safe_supported_delta(
            &collision,
            *hitbox,
            transform.translation,
            requested_delta,
            up,
            edges,
        );

        // Walk tangent to the actual support. A gravity-up component here is
        // slope traversal, not an extra vertical impulse to preserve each tick.
        velocity.0 = safe_delta / delta_seconds;
    }
}

fn arm_latch_from_contact(
    collision: &CollisionService,
    latch: &mut SneakEdgeLatch,
    contact: CharacterContact,
    up: Vec3,
) {
    let rotation = collision
        .surface_pose(contact.surface)
        .map(|(_, rotation)| rotation)
        .unwrap_or(Quat::IDENTITY);
    let local_normal = rotation.conjugate() * contact.normal;

    let face_axis = dominant_axis(local_normal);
    // A model element may itself be rotated inside the frame. Keep its real
    // normal; the dominant axis only chooses candidate footprint directions.
    let local_face_normal = local_normal.normalize_or_zero();
    let (local_edge_a, local_edge_b) = match face_axis {
        0 => (Vec3::Y, Vec3::Z),
        1 => (Vec3::X, Vec3::Z),
        _ => (Vec3::X, Vec3::Y),
    };

    latch.active = true;
    latch.surface = contact.surface;
    latch.local_face_normal = local_face_normal;
    latch.local_edge_a = local_edge_a;
    latch.local_edge_b = local_edge_b;
    latch.missed_support_ticks = 0;

    // Validate immediately. Extremely degenerate projected edges fall back to
    // a gravity-plane basis in `latch_edge_directions`.
    let _ = latch_edge_directions(collision, latch, up);
}

fn dominant_axis(v: Vec3) -> usize {
    let a = v.abs();
    if a.x >= a.y && a.x >= a.z {
        0
    } else if a.y >= a.z {
        1
    } else {
        2
    }
}

/// Returns the two real block/frame edge directions projected into the player's
/// gravity plane. The local axes remain stable while overhanging; a moving or
/// rotating frame is refreshed from its current surface pose every tick.
fn latch_world_normal(collision: &CollisionService, latch: &SneakEdgeLatch) -> Vec3 {
    let rotation = collision
        .surface_pose(latch.surface)
        .map(|(_, rotation)| rotation)
        .unwrap_or(Quat::IDENTITY);
    (rotation * latch.local_face_normal).normalize_or_zero()
}

fn latch_edge_directions(
    collision: &CollisionService,
    latch: &SneakEdgeLatch,
    up: Vec3,
) -> (Vec3, Vec3) {
    let rotation = collision
        .surface_pose(latch.surface)
        .map(|(_, rotation)| rotation)
        .unwrap_or(Quat::IDENTITY);

    let projected_a = project_to_plane(rotation * latch.local_edge_a, up);
    let projected_b = project_to_plane(rotation * latch.local_edge_b, up);

    match (
        projected_a.length_squared() > 1e-8,
        projected_b.length_squared() > 1e-8,
    ) {
        (true, true) => (projected_a.normalize(), projected_b.normalize()),
        (true, false) => {
            let a = projected_a.normalize();
            (a, up.cross(a).normalize_or_zero())
        }
        (false, true) => {
            let b = projected_b.normalize();
            (up.cross(b).normalize_or_zero(), b)
        }
        (false, false) => tangent_basis(up),
    }
}

fn project_to_plane(v: Vec3, normal: Vec3) -> Vec3 {
    v - normal * v.dot(normal)
}

/// Clamp only the part of the requested crouch movement that would leave the
/// sneak-support footprint. Candidate slide directions come from the actual
/// support frame edges, never from a sampled gradient, so corner transitions do
/// not acquire an artificial steering direction or extra velocity.
fn safe_supported_delta(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    start: Vec3,
    requested: Vec3,
    up: Vec3,
    edges: (Vec3, Vec3),
) -> Vec3 {
    if requested.length_squared() <= f32::EPSILON {
        return Vec3::ZERO;
    }

    if resolved_sneak_support(collision, hitbox, start, requested, up, edges) {
        return requested;
    }

    let mut best = max_supported_delta(collision, hitbox, start, requested, up, edges);

    for edge in [edges.0, edges.1] {
        if edge.length_squared() <= 1e-8 {
            continue;
        }

        let edge = edge.normalize();
        let tangent = edge * requested.dot(edge);
        let remainder = requested - tangent;

        // Moving along the physical edge first preserves the expected
        // Minecraft-like diagonal glide. The second ordering is useful at a
        // corner when moving inward onto the adjacent face before turning.
        for candidate in [
            sequence_candidate(
                collision, hitbox, start, tangent, remainder, up, edges,
            ),
            sequence_candidate(
                collision, hitbox, start, remainder, tangent, up, edges,
            ),
        ] {
            best = better_candidate(best, candidate, requested);
        }
    }

    clamp_to_requested_length(best, requested.length())
}

fn sequence_candidate(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    start: Vec3,
    first: Vec3,
    second: Vec3,
    up: Vec3,
    edges: (Vec3, Vec3),
) -> Vec3 {
    let safe_first = max_supported_delta(collision, hitbox, start, first, up, edges);
    let after_first = resolved_position(collision, hitbox, start, safe_first, up);
    let safe_second = max_supported_delta(collision, hitbox, after_first, second, up, edges);
    let candidate = safe_first + safe_second;

    if resolved_sneak_support(collision, hitbox, start, candidate, up, edges) {
        candidate
    } else {
        max_supported_delta(collision, hitbox, start, candidate, up, edges)
    }
}

fn better_candidate(current: Vec3, candidate: Vec3, requested: Vec3) -> Vec3 {
    const SCORE_EPSILON: f32 = 1e-7;

    let current_progress = current.dot(requested);
    let candidate_progress = candidate.dot(requested);

    if candidate_progress > current_progress + SCORE_EPSILON
        || ((candidate_progress - current_progress).abs() <= SCORE_EPSILON
            && candidate.length_squared() > current.length_squared())
    {
        candidate
    } else {
        current
    }
}

fn clamp_to_requested_length(delta: Vec3, requested_length: f32) -> Vec3 {
    let length = delta.length();
    if length <= requested_length || length <= f32::EPSILON {
        delta
    } else {
        delta * (requested_length / length)
    }
}

fn max_supported_delta(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    start: Vec3,
    requested: Vec3,
    up: Vec3,
    edges: (Vec3, Vec3),
) -> Vec3 {
    if requested.length_squared() <= f32::EPSILON {
        return Vec3::ZERO;
    }

    if resolved_sneak_support(collision, hitbox, start, requested, up, edges) {
        return requested;
    }

    // If a one-tick latch grace started just outside the sampled footprint,
    // only a movement that returns to support is allowed. The full-delta check
    // above already accepted that recovery case.
    if !has_sneak_support(collision, hitbox, start, up, edges) {
        return Vec3::ZERO;
    }

    let mut low = 0.0;
    let mut high = 1.0;

    for _ in 0..BINARY_SEARCH_STEPS {
        let middle = (low + high) * 0.5;
        if resolved_sneak_support(
            collision,
            hitbox,
            start,
            requested * middle,
            up,
            edges,
        ) {
            low = middle;
        } else {
            high = middle;
        }
    }

    backed_off_delta(requested, low)
}

fn backed_off_delta(requested: Vec3, safe_fraction: f32) -> Vec3 {
    if safe_fraction <= 0.0 {
        return Vec3::ZERO;
    }

    let requested_length = requested.length();
    if requested_length <= f32::EPSILON {
        return Vec3::ZERO;
    }

    let backoff_fraction = (EDGE_BACKOFF / requested_length).min(safe_fraction);
    requested * (safe_fraction - backoff_fraction).max(0.0)
}

fn resolved_sneak_support(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    start: Vec3,
    movement: Vec3,
    up: Vec3,
    edges: (Vec3, Vec3),
) -> bool {
    let position = resolved_position(collision, hitbox, start, movement, up);
    has_sneak_support(collision, hitbox, position, up, edges)
}

fn resolved_position(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    start: Vec3,
    movement: Vec3,
    up: Vec3,
) -> Vec3 {
    let mut query = CharacterQuery::new(start, movement, up, hitbox.radius, hitbox.height);
    query.was_grounded = true;
    collision.resolve_character(query).position
}

/// Sneak-only partial footprint support. The normal collision solver remains
/// strict and never classifies a capsule edge/corner contact as ground.
fn has_sneak_support(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    position: Vec3,
    up: Vec3,
    edges: (Vec3, Vec3),
) -> bool {
    if strict_support(collision, hitbox, position, up).is_some() {
        return true;
    }

    let overhang = hitbox.radius * SNEAK_OVERHANG_FRACTION;
    let a = edges.0.normalize_or_zero();
    let b = edges.1.normalize_or_zero();
    if a == Vec3::ZERO || b == Vec3::ZERO {
        return false;
    }

    let diagonal_ab = (a + b).normalize_or_zero();
    let diagonal_a_minus_b = (a - b).normalize_or_zero();
    let directions = [
        a,
        -a,
        b,
        -b,
        diagonal_ab,
        -diagonal_ab,
        diagonal_a_minus_b,
        -diagonal_a_minus_b,
    ];

    directions.into_iter().any(|direction| {
        direction != Vec3::ZERO
            && strict_support(
                collision,
                hitbox,
                position + direction * overhang,
                up,
            )
            .is_some()
    })
}

fn strict_support(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    position: Vec3,
    up: Vec3,
) -> Option<CharacterContact> {
    let mut query = CharacterQuery::new(position, Vec3::ZERO, up, hitbox.radius, hitbox.height);
    query.ground_probe = SUPPORT_PROBE_DISTANCE * (hitbox.height / 1.8);
    collision.character_support(query)
}

fn tangent_basis(up: Vec3) -> (Vec3, Vec3) {
    let up = up.normalize_or_zero();
    if up == Vec3::ZERO {
        return (Vec3::X, Vec3::Z);
    }

    let helper = if up.x.abs() <= up.y.abs() && up.x.abs() <= up.z.abs() {
        Vec3::X
    } else if up.y.abs() <= up.z.abs() {
        Vec3::Y
    } else {
        Vec3::Z
    };

    let axis_a = up.cross(helper).normalize_or_zero();
    let axis_b = up.cross(axis_a).normalize_or_zero();
    (axis_a, axis_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use collision_api::{Aabb, CharacterCollisionBackend, CharacterGeometry, CollisionBox};

    #[derive(Clone)]
    struct Scene(Vec<CollisionBox>);

    impl CharacterGeometry for Scene {
        fn query(&self, _: Aabb) -> Vec<CollisionBox> {
            self.0.clone()
        }
    }

    impl CharacterCollisionBackend for Scene {
        fn resolve(&self, query: CharacterQuery) -> collision_api::CharacterResult {
            character_collision_lib::resolve(query, self)
        }

        fn support(&self, query: CharacterQuery) -> Option<CharacterContact> {
            character_collision_lib::support(query, self)
        }

        fn surface_pose(&self, surface: u128) -> Option<(Vec3, Quat)> {
            self.0
                .iter()
                .find(|box_| box_.surface == surface)
                .map(|box_| (box_.center, box_.rotation))
        }
    }

    fn floor(rotation: Quat) -> CollisionService {
        CollisionService::new(|_, _, _| false, |_, _, _, _| unreachable!())
            .with_character_backend(Scene(vec![CollisionBox {
                center: rotation * Vec3::new(0.0, -0.5, 0.0),
                rotation,
                half_extents: Vec3::new(0.5, 0.5, 0.5),
                surface: 7,
            }]))
    }

    #[test]
    fn constrained_delta_never_exceeds_requested_speed() {
        let collision = floor(Quat::IDENTITY);
        let hitbox = PlayerHitbox::default();
        let start = Vec3::new(0.65, 0.001, 0.0);
        let requested = Vec3::new(0.1, 0.0, 0.1);
        let safe = safe_supported_delta(
            &collision,
            hitbox,
            start,
            requested,
            Vec3::Y,
            (Vec3::X, Vec3::Z),
        );

        assert!(safe.length() <= requested.length() + 1e-6);
    }

    #[test]
    fn diagonal_edge_motion_keeps_tangent_component() {
        let collision = floor(Quat::IDENTITY);
        let hitbox = PlayerHitbox::default();
        let start = Vec3::new(0.75, 0.001, 0.0);
        let requested = Vec3::new(0.08, 0.0, 0.08);
        let safe = safe_supported_delta(
            &collision,
            hitbox,
            start,
            requested,
            Vec3::Y,
            (Vec3::X, Vec3::Z),
        );

        assert!(safe.z > 0.02);
        assert!(safe.length() <= requested.length() + 1e-6);
    }

    #[test]
    fn frame_edges_follow_rotated_surface_pose() {
        let rotation = Quat::from_euler(EulerRot::XYZ, 0.3, 0.4, -0.2);
        let collision = floor(rotation);
        let mut latch = SneakEdgeLatch::default();
        let up = rotation * Vec3::Y;
        let contact = CharacterContact {
            normal: up,
            point: Vec3::ZERO,
            surface: 7,
        };
        arm_latch_from_contact(&collision, &mut latch, contact, up);
        let (a, b) = latch_edge_directions(&collision, &latch, up);

        assert!(a.dot(up).abs() < 1e-5);
        assert!(b.dot(up).abs() < 1e-5);
        assert!(a.length() > 0.99);
        assert!(b.length() > 0.99);
    }
}
