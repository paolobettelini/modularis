use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct DeepslateBlock;

impl Block for DeepslateBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:deepslate",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for DeepslateBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-deepslate:block/deepslate"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = DeepslateBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = DeepslateBlock::RENDER;

pub struct BlockDeepslateMod;

impl BlockDeepslateMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
