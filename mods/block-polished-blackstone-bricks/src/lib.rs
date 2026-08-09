use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PolishedBlackstoneBricksBlock;

impl Block for PolishedBlackstoneBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:polished-blackstone-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PolishedBlackstoneBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-polished-blackstone-bricks:block/polished_blackstone_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PolishedBlackstoneBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PolishedBlackstoneBricksBlock::RENDER;

pub struct BlockPolishedBlackstoneBricksMod;

impl BlockPolishedBlackstoneBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
