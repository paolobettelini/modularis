use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_game_state_api::{GameStateApi, InGameOverlayState};
use client_player_controller_api::{
    PlayerControllerApi, PlayerControllerSet, PlayerPlanarMovementIntent,
};
use player_sneak_api::{LocalPlayerSneak, PlayerSneakApi};
use tokio::task::JoinHandle;

#[derive(Resource, Debug, Clone, Copy)]
pub struct SneakSpeedConfig {
    pub multiplier: f32,
}

impl Default for SneakSpeedConfig {
    fn default() -> Self {
        Self { multiplier: 0.3 }
    }
}

pub struct ClientPlayerSneakSpeedVanillaMod;

impl ClientPlayerSneakSpeedVanillaMod {
    pub fn init<G: GameStateApi, P: PlayerControllerApi, S: PlayerSneakApi>(
        bevy: &mut BevyMod,
        _game_state: &mut G,
        _controller: &mut P,
        _sneak: &mut S,
    ) -> Self {
        bevy.app.init_resource::<SneakSpeedConfig>().add_systems(
            FixedUpdate,
            apply_sneak_speed
                .in_set(PlayerControllerSet::MovementModifiers)
                .run_if(in_state(InGameOverlayState::Playing)),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn apply_sneak_speed(
    sneak: Res<LocalPlayerSneak>,
    config: Res<SneakSpeedConfig>,
    mut movement: ResMut<PlayerPlanarMovementIntent>,
) {
    if sneak.active {
        movement.speed_multiplier *= config.multiplier.max(0.0);
    }
}
