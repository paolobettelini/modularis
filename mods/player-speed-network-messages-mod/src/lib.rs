use tokio::task::JoinHandle;

pub struct PlayerSpeedNetworkMessagesMod;

impl PlayerSpeedNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
