use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_lifecycle_events_api::ServerPlayerLeft;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::ServerPlayerSessionSet;
use server_scope_api::{
    ScopeNodeId, ServerPlayerScopeChanged, ServerScopeApi, ServerScopeNode, ServerScopeSet,
    ServerScopes, SetServerPlayerScope,
};
use tokio::task::JoinHandle;

pub struct ServerScopeTreeMod;

impl ServerScopeTreeMod {
    pub fn init(bevy: &mut BevyMod, _lifecycle: &mut ServerPlayerLifecycleEventsMod) -> Self {
        let root = ScopeNodeId::root();
        let root_entity = bevy
            .app
            .world_mut()
            .spawn(ServerScopeNode {
                id: root,
                parent: None,
            })
            .id();
        bevy.app
            .insert_resource(ServerScopes::with_root(root_entity))
            .add_message::<SetServerPlayerScope>()
            .add_message::<ServerPlayerScopeChanged>()
            .configure_sets(
                Update,
                (
                    ServerScopeSet::ApplyMembership,
                    ServerScopeSet::React,
                    ServerScopeSet::Cleanup,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                apply_membership_requests.in_set(ServerScopeSet::ApplyMembership),
            )
            .add_systems(
                Update,
                cleanup_left_players
                    .after(ServerPlayerSessionSet::Cleanup)
                    .in_set(ServerScopeSet::Cleanup),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerScopeApi for ServerScopeTreeMod {}

fn apply_membership_requests(
    scopes: Res<ServerScopes>,
    mut requests: MessageReader<SetServerPlayerScope>,
    mut changed: MessageWriter<ServerPlayerScopeChanged>,
) {
    for request in requests.read() {
        match scopes.assign_player(request.player_id, request.target.clone()) {
            Ok(previous) if previous.as_ref() != Some(&request.target) => {
                changed.write(ServerPlayerScopeChanged {
                    player_id: request.player_id,
                    previous,
                    current: Some(request.target.clone()),
                });
            }
            Ok(_) => {}
            Err(error) => warn!(
                "could not assign player {} to scope '{}': {error}",
                request.player_id, request.target
            ),
        }
    }
}

fn cleanup_left_players(
    scopes: Res<ServerScopes>,
    mut left: MessageReader<ServerPlayerLeft>,
    mut changed: MessageWriter<ServerPlayerScopeChanged>,
) {
    for player in left.read() {
        let previous = scopes.remove_player(player.player_id);
        if previous.is_some() {
            changed.write(ServerPlayerScopeChanged {
                player_id: player.player_id,
                previous,
                current: None,
            });
        }
    }
}
