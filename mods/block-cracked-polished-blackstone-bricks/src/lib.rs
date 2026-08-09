use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CrackedPolishedBlackstoneBricksBlock;

impl Block for CrackedPolishedBlackstoneBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cracked-polished-blackstone-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CrackedPolishedBlackstoneBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some(
            "block-cracked-polished-blackstone-bricks:block/cracked_polished_blackstone_bricks",
        ),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CrackedPolishedBlackstoneBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CrackedPolishedBlackstoneBricksBlock::RENDER;

pub struct BlockCrackedPolishedBlackstoneBricksMod;

impl BlockCrackedPolishedBlackstoneBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
