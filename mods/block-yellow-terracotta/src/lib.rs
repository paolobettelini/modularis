use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct YellowTerracottaBlock;

impl Block for YellowTerracottaBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:yellow-terracotta",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for YellowTerracottaBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-yellow-terracotta:block/yellow_terracotta"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = YellowTerracottaBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = YellowTerracottaBlock::RENDER;

pub struct BlockYellowTerracottaMod;

impl BlockYellowTerracottaMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
