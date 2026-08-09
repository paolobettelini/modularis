use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct IronBlockItem;

impl Item for IronBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:iron_block",
        label: "Iron Block",
    };
}

impl ItemRender for IronBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-iron-block:item/iron_block"),
    };
}

pub const ITEM_INFO: ItemInfo = IronBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <IronBlockItem as ItemRender>::RENDER;

pub struct ItemIronBlockMod;

impl ItemIronBlockMod {
    pub fn init(_block: &mut block_iron_block::BlockIronBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
