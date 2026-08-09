use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct GildedBlackstoneBlock;

impl Block for GildedBlackstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:gilded-blackstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for GildedBlackstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-gilded-blackstone:block/gilded_blackstone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = GildedBlackstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = GildedBlackstoneBlock::RENDER;

pub struct BlockGildedBlackstoneMod;

impl BlockGildedBlackstoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
