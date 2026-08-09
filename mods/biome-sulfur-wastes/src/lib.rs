use tokio::task::JoinHandle;

pub struct BiomeSulfurWastesMod;

impl BiomeSulfurWastesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
