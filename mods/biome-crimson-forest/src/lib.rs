use tokio::task::JoinHandle;

pub struct BiomeCrimsonForestMod;

impl BiomeCrimsonForestMod {
    pub fn init() -> Self {
        Self
    }
    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
