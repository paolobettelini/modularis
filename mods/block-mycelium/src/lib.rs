use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, RenderShape};
use tokio::task::JoinHandle;

pub struct MyceliumBlock;

impl Block for MyceliumBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:mycelium",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for MyceliumBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Model,
        model: Some("block-mycelium:block/mycelium"),
        textures: None,
    };
}

pub const BLOCK_INFO: BlockInfo = MyceliumBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = MyceliumBlock::RENDER;

pub struct BlockMyceliumMod;

impl BlockMyceliumMod {
    pub fn init(
        _templates: &mut voxel_model_block_templates_mod::VoxelModelBlockTemplatesMod,
    ) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
