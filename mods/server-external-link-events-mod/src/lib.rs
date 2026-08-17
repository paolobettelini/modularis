use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_external_link_api::{
    PublishServerExternalLink, ServerExternalLinkApi, ServerExternalLinkSet,
};
use tokio::task::JoinHandle;

pub struct ServerExternalLinkEventsMod;

impl ServerExternalLinkEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .add_message::<PublishServerExternalLink>()
            .configure_sets(
                Update,
                (ServerExternalLinkSet::Publish, ServerExternalLinkSet::Sync).chain(),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl ServerExternalLinkApi for ServerExternalLinkEventsMod {}
