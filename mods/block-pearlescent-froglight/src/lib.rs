use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PearlescentFroglightBlock;

impl Block for PearlescentFroglightBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:pearlescent-froglight",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PearlescentFroglightBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-pearlescent-froglight:block/pearlescent_froglight"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PearlescentFroglightBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PearlescentFroglightBlock::RENDER;

pub struct BlockPearlescentFroglightMod;

impl BlockPearlescentFroglightMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
