use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct JungleLeavesBlockItem;

impl Item for JungleLeavesBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:jungle_leaves_block",
        label: "Jungle Leaves",
    };
}

impl ItemRender for JungleLeavesBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-jungle-leaves-block:item/jungle_leaves_block"),
    };
}

pub const ITEM_INFO: ItemInfo = JungleLeavesBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <JungleLeavesBlockItem as ItemRender>::RENDER;

pub struct ItemJungleLeavesBlockMod;

impl ItemJungleLeavesBlockMod {
    pub fn init(_block: &mut block_jungle_leaves::BlockJungleLeavesMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
