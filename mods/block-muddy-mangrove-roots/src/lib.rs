use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MuddyMangroveRootsBlock;

impl Block for MuddyMangroveRootsBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:muddy-mangrove-roots",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MuddyMangroveRootsBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-muddy-mangrove-roots:block/muddy_mangrove_roots"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MuddyMangroveRootsBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MuddyMangroveRootsBlock::RENDER;

pub struct BlockMuddyMangroveRootsMod;

impl BlockMuddyMangroveRootsMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
