use tokio::task::JoinHandle;

pub struct BiomeDarkForestMod;

impl BiomeDarkForestMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
