use tokio::task::JoinHandle;

pub struct BiomeTundraMod;

impl BiomeTundraMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
