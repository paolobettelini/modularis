use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct ChiseledSandstoneBlock;

impl Block for ChiseledSandstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:chiseled-sandstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for ChiseledSandstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-chiseled-sandstone:block/chiseled_sandstone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = ChiseledSandstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = ChiseledSandstoneBlock::RENDER;

pub struct BlockChiseledSandstoneMod;

impl BlockChiseledSandstoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
