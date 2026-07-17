use tokio::task::JoinHandle;

pub struct BiomeBirchForestMod;

impl BiomeBirchForestMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
