use tokio::task::JoinHandle;

pub struct BiomeWarpedForestMod;

impl BiomeWarpedForestMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
