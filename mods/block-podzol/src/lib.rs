use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PodzolBlock;

impl Block for PodzolBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:podzol",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PodzolBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-podzol:block/podzol"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PodzolBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PodzolBlock::RENDER;

pub struct BlockPodzolMod;

impl BlockPodzolMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
