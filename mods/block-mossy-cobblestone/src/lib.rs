use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MossyCobblestoneBlock;

impl Block for MossyCobblestoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:mossy-cobblestone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MossyCobblestoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-mossy-cobblestone:block/mossy_cobblestone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MossyCobblestoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MossyCobblestoneBlock::RENDER;

pub struct BlockMossyCobblestoneMod;

impl BlockMossyCobblestoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
