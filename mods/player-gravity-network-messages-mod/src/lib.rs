use tokio::task::JoinHandle;

pub struct PlayerGravityNetworkMessagesMod;

impl PlayerGravityNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
