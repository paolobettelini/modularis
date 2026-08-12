use bevy_mod::BevyMod;
use generated_game_mode_registry::GameMode;
use server_game_mode_creative_vanilla_lib::creative_policy;
use server_player_game_mode_api::{ServerPlayerGameModeApi, ServerPlayerGameModePolicies};
use tokio::task::JoinHandle;

pub struct ServerGameModeCreativeVanillaMod;

impl ServerGameModeCreativeVanillaMod {
    pub fn init<G: ServerPlayerGameModeApi>(bevy: &mut BevyMod, _game_modes: &mut G) -> Self {
        bevy.app.world_mut().resource_mut::<ServerPlayerGameModePolicies>()
            .register(GameMode::Creative, creative_policy());
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

