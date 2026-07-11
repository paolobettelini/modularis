use tokio::task::JoinHandle;

pub struct HotbarNetworkMessagesMod;

impl HotbarNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
