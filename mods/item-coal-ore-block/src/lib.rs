use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct CoalOreBlockItem;

impl Item for CoalOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:coal_ore_block",
        label: "Coal Ore",
    };
}

impl ItemRender for CoalOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-coal-ore-block:item/coal_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = CoalOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <CoalOreBlockItem as ItemRender>::RENDER;

pub struct ItemCoalOreBlockMod;

impl ItemCoalOreBlockMod {
    pub fn init(_block: &mut block_coal_ore::BlockCoalOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
