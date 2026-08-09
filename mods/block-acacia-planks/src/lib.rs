use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct AcaciaPlanksBlock;

impl Block for AcaciaPlanksBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:acacia-planks",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for AcaciaPlanksBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-acacia-planks:block/acacia_planks"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = AcaciaPlanksBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = AcaciaPlanksBlock::RENDER;

pub struct BlockAcaciaPlanksMod;

impl BlockAcaciaPlanksMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
