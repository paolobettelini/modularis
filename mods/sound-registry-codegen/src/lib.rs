use tokio::task::JoinHandle;

pub struct SoundRegistryCodegenMod;

impl SoundRegistryCodegenMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
