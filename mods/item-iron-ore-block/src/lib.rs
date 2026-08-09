use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct IronOreBlockItem;

impl Item for IronOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:iron_ore_block",
        label: "Iron Ore",
    };
}

impl ItemRender for IronOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-iron-ore-block:item/iron_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = IronOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <IronOreBlockItem as ItemRender>::RENDER;

pub struct ItemIronOreBlockMod;

impl ItemIronOreBlockMod {
    pub fn init(_block: &mut block_iron_ore::BlockIronOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
