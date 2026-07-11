use tokio::task::JoinHandle;

pub struct PlayerJumpNetworkMessagesMod;

impl PlayerJumpNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
