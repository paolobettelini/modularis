use tokio::task::JoinHandle;

pub struct DimensionAetherMod;

impl DimensionAetherMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
