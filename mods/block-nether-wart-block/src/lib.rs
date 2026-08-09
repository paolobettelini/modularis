use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct NetherWartBlockBlock;

impl Block for NetherWartBlockBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:nether-wart-block",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for NetherWartBlockBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-nether-wart-block:block/nether_wart_block"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = NetherWartBlockBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = NetherWartBlockBlock::RENDER;

pub struct BlockNetherWartBlockMod;

impl BlockNetherWartBlockMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
