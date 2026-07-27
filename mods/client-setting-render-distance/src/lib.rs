use tokio::task::JoinHandle;

pub struct SettingRenderDistanceMod;

impl SettingRenderDistanceMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
