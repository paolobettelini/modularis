use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct NetherQuartzOreBlockItem;

impl Item for NetherQuartzOreBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:nether_quartz_ore_block",
        label: "Nether Quartz Ore",
    };
}

impl ItemRender for NetherQuartzOreBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-nether-quartz-ore-block:item/nether_quartz_ore_block"),
    };
}

pub const ITEM_INFO: ItemInfo = NetherQuartzOreBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <NetherQuartzOreBlockItem as ItemRender>::RENDER;

pub struct ItemNetherQuartzOreBlockMod;

impl ItemNetherQuartzOreBlockMod {
    pub fn init(_block: &mut block_nether_quartz_ore::BlockNetherQuartzOreMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
