use tokio::task::JoinHandle;

pub struct BiomeRegistryCodegenMod;

impl BiomeRegistryCodegenMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
