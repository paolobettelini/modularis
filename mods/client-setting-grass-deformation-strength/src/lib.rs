use tokio::task::JoinHandle;

pub struct ClientSettingGrassDeformationStrengthMod;

impl ClientSettingGrassDeformationStrengthMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
