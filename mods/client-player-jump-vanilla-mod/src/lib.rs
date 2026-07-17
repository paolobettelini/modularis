use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_keybinding_api::parse_key_code;
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use client_player_controller_api::{
    Grounded, Player, PlayerControllerApi, PlayerControllerSet, PlayerVelocity,
};
use client_session_api::{ClientSession, ClientSessionApi};
use client_settings_api::{SettingsApi, SettingsStore};
use client_settings_registry_codegen::SettingsRegistryCodegenMod;
use generated_client_settings_registry::SettingKey;
use generated_network_messages::ServerBoundMessage;
use network_protocol_mod::NetworkProtocolMod;
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_jump_api::{JumpConfig, LocalPlayerJumped, PlayerJumpApi};
use player_jump_network_message_types::PlayerJumpRequest;
use tokio::task::JoinHandle;

#[derive(Resource, Debug)]
struct PredictedJumpGate {
    rearm_remaining_seconds: f32,
    buffered_input_seconds: f32,
}

impl Default for PredictedJumpGate {
    fn default() -> Self {
        Self {
            rearm_remaining_seconds: 0.0,
            buffered_input_seconds: 0.0,
        }
    }
}

pub struct ClientPlayerJumpVanillaMod;

impl ClientPlayerJumpVanillaMod {
    pub fn init<
        N: ClientNetworkApi,
        C: ClientSessionApi,
        P: PlayerControllerApi,
        G: PlayerGravityApi,
        S: GameStateApi,
        T: SettingsApi,
    >(
        bevy: &mut BevyMod,
        _network: &mut N,
        _session: &mut C,
        _player: &mut P,
        _gravity: &mut G,
        _protocol: &mut NetworkProtocolMod,
        _game_state: &mut S,
        _settings: &mut T,
        _settings_codegen: &mut SettingsRegistryCodegenMod,
    ) -> Self {
        bevy.app
            .init_resource::<JumpConfig>()
            .init_resource::<PredictedJumpGate>()
            .add_message::<LocalPlayerJumped>()
            .add_systems(
                Update,
                buffer_jump_input.run_if(in_state(InGameOverlayState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                jump.in_set(PlayerControllerSet::JumpForces)
                    .run_if(in_state(InGameOverlayState::Playing)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl PlayerJumpApi for ClientPlayerJumpVanillaMod {}

fn buffer_jump_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<SettingsStore>,
    config: Res<JumpConfig>,
    mut gate: ResMut<PredictedJumpGate>,
) {
    let jump_key = settings
        .get_string(SettingKey::ControlsJumpKey)
        .and_then(parse_key_code)
        .unwrap_or(KeyCode::Space);
    if keyboard.just_pressed(jump_key) {
        gate.buffered_input_seconds = config.input_buffer_seconds.max(0.0);
    }
}

fn jump(
    time: Res<Time<Fixed>>,
    gravity: Res<Gravity>,
    config: Res<JumpConfig>,
    sender: Option<Res<ClientNetworkSender>>,
    session: Res<ClientSession>,
    mut gate: ResMut<PredictedJumpGate>,
    mut players: Query<(&mut PlayerVelocity, &mut Grounded), With<Player>>,
    mut jumped_events: MessageWriter<LocalPlayerJumped>,
) {
    gate.rearm_remaining_seconds = (gate.rearm_remaining_seconds - time.delta_secs()).max(0.0);
    gate.buffered_input_seconds = (gate.buffered_input_seconds - time.delta_secs()).max(0.0);
    if gate.buffered_input_seconds <= 0.0 {
        return;
    }
    if gate.rearm_remaining_seconds > 0.0 {
        return;
    }

    let up = gravity.up();
    let mut jumped = false;
    for (mut velocity, mut grounded) in &mut players {
        if !grounded.0 {
            continue;
        }
        velocity.0 = velocity.0 - up * velocity.0.dot(up) + up * config.speed;
        grounded.0 = false;
        jumped = true;
    }

    if !jumped {
        return;
    }

    gate.buffered_input_seconds = 0.0;
    gate.rearm_remaining_seconds = config.rearm_seconds;
    jumped_events.write(LocalPlayerJumped);
    if session.player_id.is_some()
        && let Some(sender) = sender.as_ref()
    {
        let _ = sender.send(&ServerBoundMessage::PlayerJumpRequest(PlayerJumpRequest));
    }
}
