use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DiamondBlockBlock;

impl Block for DiamondBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:diamond-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DiamondBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-diamond-block:block/diamond_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DiamondBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DiamondBlockBlock::RENDER;

pub struct BlockDiamondBlockMod;

impl BlockDiamondBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
