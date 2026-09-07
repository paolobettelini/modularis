use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_player_controller_api::{
    Grounded, Player, PlayerControllerApi, PlayerControllerSet, PlayerPlanarMovementIntent,
    PlayerVelocity,
};
use collision_api::{CharacterQuery, CollisionApi, CollisionService};
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_hitbox_api::{PlayerHitbox, PlayerHitboxApi};
use player_sneak_api::{LocalPlayerSneak, PlayerSneakApi};
use tokio::task::JoinHandle;

const SUPPORT_PROBE_DISTANCE: f32 = 0.025;

/// Fraction of the capsule radius by which the player center may hang past the
/// supporting face while sneaking.
const SNEAK_OVERHANG_FRACTION: f32 = 0.90;

/// Small numerical margin from the exact sneak-support boundary.
const EDGE_BACKOFF: f32 = 0.002;

const PATH_SAMPLES: usize = 8;
const BINARY_SEARCH_STEPS: usize = 9;
const EDGE_NORMAL_PROBE: f32 = 0.03;

const DIAGONAL: f32 = 0.70710677;
const SAMPLE_DIRECTIONS: [(f32, f32); 8] = [
    (1.0, 0.0),
    (-1.0, 0.0),
    (0.0, 1.0),
    (0.0, -1.0),
    (DIAGONAL, DIAGONAL),
    (DIAGONAL, -DIAGONAL),
    (-DIAGONAL, DIAGONAL),
    (-DIAGONAL, -DIAGONAL),
];

#[derive(Resource, Default)]
struct SneakEdgeLatch {
    active: bool,
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
            // The controller's normal grounded probe runs in Input. It must stay
            // strict: capsule edge/corner contacts are not ground. Immediately
            // afterwards this sneak-only policy may restore Grounded while a
            // previously grounded crouching player still has partial footprint
            // support.
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
    if !sneak.active || gravity.0.length_squared() == 0.0 {
        latch.active = false;
        return;
    }

    let up = gravity.up();
    let (axis_a, axis_b) = tangent_basis(up);

    for (transform, velocity, mut grounded) in &mut players {
        // Never turn a real jump / upward launch into edge grounding.
        if velocity.0.dot(up) > 0.05 {
            latch.active = false;
            continue;
        }

        // A true support contact arms the latch. The normal controller probe has
        // already run in Input, so Grounded here represents genuine support.
        if grounded.0 {
            latch.active = true;
            continue;
        }

        // Once armed, sneak may keep the player grounded for a controlled
        // partial-footprint overhang. This happens before GravityForces and
        // MovementConstraints, so gravity cannot steal one falling tick.
        if latch.active
            && has_sneak_support(
                &collision,
                *hitbox,
                transform.translation,
                up,
                axis_a,
                axis_b,
            )
        {
            grounded.0 = true;
        } else {
            latch.active = false;
        }
    }
}

fn constrain_sneaking_movement(
    time: Res<Time<Fixed>>,
    sneak: Res<LocalPlayerSneak>,
    gravity: Res<Gravity>,
    hitbox: Res<PlayerHitbox>,
    collision: Res<CollisionService>,
    movement_intent: Res<PlayerPlanarMovementIntent>,
    mut players: Query<(&Transform, &Grounded, &mut PlayerVelocity), With<Player>>,
) {
    if !sneak.active || gravity.0.length_squared() == 0.0 {
        return;
    }

    let delta_seconds = time.delta_secs();
    if delta_seconds <= f32::EPSILON {
        return;
    }

    let up = gravity.up();
    let (axis_a, axis_b) = tangent_basis(up);

    for (transform, grounded, mut velocity) in &mut players {
        // JumpForces runs before this set and clears Grounded when jumping.
        if !grounded.0 {
            continue;
        }

        if !has_sneak_support(
            &collision,
            *hitbox,
            transform.translation,
            up,
            axis_a,
            axis_b,
        ) {
            continue;
        }

        let vertical_speed = velocity.0.dot(up);

        // IMPORTANT: edge protection is a movement constraint, not another
        // movement/physics source. Use the already-modified planar intent as
        // the authoritative crouch walking velocity. In particular, the sneak
        // speed mod has already applied its multiplier before this set.
        //
        // Do not derive edge motion from PlayerVelocity: that value may contain
        // residual tangential components produced by collision resolution or
        // other forces, which previously made edge movement feel like ice.
        let desired_speed =
            (movement_intent.target_speed * movement_intent.speed_multiplier).max(0.0);
        let desired_planar_velocity =
            movement_intent.direction.normalize_or_zero() * desired_speed;
        let requested_planar_delta = desired_planar_velocity * delta_seconds;

        let safe_planar_delta = safe_supported_delta(
            &collision,
            *hitbox,
            transform.translation,
            requested_planar_delta,
            up,
            axis_a,
            axis_b,
        );

        // Ground support owns motion into the floor. Preserve upward motion,
        // but planar movement while crouching is exactly the constrained
        // movement intent above. Releasing movement input therefore stops the
        // player immediately just like ordinary crouch walking.
        let vertical_velocity = up * vertical_speed.max(0.0);

        velocity.0 = vertical_velocity + safe_planar_delta / delta_seconds;
    }
}

