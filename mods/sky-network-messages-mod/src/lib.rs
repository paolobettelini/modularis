use tokio::task::JoinHandle;

pub struct SkyNetworkMessagesMod;

impl SkyNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
