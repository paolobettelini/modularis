use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OakStairsBlock;

impl Block for OakStairsBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:oak-stairs",
        is_air: false,
        solid: true,
        opaque: false,
    };
}

impl BlockRender for OakStairsBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-oak-stairs:block/oak_stairs"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OakStairsBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OakStairsBlock::RENDER;

pub struct BlockOakStairsMod;

impl BlockOakStairsMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
