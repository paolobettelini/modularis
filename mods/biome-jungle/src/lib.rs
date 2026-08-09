use tokio::task::JoinHandle;

pub struct BiomeJungleMod;

impl BiomeJungleMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
