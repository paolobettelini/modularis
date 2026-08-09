use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct JunglePlanksBlockItem;

impl Item for JunglePlanksBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:jungle_planks_block",
        label: "Jungle Planks",
    };
}

impl ItemRender for JunglePlanksBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-jungle-planks-block:item/jungle_planks_block"),
    };
}

pub const ITEM_INFO: ItemInfo = JunglePlanksBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <JunglePlanksBlockItem as ItemRender>::RENDER;

pub struct ItemJunglePlanksBlockMod;

impl ItemJunglePlanksBlockMod {
    pub fn init(_block: &mut block_jungle_planks::BlockJunglePlanksMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
