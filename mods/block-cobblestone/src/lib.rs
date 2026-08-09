use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CobblestoneBlock;

impl Block for CobblestoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cobblestone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CobblestoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cobblestone:block/cobblestone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CobblestoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CobblestoneBlock::RENDER;

pub struct BlockCobblestoneMod;

impl BlockCobblestoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
