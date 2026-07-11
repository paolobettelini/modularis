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
use player_jump_api::{JumpConfig, PlayerJumpApi};
use player_jump_network_message_types::PlayerJumpRequest;
use tokio::task::JoinHandle;

#[derive(Resource, Debug)]
struct PredictedJumpGate {
    remaining_seconds: f32,
}

impl Default for PredictedJumpGate {
    fn default() -> Self {
        Self {
            remaining_seconds: 0.0,
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
            .add_systems(
                Update,
                jump.in_set(PlayerControllerSet::Forces)
                    .run_if(in_state(InGameOverlayState::Playing)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl PlayerJumpApi for ClientPlayerJumpVanillaMod {}

fn jump(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<SettingsStore>,
    gravity: Res<Gravity>,
    config: Res<JumpConfig>,
    sender: Option<Res<ClientNetworkSender>>,
    session: Res<ClientSession>,
    mut gate: ResMut<PredictedJumpGate>,
    mut players: Query<(&mut PlayerVelocity, &mut Grounded), With<Player>>,
) {
    gate.remaining_seconds = (gate.remaining_seconds - time.delta_secs()).max(0.0);
    let jump_key = settings
        .get_string(SettingKey::ControlsJumpKey)
        .and_then(parse_key_code)
        .unwrap_or(KeyCode::Space);
    if !keyboard.just_pressed(jump_key) {
        return;
    }
    if gate.remaining_seconds > 0.0 {
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

    gate.remaining_seconds = config.rearm_seconds;
    if session.player_id.is_some()
        && let Some(sender) = sender.as_ref()
    {
        let _ = sender.send(&ServerBoundMessage::PlayerJumpRequest(PlayerJumpRequest));
    }
}
