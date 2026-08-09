use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MangrovePlanksBlockItem;

impl Item for MangrovePlanksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:mangrove_planks_block",
        label: "Mangrove Planks",
    };
}

impl ItemRender for MangrovePlanksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-mangrove-planks-block:item/mangrove_planks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MangrovePlanksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MangrovePlanksBlockItem as ItemRender>::RENDER;

pub struct ItemMangrovePlanksBlockMod;

impl ItemMangrovePlanksBlockMod {
    pub fn init(_block: &mut block_mangrove_planks::BlockMangrovePlanksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
