use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi, InGameOverlayState};
use client_keybinding_api::parse_key_code;
use client_player_controller_api::{
    Grounded, Player, PlayerControllerApi, PlayerControllerSet, PlayerMovementConfig,
    PlayerPlanarMovementIntent, PlayerVelocity,
};
use client_settings_api::{SettingsApi, SettingsStore};
use client_settings_registry_codegen::SettingsRegistryCodegenMod;
use generated_client_settings_registry::SettingKey;
use player_flight_api::{FlightConfig, LocalPlayerFlight, PlayerFlightApi};
use player_flight_speed_api::{PlayerFlightSpeedApi, PlayerFlightSpeedMultiplier};
use player_gravity_api::{Gravity, PlayerGravityApi};
use tokio::task::JoinHandle;

#[derive(Resource, Default)]
struct FlightToggleInput {
    previous_press_at: Option<f64>,
}

pub struct ClientPlayerFlightVanillaMod;

impl ClientPlayerFlightVanillaMod {
    pub fn init<
        F: PlayerFlightApi,
        V: PlayerFlightSpeedApi,
        P: PlayerControllerApi,
        G: PlayerGravityApi,
        S: SettingsApi,
        State: GameStateApi,
    >(
        bevy: &mut BevyMod,
        _flight: &mut F,
        _flight_speed: &mut V,
        _player: &mut P,
        _gravity: &mut G,
        _settings: &mut S,
        _settings_codegen: &mut SettingsRegistryCodegenMod,
        _game_state: &mut State,
    ) -> Self {
        bevy.app
            .init_resource::<FlightToggleInput>()
            .add_systems(
                Update,
                toggle_flight.run_if(in_state(InGameOverlayState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                apply_flight_planar_speed
                    .in_set(PlayerControllerSet::MovementModifiers)
                    .run_if(in_state(InGameOverlayState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                apply_flight_velocity
                    .in_set(PlayerControllerSet::ForceOverrides)
                    .run_if(in_state(GameState::InGame)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn toggle_flight(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<SettingsStore>,
    config: Res<FlightConfig>,
    mut input: ResMut<FlightToggleInput>,
    mut flight: ResMut<LocalPlayerFlight>,
    mut players: Query<(&mut PlayerVelocity, &mut Grounded), With<Player>>,
) {
    if !flight.capability_enabled {
        input.previous_press_at = None;
        flight.flying = false;
        return;
    }
    let jump_key = settings
        .get_string(SettingKey::ControlsJumpKey)
        .and_then(parse_key_code)
        .unwrap_or(KeyCode::Space);
    if !keyboard.just_pressed(jump_key) {
        return;
    }
    let now = time.elapsed_secs_f64();
    let double_tapped = input
        .previous_press_at
        .is_some_and(|previous| now - previous <= config.double_tap_seconds);
    input.previous_press_at = Some(now);
    if !double_tapped {
        return;
    }
    input.previous_press_at = None;
    flight.flying = !flight.flying;
    if flight.flying {
        for (mut velocity, mut grounded) in &mut players {
            velocity.0 = Vec3::ZERO;
            grounded.0 = false;
        }
    }
}

fn apply_flight_planar_speed(
    movement_config: Res<PlayerMovementConfig>,
    flight_speed: Res<PlayerFlightSpeedMultiplier>,
    flight: Res<LocalPlayerFlight>,
    mut movement: ResMut<PlayerPlanarMovementIntent>,
) {
    if flight.capability_enabled && flight.flying {
        movement.target_speed = movement_config.walk_speed * flight_speed.0.max(0.0);
    }
}

fn apply_flight_velocity(
    keyboard: Res<ButtonInput<KeyCode>>,
    overlay: Res<State<InGameOverlayState>>,
    settings: Res<SettingsStore>,
    gravity: Res<Gravity>,
    movement_config: Res<PlayerMovementConfig>,
    movement: Res<PlayerPlanarMovementIntent>,
    flight_speed: Res<PlayerFlightSpeedMultiplier>,
    flight: Res<LocalPlayerFlight>,
    mut players: Query<(&mut PlayerVelocity, &mut Grounded), With<Player>>,
) {
    if !flight.capability_enabled || !flight.flying {
        return;
    }
    let jump_key = settings
        .get_string(SettingKey::ControlsJumpKey)
        .and_then(parse_key_code)
        .unwrap_or(KeyCode::Space);
    let vertical_input = if *overlay.get() == InGameOverlayState::Playing {
        let up = if keyboard.pressed(jump_key) { 1.0 } else { 0.0 };
        let sneak_key = settings
            .get_string(SettingKey::ControlsSneakKey)
            .and_then(parse_key_code)
            .unwrap_or(KeyCode::ShiftLeft);
        let down = if keyboard.pressed(sneak_key) {
            1.0
        } else {
            0.0
        };
        up - down
    } else {
        0.0
    };
    let up = gravity.up();
    let vertical_speed = movement_config.walk_speed * flight_speed.0.max(0.0);
    let planar_velocity =
        movement.direction * (movement.target_speed * movement.speed_multiplier).max(0.0);
    for (mut velocity, mut grounded) in &mut players {
        velocity.0 = planar_velocity + up * vertical_input * vertical_speed;
        grounded.0 = false;
    }
}
