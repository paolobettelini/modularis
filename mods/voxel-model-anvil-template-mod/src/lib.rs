use tokio::task::JoinHandle;

pub struct VoxelModelAnvilTemplateMod;

impl VoxelModelAnvilTemplateMod {
    pub fn init(
        _block_templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
