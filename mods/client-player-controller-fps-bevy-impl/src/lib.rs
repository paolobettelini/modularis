use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_camera_api::{CameraApi, PlayerCamera};
use client_game_state_api::{GameState, GameStateApi, InGameOverlayState};
use client_input_api::{InputApi, PlayerInput};
use client_player_controller_api::{
    Grounded, PLAYER_EYE_HEIGHT, PLAYER_HEIGHT, PLAYER_RADIUS, Player, PlayerControllerApi,
    PlayerControllerSet, PlayerMovementConfig, PlayerPlanarMovementIntent, PlayerVelocity,
};
use collision_api::{CollisionApi, CollisionService};
use player_gravity_api::{Gravity, PlayerGravityApi, project_on_gravity_plane};
use tokio::task::JoinHandle;

const GROUND_PROBE_DISTANCE: f32 = 0.01;
const GROUND_LEAVE_SPEED_EPSILON: f32 = 0.05;
const GROUND_STICK_SPEED_EPSILON: f32 = 0.05;

pub struct FpsPlayerControllerBevyImpl;

impl FpsPlayerControllerBevyImpl {
    pub fn init<
        I: InputApi,
        C: CameraApi,
        K: CollisionApi,
        V: PlayerGravityApi,
        G: GameStateApi,
    >(
        bevy: &mut BevyMod,
        _input: &mut I,
        _camera: &mut C,
        _collision: &mut K,
        _gravity: &mut V,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .init_resource::<PlayerMovementConfig>()
            .init_resource::<PlayerPlanarMovementIntent>()
            .configure_sets(
                Update,
                (
                    PlayerControllerSet::Input,
                    PlayerControllerSet::MovementModifiers,
                    PlayerControllerSet::ApplyMovementIntent,
                    PlayerControllerSet::Forces,
                    PlayerControllerSet::ForceOverrides,
                    PlayerControllerSet::Movement,
                    PlayerControllerSet::CameraSync,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                collect_planar_movement_intent
                    .in_set(PlayerControllerSet::Input)
                    .run_if(in_state(InGameOverlayState::Playing)),
            )
            .add_systems(
                Update,
                apply_planar_movement_intent
                    .in_set(PlayerControllerSet::ApplyMovementIntent)
                    .run_if(in_state(InGameOverlayState::Playing)),
            )
            .add_systems(
                Update,
                (
                    update_grounded_probe.in_set(PlayerControllerSet::Input),
                    move_player.in_set(PlayerControllerSet::Movement),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(InGameOverlayState::Playing), clear_planar_velocity)
            .add_systems(
                Update,
                sync_camera_to_player
                    .in_set(PlayerControllerSet::CameraSync)
                    .run_if(in_state(GameState::InGame)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl PlayerControllerApi for FpsPlayerControllerBevyImpl {}

fn clear_planar_velocity(
    gravity: Res<Gravity>,
    mut players: Query<&mut PlayerVelocity, With<Player>>,
) {
    let up = gravity.up();
    for mut velocity in &mut players {
        velocity.0 = up * velocity.0.dot(up);
    }
}

fn collect_planar_movement_intent(
    input: Res<PlayerInput>,
    gravity: Res<Gravity>,
    camera: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    mut intent: ResMut<PlayerPlanarMovementIntent>,
) {
    let Ok(camera) = camera.single() else {
        *intent = PlayerPlanarMovementIntent::default();
        return;
    };

    let up = gravity.up();
    let mut forward =
        project_on_gravity_plane(camera.rotation * Vec3::NEG_Z, gravity.0).normalize_or_zero();
    if forward.length_squared() == 0.0 {
        forward = project_on_gravity_plane(Vec3::NEG_Z, gravity.0).normalize_or_zero();
    }
    let right = forward.cross(up).normalize_or_zero();
    intent.direction = (forward * input.movement.y + right * input.movement.x).normalize_or_zero();
    intent.speed_multiplier = 1.0;
}

fn apply_planar_movement_intent(
    intent: Res<PlayerPlanarMovementIntent>,
    config: Res<PlayerMovementConfig>,
    gravity: Res<Gravity>,
    mut players: Query<&mut PlayerVelocity, With<Player>>,
) {
    let up = gravity.up();
    for mut velocity in &mut players {
        let vertical = up * velocity.0.dot(up);
        velocity.0 =
            vertical + intent.direction * config.walk_speed * intent.speed_multiplier.max(0.0);
    }
}

fn update_grounded_probe(
    gravity: Res<Gravity>,
    collision: Res<CollisionService>,
    mut players: Query<(&Transform, &PlayerVelocity, &mut Grounded), With<Player>>,
) {
    let direction = gravity.direction();
    if direction.length_squared() == 0.0 {
        return;
    }
    for (transform, velocity, mut grounded) in &mut players {
        let moving_away_from_ground = velocity.0.dot(direction) < -GROUND_LEAVE_SPEED_EPSILON;
        // A probe preserves an existing contact; only a resolved collision may
        // transition an airborne player back to grounded.
        grounded.0 = grounded.0
            && !moving_away_from_ground
            && is_grounded_at(&collision, transform.translation, direction);
    }
}

fn move_player(
    time: Res<Time>,
    gravity: Res<Gravity>,
    collision: Res<CollisionService>,
    mut player: Query<
        (&mut Transform, &mut PlayerVelocity, &mut Grounded),
        (With<Player>, Without<PlayerCamera>),
    >,
) {
    let Ok((mut transform, mut velocity, mut grounded)) = player.single_mut() else {
        return;
    };
    let was_grounded = grounded.0;
    let start = transform.translation;
    let movement = velocity.0 * time.delta_secs();
    let result = collision.resolve(start, movement, PLAYER_RADIUS, PLAYER_HEIGHT);
    transform.translation = result.position;
    if result.hit_x {
        velocity.0.x = 0.0;
    }
    if result.hit_y {
        velocity.0.y = 0.0;
    }
    if result.hit_z {
        velocity.0.z = 0.0;
    }

    let gravity_direction = gravity.direction();
    let blocked_movement = movement - (result.position - start);
    let moving_into_ground = gravity_direction.length_squared() > 0.0
        && movement.dot(gravity_direction) > 0.0
        && blocked_movement.dot(gravity_direction) > 0.0001;
    let near_ground = is_grounded_at(&collision, result.position, gravity_direction);
    let gravity_speed = velocity.0.dot(gravity_direction);
    let moving_away_from_ground = velocity.0.dot(gravity_direction) < -GROUND_LEAVE_SPEED_EPSILON;
    let stable_near_ground = was_grounded
        && near_ground
        && !moving_away_from_ground
        && gravity_speed.abs() <= GROUND_STICK_SPEED_EPSILON;
    grounded.0 = moving_into_ground || stable_near_ground;
    if grounded.0 {
        let falling_speed = velocity.0.dot(gravity_direction);
        if falling_speed > 0.0 {
            velocity.0 -= gravity_direction * falling_speed;
        }
    }
}

fn is_grounded_at(collision: &CollisionService, position: Vec3, gravity_direction: Vec3) -> bool {
    gravity_direction.length_squared() > 0.0
        && collision.collides(
            position + gravity_direction * GROUND_PROBE_DISTANCE,
            PLAYER_RADIUS,
            PLAYER_HEIGHT,
        )
}

fn sync_camera_to_player(
    gravity: Res<Gravity>,
    player: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let (Ok(player), Ok(mut camera)) = (player.single(), camera.single_mut()) else {
        return;
    };
    camera.translation = player.translation + gravity.up() * PLAYER_EYE_HEIGHT;
}
