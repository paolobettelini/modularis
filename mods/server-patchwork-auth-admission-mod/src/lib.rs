use bevy::prelude::*;
use bevy_mod::BevyMod;
use patchwork_game_auth_api::{ServerAuthenticatedAccounts, ServerPatchworkPlayerJoined};
use patchwork_game_auth_events_mod::PatchworkGameAuthEventsMod;
use server_patchwork_auth_handshake_mod::ServerPatchworkAuthHandshakeMod;
use server_player_admission_api::{
    ServerJoinCandidate, ServerPlayerAdmissionRule, ServerPlayerAdmissionRules,
};
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use tokio::task::JoinHandle;

#[derive(Clone)]
struct PatchworkAccountAdmissionRule {
    accounts: ServerAuthenticatedAccounts,
}

impl ServerPlayerAdmissionRule for PatchworkAccountAdmissionRule {
    fn prepare(&self, candidate: &mut ServerJoinCandidate) -> Result<(), String> {
        let account = self
            .accounts
            .account_for_address(candidate.address)
            .ok_or_else(|| "Patchwork account authentication is required".to_owned())?;
        candidate.name = account.nickname;
        Ok(())
    }

    fn validate(
        &self,
        candidate: &ServerJoinCandidate,
        _online: &[player_network_message_types::NetworkPlayer],
    ) -> Result<(), String> {
        let account = self
            .accounts
            .account_for_address(candidate.address)
            .ok_or_else(|| "Patchwork account authentication is required".to_owned())?;
        if candidate.name != account.nickname {
            return Err(
                "player identity does not match the authenticated Patchwork account".to_owned(),
            );
        }
        Ok(())
    }
}

pub struct ServerPatchworkAuthAdmissionMod;

impl ServerPatchworkAuthAdmissionMod {
    #[allow(clippy::too_many_arguments)]
    pub fn init<P: ServerPlayerRegistryApi>(
        bevy: &mut BevyMod,
        _players: &mut P,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
        _events: &mut PatchworkGameAuthEventsMod,
        _handshake: &mut ServerPatchworkAuthHandshakeMod,
    ) -> Self {
        let accounts = bevy
            .app
            .world()
            .resource::<ServerAuthenticatedAccounts>()
            .clone();
        bevy.app
            .world_mut()
            .resource_mut::<ServerPlayerAdmissionRules>()
            .register(PatchworkAccountAdmissionRule { accounts });
        bevy.app.add_systems(Update, bind_authenticated_player);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn bind_authenticated_player(
    mut joined: MessageReader<ServerPlayerJoined>,
    registry: Res<ServerPlayerRegistry>,
    accounts: Res<ServerAuthenticatedAccounts>,
    mut authenticated_join: MessageWriter<ServerPatchworkPlayerJoined>,
) {
    for joined in joined.read() {
        let Some(address) = registry.address_for_player(joined.player_id) else {
            continue;
        };
        let Some(account) = accounts.bind_player(address, joined.player_id) else {
            warn!(
                "player {} joined without a Patchwork account binding",
                joined.player_id
            );
            continue;
        };
        authenticated_join.write(ServerPatchworkPlayerJoined {
            player_id: joined.player_id,
            account: account.clone(),
        });
        info!(
            "player '{}' logged in as player {} with Patchwork account {} from {}",
            account.nickname, joined.player_id, account.account_uuid, address
        );
    }
}
