use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_external_link_api::{
    ClientExternalLinkApi, ClientExternalLinkSet, ShowClientExternalLink,
};
use tokio::task::JoinHandle;

pub struct ClientExternalLinkEventsMod;

impl ClientExternalLinkEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .add_message::<ShowClientExternalLink>()
            .configure_sets(
                Update,
                (ClientExternalLinkSet::Receive, ClientExternalLinkSet::Present).chain(),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

impl ClientExternalLinkApi for ClientExternalLinkEventsMod {}
