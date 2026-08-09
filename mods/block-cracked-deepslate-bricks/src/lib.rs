use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CrackedDeepslateBricksBlock;

impl Block for CrackedDeepslateBricksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cracked-deepslate-bricks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CrackedDeepslateBricksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cracked-deepslate-bricks:block/cracked_deepslate_bricks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CrackedDeepslateBricksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CrackedDeepslateBricksBlock::RENDER;

pub struct BlockCrackedDeepslateBricksMod;

impl BlockCrackedDeepslateBricksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
