use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DioriteBlock;

impl Block for DioriteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:diorite",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DioriteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-diorite:block/diorite"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DioriteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DioriteBlock::RENDER;

pub struct BlockDioriteMod;

impl BlockDioriteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
