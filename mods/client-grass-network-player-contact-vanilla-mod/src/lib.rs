use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_grass_interaction_api::{
    ClientGrassInteractionApi, ClientGrassInteractionField, GrassInteractionCollectSet,
    GrassInteractionSource,
};
use client_player_gravity_map_api::{ClientPlayerGravities, ClientPlayerGravityMapApi};
use client_player_render_api::{ClientPlayerRenderApi, RenderedNetworkPlayers};
use client_player_scale_map_api::{ClientPlayerScaleMapApi, ClientPlayerScales};
use player_hitbox_api::PlayerHitbox;
use std::collections::BTreeSet;
use tokio::task::JoinHandle;

const REMOTE_PLAYER_SOURCE_PREFIX: &str = "vanilla:network-player:";

#[derive(Resource, Default)]
struct NetworkPlayerGrassSources(BTreeSet<String>);

pub struct ClientGrassNetworkPlayerContactVanillaMod;

impl ClientGrassNetworkPlayerContactVanillaMod {
    pub fn init<
        I: ClientGrassInteractionApi,
        R: ClientPlayerRenderApi,
        G: ClientPlayerGravityMapApi,
        S: ClientPlayerScaleMapApi,
    >(
        bevy: &mut BevyMod,
        _interactions: &mut I,
        _player_render: &mut R,
        _gravities: &mut G,
        _scales: &mut S,
    ) -> Self {
        bevy.app
            .init_resource::<NetworkPlayerGrassSources>()
            .add_systems(
                Update,
                update_network_player_grass_contacts.in_set(GrassInteractionCollectSet),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn update_network_player_grass_contacts(
    rendered: Res<RenderedNetworkPlayers>,
    transforms: Query<&Transform>,
    gravities: Res<ClientPlayerGravities>,
    scales: Res<ClientPlayerScales>,
    mut previous: ResMut<NetworkPlayerGrassSources>,
    mut field: ResMut<ClientGrassInteractionField>,
) {
    let mut current = BTreeSet::new();
    for (player_id, visual) in &rendered.entities {
        let Ok(transform) = transforms.get(visual.avatar) else {
            continue;
        };
        let owner = format!("{REMOTE_PLAYER_SOURCE_PREFIX}{player_id}");
        let hitbox = PlayerHitbox::default().scaled(scales.scale(*player_id));
        let axis = gravities.gravity(*player_id).up();
        let half_length = hitbox.height * 0.5;
        field.set(
            owner.clone(),
            GrassInteractionSource {
                position: transform.translation + axis * half_length,
                axis,
                half_length,
                radius: hitbox.radius + 0.55,
                strength: 1.0,
            },
        );
        current.insert(owner);
    }

    for stale in previous.0.difference(&current) {
        field.remove(stale);
    }
    previous.0 = current;
}
