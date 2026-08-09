use tokio::task::JoinHandle;

pub struct PermissionRegistryCodegenMod;

impl PermissionRegistryCodegenMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
