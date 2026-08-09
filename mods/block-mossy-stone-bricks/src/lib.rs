use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MossyStoneBricksBlock;

impl Block for MossyStoneBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:mossy-stone-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MossyStoneBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-mossy-stone-bricks:block/mossy_stone_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MossyStoneBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MossyStoneBricksBlock::RENDER;

pub struct BlockMossyStoneBricksMod;

impl BlockMossyStoneBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
