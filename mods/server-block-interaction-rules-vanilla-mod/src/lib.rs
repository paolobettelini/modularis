use bevy_mod::BevyMod;
use server_block_interaction_rules_api::{
    ServerBlockInteractionRules, ServerBlockInteractionRulesApi,
};
use tokio::task::JoinHandle;

pub struct ServerBlockInteractionRulesVanillaMod;

impl ServerBlockInteractionRulesVanillaMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.insert_resource(ServerBlockInteractionRules {
            max_reach: 6.5,
            eye_height: 1.5,
        });
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ServerBlockInteractionRulesApi for ServerBlockInteractionRulesVanillaMod {}
