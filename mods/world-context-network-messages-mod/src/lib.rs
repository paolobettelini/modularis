use tokio::task::JoinHandle;

pub struct WorldContextNetworkMessagesMod;

impl WorldContextNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
