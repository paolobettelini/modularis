use tokio::task::JoinHandle;

pub struct SettingsRegistryCodegenMod;

impl SettingsRegistryCodegenMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
