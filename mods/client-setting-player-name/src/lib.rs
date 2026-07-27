use client_config_api::ClientConfigApi;
use tokio::task::JoinHandle;

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
