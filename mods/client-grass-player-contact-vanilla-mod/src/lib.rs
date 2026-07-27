use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_grass_interaction_api::{
    ClientGrassInteractionApi, ClientGrassInteractionField, GrassInteractionCollectSet,
    GrassInteractionSource,
};
use client_player_controller_api::{Player, PlayerControllerApi, PlayerControllerSet};
use player_gravity_api::{Gravity, PlayerGravityApi};
use player_hitbox_api::{PlayerHitbox, PlayerHitboxApi};
use tokio::task::JoinHandle;

const LOCAL_PLAYER_SOURCE: &str = "vanilla:local-player";

pub struct ClientGrassPlayerContactVanillaMod;

impl ClientGrassPlayerContactVanillaMod {
    pub fn init<
        I: ClientGrassInteractionApi,
        P: PlayerControllerApi,
        G: PlayerGravityApi,
        H: PlayerHitboxApi,
    >(
        bevy: &mut BevyMod,
        _interactions: &mut I,
        _controller: &mut P,
        _gravity: &mut G,
        _hitbox: &mut H,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            update_player_grass_contact
                .in_set(GrassInteractionCollectSet)
                .after(PlayerControllerSet::PostMovement),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn update_player_grass_contact(
    player: Query<&Transform, With<Player>>,
    gravity: Res<Gravity>,
    hitbox: Res<PlayerHitbox>,
    mut field: ResMut<ClientGrassInteractionField>,
) {
    let Ok(transform) = player.single() else {
        field.remove(LOCAL_PLAYER_SOURCE);
        return;
    };
    let axis = gravity.up();
    let half_length = hitbox.height * 0.5;
    field.set(
        LOCAL_PLAYER_SOURCE,
        GrassInteractionSource {
            position: transform.translation + axis * half_length,
            axis,
            half_length,
            radius: hitbox.radius + 0.55,
            strength: 1.0,
        },
    );
}
