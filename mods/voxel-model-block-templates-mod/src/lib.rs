use tokio::task::JoinHandle;

/// Asset-only parent models shared through formal Patchwork dependencies.
pub struct VoxelModelBlockTemplatesMod;

impl VoxelModelBlockTemplatesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
