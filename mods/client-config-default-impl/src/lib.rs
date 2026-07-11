use client_config_api::ClientConfigApi;
use tokio::task::JoinHandle;

pub struct DefaultClientConfig;

impl DefaultClientConfig {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientConfigApi for DefaultClientConfig {
    fn window_title() -> &'static str {
        "Patchwork"
    }

    fn default_player_name() -> &'static str {
        "Player"
    }

    fn default_server_address() -> &'static str {
        "127.0.0.1:9999"
    }
}
