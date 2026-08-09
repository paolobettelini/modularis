use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct IceBlock;

impl Block for IceBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:ice",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for IceBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-ice:block/ice"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = IceBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = IceBlock::RENDER;

pub struct BlockIceMod;

impl BlockIceMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
