use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MangroveRootsBlock;

impl Block for MangroveRootsBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:mangrove-roots",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MangroveRootsBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-mangrove-roots:block/mangrove_roots"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MangroveRootsBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MangroveRootsBlock::RENDER;

pub struct BlockMangroveRootsMod;

impl BlockMangroveRootsMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
