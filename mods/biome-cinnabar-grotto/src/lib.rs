use tokio::task::JoinHandle;

pub struct BiomeCinnabarGrottoMod;

impl BiomeCinnabarGrottoMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
