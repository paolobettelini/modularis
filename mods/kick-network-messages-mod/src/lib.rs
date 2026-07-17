use tokio::task::JoinHandle;

pub struct KickNetworkMessagesMod;

impl KickNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
