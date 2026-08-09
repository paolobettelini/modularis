use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CutSandstoneBlock;

impl Block for CutSandstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cut-sandstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CutSandstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cut-sandstone:block/cut_sandstone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CutSandstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CutSandstoneBlock::RENDER;

pub struct BlockCutSandstoneMod;

impl BlockCutSandstoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
