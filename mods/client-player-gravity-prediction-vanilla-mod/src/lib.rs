use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameState, GameStateApi};
use client_player_controller_api::{
    Grounded, Player, PlayerControllerApi, PlayerControllerSet, PlayerVelocity,
};
use player_gravity_api::{Gravity, PlayerGravityApi};
use tokio::task::JoinHandle;

pub struct ClientPlayerGravityPredictionVanillaMod;

impl ClientPlayerGravityPredictionVanillaMod {
    pub fn init<P: PlayerControllerApi, G: PlayerGravityApi, S: GameStateApi>(
        bevy: &mut BevyMod,
        _player: &mut P,
        _gravity: &mut G,
        _game_state: &mut S,
    ) -> Self {
        bevy.app.add_systems(
            FixedUpdate,
            apply_predicted_gravity
                .in_set(PlayerControllerSet::GravityForces)
                .run_if(in_state(GameState::InGame)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_predicted_gravity(
    time: Res<Time<Fixed>>,
    gravity: Res<Gravity>,
    mut players: Query<(&mut PlayerVelocity, &Grounded), With<Player>>,
) {
    let direction = gravity.direction();
    if direction.length_squared() == 0.0 {
        return;
    }
    for (mut velocity, grounded) in &mut players {
        if grounded.0 {
            let falling_speed = velocity.0.dot(direction);
            if falling_speed > 0.0 {
                velocity.0 -= direction * falling_speed;
            }
        } else {
            velocity.0 += gravity.0 * time.delta_secs();
        }
    }
}
