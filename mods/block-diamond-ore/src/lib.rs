use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DiamondOreBlock;

impl Block for DiamondOreBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:diamond-ore",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DiamondOreBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-diamond-ore:block/diamond_ore"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DiamondOreBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DiamondOreBlock::RENDER;

pub struct BlockDiamondOreMod;

impl BlockDiamondOreMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
