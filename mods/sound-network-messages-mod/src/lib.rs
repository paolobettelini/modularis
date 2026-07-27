use tokio::task::JoinHandle;

pub struct SoundNetworkMessagesMod;

impl SoundNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
