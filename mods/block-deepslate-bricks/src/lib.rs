use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeepslateBricksBlock;

impl Block for DeepslateBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:deepslate-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeepslateBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-deepslate-bricks:block/deepslate_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeepslateBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeepslateBricksBlock::RENDER;

pub struct BlockDeepslateBricksMod;

impl BlockDeepslateBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
