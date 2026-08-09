use tokio::task::JoinHandle;

pub struct BiomeMangroveMarshMod;

impl BiomeMangroveMarshMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
