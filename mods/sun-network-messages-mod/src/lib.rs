use tokio::task::JoinHandle;

pub struct SunNetworkMessagesMod;

impl SunNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
