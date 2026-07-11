use tokio::task::JoinHandle;

pub struct PlayerNetworkMessagesMod;

impl PlayerNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
