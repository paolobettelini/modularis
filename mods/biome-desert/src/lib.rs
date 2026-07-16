use tokio::task::JoinHandle;

pub struct BiomeDesertMod;

impl BiomeDesertMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
