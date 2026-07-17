use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CraftingTableBlockItem;

impl Item for CraftingTableBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:crafting_table_block",
        label: "Crafting Table",
    };
}

impl ItemRender for CraftingTableBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-crafting-table-block:item/crafting_table_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CraftingTableBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CraftingTableBlockItem as ItemRender>::RENDER;

pub struct ItemCraftingTableBlockMod;

impl ItemCraftingTableBlockMod {
    pub fn init(_block: &mut block_crafting_table::BlockCraftingTableMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
