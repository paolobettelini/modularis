use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RawGoldBlockBlock;

impl Block for RawGoldBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:raw-gold-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RawGoldBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-raw-gold-block:block/raw_gold_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RawGoldBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RawGoldBlockBlock::RENDER;

pub struct BlockRawGoldBlockMod;

impl BlockRawGoldBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
