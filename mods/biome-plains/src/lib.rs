use tokio::task::JoinHandle;

pub struct BiomePlainsMod;

impl BiomePlainsMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
