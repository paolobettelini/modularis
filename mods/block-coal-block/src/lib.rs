use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CoalBlockBlock;

impl Block for CoalBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:coal-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CoalBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-coal-block:block/coal_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CoalBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CoalBlockBlock::RENDER;

pub struct BlockCoalBlockMod;

impl BlockCoalBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
