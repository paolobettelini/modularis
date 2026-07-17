use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_camera_api::{CameraAngles, CameraApi, PlayerCamera};
use client_game_state_api::{GameState, GameStateApi, InGameOverlayState};
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use client_player_controller_api::{
    Player, PlayerControllerApi, PlayerControllerSet, PreviousPlayerPosition,
};
use client_session_api::{ClientSession, ClientSessionApi};
use generated_network_messages::{
    NetworkMessageSet, PlayerMovedReceived, PlayerRotationChangedReceived, ServerBoundMessage,
};
use network_protocol_mod::NetworkProtocolMod;
use player_gravity_api::PlayerGravityApi;
use player_network_message_types::PlayerMove;
use tokio::task::JoinHandle;

#[derive(Resource)]
struct MovementSendTimer(Timer);

#[derive(Resource, Default)]
struct AuthoritativePlayerTarget {
    position: Option<Vec3>,
    rotation: Option<(f32, f32)>,
}

pub struct ClientPlayerNetworkSyncMod;

impl ClientPlayerNetworkSyncMod {
    pub fn init<
        N: ClientNetworkApi,
        S: ClientSessionApi,
        P: PlayerControllerApi,
        C: CameraApi,
        V: PlayerGravityApi,
        G: GameStateApi,
    >(
        bevy: &mut BevyMod,
        _network: &mut N,
        _session: &mut S,
        _player: &mut P,
        _camera: &mut C,
        _gravity: &mut V,
        _game_state: &mut G,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app
            .init_resource::<AuthoritativePlayerTarget>()
            .insert_resource(MovementSendTimer(Timer::from_seconds(
                0.05,
                TimerMode::Repeating,
            )))
            .add_systems(
                Update,
                (
                    apply_authoritative_player_updates.after(NetworkMessageSet::DispatchPackets),
                    apply_authoritative_player_target.before(PlayerControllerSet::CameraSync),
                    send_player_movement.after(PlayerControllerSet::CameraSync),
                )
                    .run_if(in_state(GameState::InGame)),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_authoritative_player_updates(
    session: Res<ClientSession>,
    mut moved: MessageReader<PlayerMovedReceived>,
    mut rotated: MessageReader<PlayerRotationChangedReceived>,
    mut target: ResMut<AuthoritativePlayerTarget>,
) {
    let Some(local_id) = session.player_id else {
        return;
    };

    for moved in moved.read() {
        if moved.0.player_id != local_id {
            continue;
        }
        target.position = Some(Vec3::from_array(moved.0.position));
    }

    for rotated in rotated.read() {
        if rotated.0.player_id != local_id {
            continue;
        }
        target.rotation = Some((rotated.0.yaw, rotated.0.pitch));
    }
}

fn apply_authoritative_player_target(
    time: Res<Time>,
    overlay: Res<State<InGameOverlayState>>,
    mut target: ResMut<AuthoritativePlayerTarget>,
    mut player: Query<(&mut Transform, &mut PreviousPlayerPosition), With<Player>>,
    mut camera: Query<&mut CameraAngles, With<PlayerCamera>>,
) {
    let playing = *overlay.get() == InGameOverlayState::Playing;
    let rotation_smoothing = 1.0 - (-35.0 * time.delta_secs()).exp();
    if let Some(target_position) = target.position
        && let Ok((mut player, mut previous)) = player.single_mut()
    {
        let delta = target_position - player.translation;
        let ignore_threshold = if playing { 0.03 } else { 0.01 };
        let snap_threshold = if playing { 0.75 } else { 0.25 };
        if delta.length() > snap_threshold {
            player.translation = target_position;
            previous.0 = target_position;
        } else if delta.length() > ignore_threshold {
            // Server positions sent for the local player are corrections, not a
            // continuously refreshed interpolation target. Consume each one
            // once so a stale target cannot fight local gravity every frame.
            player.translation += delta * 0.5;
            previous.0 = player.translation;
        }
        target.position = None;
    }

    if let Some((yaw, pitch)) = target.rotation
        && let Ok(mut camera) = camera.single_mut()
    {
        if (camera.yaw - yaw).abs() < 0.001 && (camera.pitch - pitch).abs() < 0.001 {
            camera.yaw = yaw;
            camera.pitch = pitch;
            target.rotation = None;
        } else {
            camera.yaw += (yaw - camera.yaw) * rotation_smoothing;
            camera.pitch += (pitch - camera.pitch) * rotation_smoothing;
        }
    }
}

fn send_player_movement(
    time: Res<Time>,
    mut timer: ResMut<MovementSendTimer>,
    sender: Option<Res<ClientNetworkSender>>,
    session: Res<ClientSession>,
    player: Query<&Transform, With<Player>>,
    camera: Query<&CameraAngles, With<PlayerCamera>>,
) {
    if session.player_id.is_none() || !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let (Some(sender), Ok(player), Ok(camera)) = (sender, player.single(), camera.single()) else {
        return;
    };
    let _ = sender.send(&ServerBoundMessage::PlayerMove(PlayerMove {
        position: player.translation.to_array(),
        yaw: camera.yaw,
        pitch: camera.pitch,
    }));
}
