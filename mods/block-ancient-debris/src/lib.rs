use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct AncientDebrisBlock;

impl Block for AncientDebrisBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:ancient-debris",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for AncientDebrisBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-ancient-debris:block/ancient_debris"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = AncientDebrisBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = AncientDebrisBlock::RENDER;

pub struct BlockAncientDebrisMod;

impl BlockAncientDebrisMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
