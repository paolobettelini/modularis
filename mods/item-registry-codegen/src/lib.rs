use tokio::task::JoinHandle;

pub struct ItemRegistryCodegenMod;

impl ItemRegistryCodegenMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
