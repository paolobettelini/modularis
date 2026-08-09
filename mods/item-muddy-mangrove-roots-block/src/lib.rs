use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct MuddyMangroveRootsBlockItem;

impl Item for MuddyMangroveRootsBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:muddy_mangrove_roots_block",
        label: "Muddy Mangrove Roots",
    };
}

impl ItemRender for MuddyMangroveRootsBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-muddy-mangrove-roots-block:item/muddy_mangrove_roots_block"),
    };
}

pub const ITEM_INFO: ItemInfo = MuddyMangroveRootsBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <MuddyMangroveRootsBlockItem as ItemRender>::RENDER;

pub struct ItemMuddyMangroveRootsBlockMod;

impl ItemMuddyMangroveRootsBlockMod {
    pub fn init(_block: &mut block_muddy_mangrove_roots::BlockMuddyMangroveRootsMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
