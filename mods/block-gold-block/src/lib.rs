use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GoldBlockBlock;

impl Block for GoldBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:gold-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GoldBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-gold-block:block/gold_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GoldBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GoldBlockBlock::RENDER;

pub struct BlockGoldBlockMod;

impl BlockGoldBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
