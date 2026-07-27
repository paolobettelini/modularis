use client_config_api::ClientConfigApi;
use tokio::task::JoinHandle;

pub struct ClientSettingServerAddressMod;

impl ClientSettingServerAddressMod {
    pub fn init<C: ClientConfigApi>(_config: &mut C) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
