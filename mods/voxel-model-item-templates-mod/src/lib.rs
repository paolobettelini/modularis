use tokio::task::JoinHandle;

/// Asset-only generated and handheld item parents.
pub struct VoxelModelItemTemplatesMod;

impl VoxelModelItemTemplatesMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
