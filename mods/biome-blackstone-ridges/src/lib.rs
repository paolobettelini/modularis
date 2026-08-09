use tokio::task::JoinHandle;

pub struct BiomeBlackstoneRidgesMod;

impl BiomeBlackstoneRidgesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
