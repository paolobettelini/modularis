use block_api::{Block, BlockInfo};
use block_render_api::{BlockRender, BlockRenderInfo, BlockTextures, RenderShape};
use tokio::task::JoinHandle;

pub struct CraftingTableBlock;

impl Block for CraftingTableBlock {
    const INFO: BlockInfo = BlockInfo {
        id: "demo:crafting-table",
        is_air: false,
        solid: true,
        opaque: true,
    };
}

impl BlockRender for CraftingTableBlock {
    const RENDER: BlockRenderInfo = BlockRenderInfo {
        shape: RenderShape::Cube,
        textures: Some(BlockTextures::PerFace {
            east: "block-crafting-table/crafting_table_side.png",
            west: "block-crafting-table/crafting_table_side.png",
            top: "block-crafting-table/crafting_table_top.png",
            bottom: "block-crafting-table/crafting_table_side.png",
            south: "block-crafting-table/crafting_table_front.png",
            north: "block-crafting-table/crafting_table_side.png",
        }),
    };
}

pub const BLOCK_INFO: BlockInfo = CraftingTableBlock::INFO;
pub const RENDER_INFO: BlockRenderInfo = CraftingTableBlock::RENDER;

pub struct BlockCraftingTableMod;

impl BlockCraftingTableMod {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
