use tokio::task::JoinHandle;

pub struct BiomePaleGardenMod;

impl BiomePaleGardenMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
