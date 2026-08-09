use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PolishedSulfurBlock;

impl Block for PolishedSulfurBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:polished-sulfur",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PolishedSulfurBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-polished-sulfur:block/polished_sulfur"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PolishedSulfurBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PolishedSulfurBlock::RENDER;

pub struct BlockPolishedSulfurMod;

impl BlockPolishedSulfurMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
