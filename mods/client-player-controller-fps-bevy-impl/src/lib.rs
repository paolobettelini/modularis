use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_camera_api::{CameraApi, PlayerCamera};
use client_game_state_api::{GameState, GameStateApi, InGameOverlayState};
use client_input_api::{InputApi, PlayerInput};
use client_player_controller_api::{
    Grounded, PLAYER_EYE_HEIGHT, Player, PlayerControllerApi, PlayerControllerSet,
    PlayerMovementConfig, PlayerPlanarMovementIntent, PlayerVelocity, PreviousPlayerPosition,
};
use client_player_physics_tick_api::ClientPlayerPhysicsTickApi;
use collision_api::{CollisionApi, CollisionService};
use player_gravity_api::{Gravity, PlayerGravityApi, project_on_gravity_plane};
use player_hitbox_api::{PlayerHitbox, PlayerHitboxApi};
use player_speed_api::{PlayerSpeedApi, PlayerSpeedMultiplier};
use tokio::task::JoinHandle;

const GROUND_LEAVE_SPEED_EPSILON: f32 = 0.05;

pub struct FpsPlayerControllerBevyImpl;

impl FpsPlayerControllerBevyImpl {
    pub fn init<
        T: ClientPlayerPhysicsTickApi,
        I: InputApi,
        C: CameraApi,
        K: CollisionApi,
        H: PlayerHitboxApi,
        V: PlayerGravityApi,
        M: PlayerSpeedApi,
        G: GameStateApi,
    >(
        bevy: &mut BevyMod,
        _physics_tick: &mut T,
        _input: &mut I,
        _camera: &mut C,
        _collision: &mut K,
        _hitbox: &mut H,
        _gravity: &mut V,
        _speed: &mut M,
        _game_state: &mut G,
    ) -> Self {
        bevy.app
            .insert_resource(Time::<Fixed>::from_hz(T::ticks_per_second()))
            .init_resource::<PlayerMovementConfig>().init_resource::<client_player_surface_api::PlayerSurfaceContact>()
            .init_resource::<PlayerPlanarMovementIntent>()
            .configure_sets(
                FixedUpdate,
                (
                    PlayerControllerSet::Input,
                    PlayerControllerSet::MovementModifiers,
                    PlayerControllerSet::ApplyMovementIntent,
                    PlayerControllerSet::GravityForces,
                    PlayerControllerSet::JumpForces,
                    PlayerControllerSet::Forces,
                    PlayerControllerSet::ForceOverrides,
                    PlayerControllerSet::MovementConstraints,
                    PlayerControllerSet::Movement,
                    PlayerControllerSet::PostMovement,
                )
                    .chain(),
            )
            .configure_sets(
                Update,
                (
                    PlayerControllerSet::CameraSync,
                    PlayerControllerSet::CameraModifiers,
                )
                    .chain(),
            )
            .add_systems(
                FixedUpdate,
                collect_planar_movement_intent
                    .in_set(PlayerControllerSet::Input)
                    .run_if(in_state(InGameOverlayState::Playing)),
            )
            .add_systems(
                FixedUpdate,
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
    config: Res<PlayerMovementConfig>,
    base_speed: Res<PlayerSpeedMultiplier>,
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
    intent.target_speed = config.walk_speed * base_speed.0.max(0.0);
    intent.speed_multiplier = 1.0;
}

fn update_grounded_probe(
    gravity: Res<Gravity>,
    hitbox: Res<PlayerHitbox>,
    collision: Res<CollisionService>,
    mut players: Query<(&Transform, &PlayerVelocity, &mut Grounded), With<Player>>,
) {
    let direction = gravity.direction();
    if direction.length_squared() == 0.0 {
        for (_,_,mut grounded) in &mut players {grounded.0=false;}
        return;
    }
    for (transform, velocity, mut grounded) in &mut players {
        let query=collision_api::CharacterQuery::new(transform.translation,Vec3::ZERO,gravity.up(),hitbox.radius,hitbox.height);
        let support=collision.character_support(query);
        grounded.0=support.is_some_and(|hit|velocity.0.dot(hit.normal)<=GROUND_LEAVE_SPEED_EPSILON);
    }
}

fn move_player(
    mut surface:ResMut<client_player_surface_api::PlayerSurfaceContact>,
    time: Res<Time>,
    gravity: Res<Gravity>,
    hitbox: Res<PlayerHitbox>,
    collision: Res<CollisionService>,
    mut player: Query<
        (
            &mut Transform,
            &mut PreviousPlayerPosition,
            &mut PlayerVelocity,
            &mut Grounded,
        ),
        (With<Player>, Without<PlayerCamera>),
    >,
) {
    let Ok((mut transform, mut previous, mut velocity, mut grounded)) = player.single_mut() else {
        return;
    };
    let was_grounded = grounded.0;
    let start = transform.translation;
    previous.0 = start;
    let movement = velocity.0 * time.delta_secs();
    let mut query=collision_api::CharacterQuery::new(start,movement,gravity.up(),hitbox.radius,hitbox.height);
    query.was_grounded=was_grounded;
    let result=collision.resolve_character(query);
    transform.translation=result.position;
    velocity.0=result.project_velocity(velocity.0);
    grounded.0=result.support.is_some() && (was_grounded || movement.dot(gravity.up())<=query.skin());
    surface.0=if grounded.0{result.support}else{None};

}

fn sync_camera_to_player(
    gravity: Res<Gravity>,
    fixed_time: Res<Time<Fixed>>,
    player: Query<(&Transform, &PreviousPlayerPosition), (With<Player>, Without<PlayerCamera>)>,
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let (Ok((player, previous)), Ok(mut camera)) = (player.single(), camera.single_mut()) else {
        return;
    };
    let rendered_position = previous
        .0
        .lerp(player.translation, fixed_time.overstep_fraction());
    camera.translation = rendered_position + gravity.up() * PLAYER_EYE_HEIGHT;
}
