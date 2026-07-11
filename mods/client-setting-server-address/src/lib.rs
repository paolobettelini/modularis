use client_config_api::ClientConfigApi;
use settings_schema_api::{SettingDefault, SettingDefinition, SettingType};
use tokio::task::JoinHandle;

pub const DEFINITION: SettingDefinition = SettingDefinition {
    id: "network.server_address",
    label: "Server address",
    kind: SettingType::String,
    input: "string",
    default: SettingDefault::String("127.0.0.1:9999"),
};

pub struct ClientSettingServerAddressMod;

impl ClientSettingServerAddressMod {
    pub fn init<C: ClientConfigApi>(_config: &mut C) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
