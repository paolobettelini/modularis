use bevy_mod::BevyMod;
use client_block_interaction_rules_api::{
    ClientBlockInteractionRules, ClientBlockInteractionRulesApi,
};
use tokio::task::JoinHandle;

pub struct ClientBlockInteractionRulesVanillaMod;

impl ClientBlockInteractionRulesVanillaMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .insert_resource(ClientBlockInteractionRules { max_reach: 6.0 });
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientBlockInteractionRulesApi for ClientBlockInteractionRulesVanillaMod {}
