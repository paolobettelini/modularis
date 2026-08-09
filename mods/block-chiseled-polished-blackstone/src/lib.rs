use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ChiseledPolishedBlackstoneBlock;

impl Block for ChiseledPolishedBlackstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:chiseled-polished-blackstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ChiseledPolishedBlackstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-chiseled-polished-blackstone:block/chiseled_polished_blackstone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ChiseledPolishedBlackstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ChiseledPolishedBlackstoneBlock::RENDER;

pub struct BlockChiseledPolishedBlackstoneMod;

impl BlockChiseledPolishedBlackstoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
