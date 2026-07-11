use tokio::task::JoinHandle;

pub struct DimensionRegistryCodegenMod;

impl DimensionRegistryCodegenMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
