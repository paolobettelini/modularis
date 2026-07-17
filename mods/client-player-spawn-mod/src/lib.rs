use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_camera_api::CameraApi;
use client_game_state_api::{GameState, GameStateApi};
use client_player_controller_api::{
    Grounded, Player, PlayerControllerApi, PlayerVelocity, PreviousPlayerPosition,
};
use tokio::task::JoinHandle;

pub struct PlayerSpawnMod;

impl PlayerSpawnMod {
    pub fn init<G: GameStateApi, P: PlayerControllerApi, C: CameraApi>(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _player_controller: &mut P,
        _camera: &mut C,
    ) -> Self {
        bevy.app
            .add_systems(OnEnter(GameState::InGame), spawn_player);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn spawn_player(mut commands: Commands, players: Query<(), With<Player>>) {
    if !players.is_empty() {
        return;
    }
    commands.spawn((
        Player,
        PlayerVelocity::default(),
        Grounded::default(),
        PreviousPlayerPosition(Vec3::new(0.0, 2.0, 0.0)),
        Transform::from_xyz(0.0, 2.0, 0.0),
        DespawnOnExit(GameState::InGame),
    ));
}
