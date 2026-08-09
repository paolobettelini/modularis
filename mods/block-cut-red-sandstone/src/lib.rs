use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct CutRedSandstoneBlock;

impl Block for CutRedSandstoneBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:cut-red-sandstone",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CutRedSandstoneBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-cut-red-sandstone:block/cut_red_sandstone"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = CutRedSandstoneBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CutRedSandstoneBlock::RENDER;

pub struct BlockCutRedSandstoneMod;

impl BlockCutRedSandstoneMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
