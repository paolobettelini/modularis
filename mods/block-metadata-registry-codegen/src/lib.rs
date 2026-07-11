use tokio::task::JoinHandle;

pub struct BlockMetadataRegistryCodegenMod;

impl BlockMetadataRegistryCodegenMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
