use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_permission_registry::PermissionId;
use server_player_game_mode_api::{GameMode, ServerPlayerGameModeApi, SetPlayerGameMode};
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_permission_api::{ServerPlayerPermissionApi, SetPlayerPermission};
use server_player_registry_api::ServerPlayerSessionSet;
use tokio::task::JoinHandle;

const DEFAULT_CREATIVE_PERMISSION_OWNER: &str = "vanilla:default-creative";

/// Vanilla join policy: every new player starts in Creative and receives the
/// `Privileged` permission with its implied capabilities.
pub struct ServerPlayerDefaultCreativeVanillaMod;

impl ServerPlayerDefaultCreativeVanillaMod {
    pub fn init<G: ServerPlayerGameModeApi, P: ServerPlayerPermissionApi>(
        bevy: &mut BevyMod,
        _game_modes: &mut G,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _permissions: &mut P,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            set_creative_on_join.in_set(ServerPlayerSessionSet::Initialize),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn set_creative_on_join(
    mut joined: MessageReader<ServerPlayerJoined>,
    mut game_modes: MessageWriter<SetPlayerGameMode>,
    mut permissions: MessageWriter<SetPlayerPermission>,
) {
    for player in joined.read() {
        game_modes.write(SetPlayerGameMode {
            player_id: player.player_id,
            mode: GameMode::Creative,
        });
        permissions.write(SetPlayerPermission {
            player_id: player.player_id,
            owner: DEFAULT_CREATIVE_PERMISSION_OWNER.to_string(),
            permission: PermissionId::Privileged,
            enabled: true,
        });
    }
}
