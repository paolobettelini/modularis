use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ChiseledQuartzBlockBlock;

impl Block for ChiseledQuartzBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:chiseled-quartz-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ChiseledQuartzBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-chiseled-quartz-block:block/chiseled_quartz_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ChiseledQuartzBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ChiseledQuartzBlockBlock::RENDER;

pub struct BlockChiseledQuartzBlockMod;

impl BlockChiseledQuartzBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
