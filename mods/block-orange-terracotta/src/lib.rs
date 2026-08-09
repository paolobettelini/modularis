use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct OrangeTerracottaBlock;

impl Block for OrangeTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:orange-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for OrangeTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-orange-terracotta:block/orange_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = OrangeTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = OrangeTerracottaBlock::RENDER;

pub struct BlockOrangeTerracottaMod;

impl BlockOrangeTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