/// Keep the requested planar motion when possible.
///
/// When the requested displacement would leave sneak support, estimate the
/// inward normal of the support boundary and decompose the ORIGINAL requested
/// displacement into:
///
/// - motion toward the supported area;
/// - motion tangent to the edge;
/// - motion outward from the edge.
///
/// Only the outward component is constrained. The tangential component is never
/// added on top of movement already performed, so edge movement cannot become
/// faster than the original requested speed. For a diagonal input, the speed
/// along the edge naturally becomes the tangential projection of that input.
fn safe_supported_delta(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    start: Vec3,
    requested: Vec3,
    up: Vec3,
    axis_a: Vec3,
    axis_b: Vec3,
) -> Vec3 {
    if requested.length_squared() <= f32::EPSILON {
        return Vec3::ZERO;
    }

    if resolved_sneak_support(
        collision,
        hitbox,
        start,
        requested,
        up,
        axis_a,
        axis_b,
    ) {
        return requested;
    }

    // First locate the boundary in the direction the player actually asked to
    // move. This is used only to estimate the local edge orientation.
    let boundary_delta = max_supported_delta(
        collision,
        hitbox,
        start,
        requested,
        up,
        axis_a,
        axis_b,
    );
    let boundary_position =
        resolved_position(collision, hitbox, start, boundary_delta, up);

    let inward = estimate_support_inward(
        collision,
        hitbox,
        boundary_position,
        up,
        axis_a,
        axis_b,
    );

    if inward.length_squared() <= 1e-8 {
        let fallback = best_projected_supported_delta(
            collision,
            hitbox,
            start,
            requested,
            up,
            axis_a,
            axis_b,
        );
        return if fallback.length_squared() > boundary_delta.length_squared() {
            fallback
        } else {
            boundary_delta
        };
    }

    let normal_amount = requested.dot(inward);
    let tangent_requested = requested - inward * normal_amount;

    // Positive is toward more support and can remain untouched. Negative is
    // outward and is the only component sneak protection needs to limit.
    let inward_requested = inward * normal_amount.max(0.0);
    let outward_requested = inward * normal_amount.min(0.0);

    // Preserve the complete non-outward part first. At a corner, even tangent
    // motion can meet another edge, so still validate it through the same
    // support predicate rather than blindly accepting it.
    let non_outward_requested = inward_requested + tangent_requested;
    let safe_non_outward = max_supported_delta(
        collision,
        hitbox,
        start,
        non_outward_requested,
        up,
        axis_a,
        axis_b,
    );

    let after_non_outward =
        resolved_position(collision, hitbox, start, safe_non_outward, up);

    // Then allow only as much outward overhang as the sneak footprint permits.
    let safe_outward = max_supported_delta(
        collision,
        hitbox,
        after_non_outward,
        outward_requested,
        up,
        axis_a,
        axis_b,
    );

    let candidate = safe_non_outward + safe_outward;

    // Never return a displacement whose magnitude exceeds the requested
    // movement. In exact arithmetic the orthogonal decomposition already
    // guarantees this; the clamp protects against numerical noise / sequential
    // collision resolution.
    let candidate = clamp_to_requested_length(candidate, requested.length());

    let primary = if resolved_sneak_support(
        collision,
        hitbox,
        start,
        candidate,
        up,
        axis_a,
        axis_b,
    ) {
        candidate
    } else {
        max_supported_delta(
            collision,
            hitbox,
            start,
            candidate,
            up,
            axis_a,
            axis_b,
        )
    };

    // Near a corner the coverage gradient can point between two real edges and
    // make the primary solution unnecessarily stall. Compare it with a small
    // projection-only search and keep whichever preserves more of the original
    // requested motion. Neither path can exceed the requested speed.
    let fallback = best_projected_supported_delta(
        collision,
        hitbox,
        start,
        requested,
        up,
        axis_a,
        axis_b,
    );

    if fallback.dot(requested) > primary.dot(requested) {
        fallback
    } else {
        primary
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

/// Estimate the direction toward increasing sneak support using the gradient of
/// footprint coverage instead of a single supported/unsupported sample.
///
/// This is substantially less jittery near corners and rotated frame edges.
fn estimate_support_inward(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    position: Vec3,
    up: Vec3,
    axis_a: Vec3,
    axis_b: Vec3,
) -> Vec3 {
    let plus_a = support_coverage(
        collision,
        hitbox,
        position + axis_a * EDGE_NORMAL_PROBE,
        up,
        axis_a,
        axis_b,
    );
    let minus_a = support_coverage(
        collision,
        hitbox,
        position - axis_a * EDGE_NORMAL_PROBE,
        up,
        axis_a,
        axis_b,
    );
    let plus_b = support_coverage(
        collision,
        hitbox,
        position + axis_b * EDGE_NORMAL_PROBE,
        up,
        axis_a,
        axis_b,
    );
    let minus_b = support_coverage(
        collision,
        hitbox,
        position - axis_b * EDGE_NORMAL_PROBE,
        up,
        axis_a,
        axis_b,
    );

    let gradient =
        axis_a * (plus_a - minus_a) + axis_b * (plus_b - minus_b);

    if gradient.length_squared() > 1e-8 {
        return gradient.normalize();
    }

    // Fallback for a perfectly flat quantized coverage field: use a wider
    // directional vote. This only decides edge orientation; it never changes
    // the support rules themselves.
    let mut inward = Vec3::ZERO;

    for (a, b) in SAMPLE_DIRECTIONS {
        let direction = (axis_a * a + axis_b * b).normalize_or_zero();

        let supported_forward = has_sneak_support(
            collision,
            hitbox,
            position + direction * EDGE_NORMAL_PROBE * 2.0,
            up,
            axis_a,
            axis_b,
        );
        let supported_backward = has_sneak_support(
            collision,
            hitbox,
            position - direction * EDGE_NORMAL_PROBE * 2.0,
            up,
            axis_a,
            axis_b,
        );

        match (supported_forward, supported_backward) {
            (true, false) => inward += direction,
            (false, true) => inward -= direction,
            _ => {}
        }
    }

    (inward - up * inward.dot(up)).normalize_or_zero()
}

fn support_coverage(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    position: Vec3,
    up: Vec3,
    axis_a: Vec3,
    axis_b: Vec3,
) -> f32 {
    let mut supported = 0.0;
    let mut samples = 1.0;

    if has_support(collision, hitbox, position, up) {
        supported += 1.0;
    }

    let overhang = hitbox.radius * SNEAK_OVERHANG_FRACTION;

    for (a, b) in SAMPLE_DIRECTIONS {
        samples += 1.0;
        let offset = (axis_a * a + axis_b * b) * overhang;

        if has_support(collision, hitbox, position + offset, up) {
            supported += 1.0;
        }
    }

    supported / samples
}


/// Corner fallback for cases where the local support gradient is ambiguous.
///
/// Candidate directions are sampled in the gravity plane. Each candidate gets
/// only the scalar projection of the ORIGINAL requested displacement onto that
/// direction, so it can never create speed or inertia. The candidate with the
/// greatest progress along the player's requested motion wins.
fn best_projected_supported_delta(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    start: Vec3,
    requested: Vec3,
    up: Vec3,
    axis_a: Vec3,
    axis_b: Vec3,
) -> Vec3 {
    let requested_len = requested.length();
    if requested_len <= f32::EPSILON {
        return Vec3::ZERO;
    }

    let requested_dir = requested / requested_len;
    let mut best = Vec3::ZERO;
    let mut best_progress = 0.0;

    // 16 directions are enough to make corners stable without turning this
    // constraint into a steering system.
    const DIRECTIONS: usize = 16;

    for i in 0..DIRECTIONS {
        let angle = i as f32 * std::f32::consts::TAU / DIRECTIONS as f32;
        let dir = axis_a * angle.cos() + axis_b * angle.sin();

        // Projection, not full requested speed. A 45 degree approach to an edge
        // therefore moves along it at about 70.7% speed, never faster.
        let projected_len = requested.dot(dir);
        if projected_len <= 0.0 {
            continue;
        }

        let candidate = dir * projected_len;

        if resolved_sneak_support(
            collision,
            hitbox,
            start,
            candidate,
            up,
            axis_a,
            axis_b,
        ) {
            let progress = candidate.dot(requested_dir);
            if progress > best_progress {
                best_progress = progress;
                best = candidate;
            }
        }
    }

    best
}

fn max_supported_delta(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    start: Vec3,
    requested: Vec3,
    up: Vec3,
    axis_a: Vec3,
    axis_b: Vec3,
) -> Vec3 {
    if requested.length_squared() <= f32::EPSILON {
        return Vec3::ZERO;
    }

    let mut safe_fraction = 0.0;

    for sample in 1..=PATH_SAMPLES {
        let fraction = sample as f32 / PATH_SAMPLES as f32;

        if resolved_sneak_support(
            collision,
            hitbox,
            start,
            requested * fraction,
            up,
            axis_a,
            axis_b,
        ) {
            safe_fraction = fraction;
            continue;
        }

        let mut low = safe_fraction;
        let mut high = fraction;

        for _ in 0..BINARY_SEARCH_STEPS {
            let middle = (low + high) * 0.5;

            if resolved_sneak_support(
                collision,
                hitbox,
                start,
                requested * middle,
                up,
                axis_a,
                axis_b,
            ) {
                low = middle;
            } else {
                high = middle;
            }
        }

        return backed_off_delta(requested, low);
    }

    requested
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
    axis_a: Vec3,
    axis_b: Vec3,
) -> bool {
    let result_position = resolved_position(collision, hitbox, start, movement, up);

    has_sneak_support(
        collision,
        hitbox,
        result_position,
        up,
        axis_a,
        axis_b,
    )
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

/// Sneak-specific partial-footprint support.
///
/// The normal character solver intentionally does NOT use capsule edge/corner
/// contacts as ground. Here we probe virtual capsule-axis positions within the
/// real player's footprint, allowing the center to extend beyond a ledge while
/// some of the footprint still lies over a genuine walkable face.
fn has_sneak_support(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    position: Vec3,
    up: Vec3,
    axis_a: Vec3,
    axis_b: Vec3,
) -> bool {
    if has_support(collision, hitbox, position, up) {
        return true;
    }

    let overhang = hitbox.radius * SNEAK_OVERHANG_FRACTION;

    for (a, b) in SAMPLE_DIRECTIONS {
        let offset = (axis_a * a + axis_b * b) * overhang;

        if has_support(collision, hitbox, position + offset, up) {
            return true;
        }
    }

    false
}

fn has_support(
    collision: &CollisionService,
    hitbox: PlayerHitbox,
    position: Vec3,
    up: Vec3,
) -> bool {
    collision.has_support(
        position,
        -up,
        SUPPORT_PROBE_DISTANCE * (hitbox.height / 1.8),
        hitbox.radius,
        hitbox.height,
    )
}

/// Deterministic orthonormal basis of the plane perpendicular to gravity-up.
fn tangent_basis(up: Vec3) -> (Vec3, Vec3) {
    let up = up.normalize();

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
