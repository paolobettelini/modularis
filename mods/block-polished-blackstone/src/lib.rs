use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PolishedBlackstoneBlock;

impl Block for PolishedBlackstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:polished-blackstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PolishedBlackstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-polished-blackstone:block/polished_blackstone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PolishedBlackstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PolishedBlackstoneBlock::RENDER;

pub struct BlockPolishedBlackstoneMod;

impl BlockPolishedBlackstoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
