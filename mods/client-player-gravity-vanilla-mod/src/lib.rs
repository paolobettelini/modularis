use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_player_controller_api::{
    Player, PlayerControllerApi, PlayerControllerSet, PlayerVelocity,
};
use player_gravity_api::{Gravity, PlayerGravityApi};
use tokio::task::JoinHandle;

pub struct ClientPlayerGravityVanillaMod;

impl ClientPlayerGravityVanillaMod {
    pub fn init<P: PlayerControllerApi, G: PlayerGravityApi, S: GameStateApi>(
        bevy: &mut BevyMod,
        _player: &mut P,
        _gravity: &mut G,
        _game_state: &mut S,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            apply_gravity
                .in_set(PlayerControllerSet::Forces)
                .run_if(in_state(InGameOverlayState::Playing)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_gravity(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut players: Query<&mut PlayerVelocity, With<Player>>,
) {
    let acceleration = gravity.0;
    if acceleration.length_squared() == 0.0 {
        return;
    }
    for mut velocity in &mut players {
        velocity.0 += acceleration * time.delta_secs();
    }
}
