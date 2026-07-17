use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct NetherrackBlock;

impl Block for NetherrackBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:netherrack",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for NetherrackBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-netherrack:block/netherrack"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = NetherrackBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = NetherrackBlock::RENDER;

pub struct BlockNetherrackMod;

impl BlockNetherrackMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
