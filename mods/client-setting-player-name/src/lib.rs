use client_config_api::ClientConfigApi;
use settings_schema_api::{SettingDefault, SettingDefinition, SettingType};
use tokio::task::JoinHandle;

pub const DEFINITION: SettingDefinition = SettingDefinition {
    id: "network.player_name",
    label: "Player name",
    kind: SettingType::String,
    input: "string",
    default: SettingDefault::String("Player"),
};

pub struct ClientSettingPlayerNameMod;

impl ClientSettingPlayerNameMod {
    pub fn init<C: ClientConfigApi>(_config: &mut C) -> Self {
        debug_assert_eq!(C::default_player_name(), "Player");
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
