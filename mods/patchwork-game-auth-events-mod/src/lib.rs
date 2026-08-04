use bevy_mod::BevyMod;
use patchwork_game_auth_api::{
    ClientPatchworkGameAuthenticated, ClientPatchworkProcessAuthenticated,
    ServerPatchworkAccountAuthenticated, ServerPatchworkPlayerJoined,
};
use tokio::task::JoinHandle;

pub struct PatchworkGameAuthEventsMod;

impl PatchworkGameAuthEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .add_message::<ClientPatchworkProcessAuthenticated>()
            .add_message::<ClientPatchworkGameAuthenticated>()
            .add_message::<ServerPatchworkAccountAuthenticated>()
            .add_message::<ServerPatchworkPlayerJoined>();
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
