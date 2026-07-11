use bevy::prelude::App;
use tokio::task::JoinHandle;

pub struct BevyMod {
    pub app: App,
}

impl BevyMod {
    pub fn init() -> Self {
        Self { app: App::new() }
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
