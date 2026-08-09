use tokio::task::JoinHandle;

pub struct BiomeCherryGroveMod;

impl BiomeCherryGroveMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
