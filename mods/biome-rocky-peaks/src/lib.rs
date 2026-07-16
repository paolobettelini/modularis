use tokio::task::JoinHandle;

pub struct BiomeRockyPeaksMod;

impl BiomeRockyPeaksMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
