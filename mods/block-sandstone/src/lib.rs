use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct SandstoneBlock;

impl Block for SandstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:sandstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for SandstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-sandstone:block/sandstone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = SandstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = SandstoneBlock::RENDER;

pub struct BlockSandstoneMod;

impl BlockSandstoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
