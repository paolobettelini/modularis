use tokio::task::JoinHandle;

pub struct ClientSettingGrassBrightnessMod;

impl ClientSettingGrassBrightnessMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
