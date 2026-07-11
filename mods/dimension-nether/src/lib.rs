use tokio::task::JoinHandle;

pub struct DimensionNetherMod;

impl DimensionNetherMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
