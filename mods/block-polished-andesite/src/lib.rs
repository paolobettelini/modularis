use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct PolishedAndesiteBlock;

impl Block for PolishedAndesiteBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:polished-andesite",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for PolishedAndesiteBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-polished-andesite:block/polished_andesite"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = PolishedAndesiteBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = PolishedAndesiteBlock::RENDER;

pub struct BlockPolishedAndesiteMod;

impl BlockPolishedAndesiteMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
