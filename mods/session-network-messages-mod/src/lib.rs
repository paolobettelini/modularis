use tokio::task::JoinHandle;

pub struct SessionNetworkMessagesMod;

impl SessionNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
