use tokio::task::JoinHandle;

pub struct BiomeAetherGoldenGroveMod;

impl BiomeAetherGoldenGroveMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
