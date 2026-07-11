use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_portal_api::{
    ServerPortalApi, ServerPortalOpened, ServerPortalRules, ServerPortalSet, ServerPortals,
};
use tokio::task::JoinHandle;

pub struct ServerPortalStateMod;

impl ServerPortalStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<ServerPortals>()
            .init_resource::<ServerPortalRules>()
            .add_message::<ServerPortalOpened>()
            .configure_sets(
                Update,
                (
                    ServerPortalSet::Ignite,
                    ServerPortalSet::Sync,
                    ServerPortalSet::Travel,
                )
                    .chain(),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerPortalApi for ServerPortalStateMod {}
