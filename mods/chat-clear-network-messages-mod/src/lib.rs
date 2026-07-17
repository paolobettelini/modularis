use tokio::task::JoinHandle;

pub struct ChatClearNetworkMessagesMod;

impl ChatClearNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
