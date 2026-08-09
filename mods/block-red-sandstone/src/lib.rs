use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct RedSandstoneBlock;

impl Block for RedSandstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:red-sandstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for RedSandstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-red-sandstone:block/red_sandstone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = RedSandstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = RedSandstoneBlock::RENDER;

pub struct BlockRedSandstoneMod;

impl BlockRedSandstoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
