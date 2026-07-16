use tokio::task::JoinHandle;

pub struct BiomeForestMod;

impl BiomeForestMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
