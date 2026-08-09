use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ResinBlockBlock;

impl Block for ResinBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:resin-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ResinBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-resin-block:block/resin_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ResinBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ResinBlockBlock::RENDER;

pub struct BlockResinBlockMod;

impl BlockResinBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
