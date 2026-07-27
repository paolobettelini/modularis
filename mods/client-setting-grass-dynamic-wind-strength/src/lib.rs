use tokio::task::JoinHandle;

pub struct ClientSettingGrassDynamicWindStrengthMod;

impl ClientSettingGrassDynamicWindStrengthMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
