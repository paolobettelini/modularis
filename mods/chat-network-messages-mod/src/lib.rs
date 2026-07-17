use tokio::task::JoinHandle;

pub struct ChatNetworkMessagesMod;

impl ChatNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
