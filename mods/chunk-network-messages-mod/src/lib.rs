use tokio::task::JoinHandle;

pub struct ChunkNetworkMessagesMod;

impl ChunkNetworkMessagesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
