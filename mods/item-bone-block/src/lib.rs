use item_api::{Item, ItemInfo};
use item_render_api::{ItemRender, ItemRenderInfo};
use tokio::task::JoinHandle;

pub struct BoneBlockItem;

impl Item for BoneBlockItem {
    const INFO: ItemInfo = ItemInfo {
        id: "demo:bone_block",
        label: "Bone Block",
    };
}

impl ItemRender for BoneBlockItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-bone-block:item/bone_block"),
    };
}

pub const ITEM_INFO: ItemInfo = BoneBlockItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo = <BoneBlockItem as ItemRender>::RENDER;

pub struct ItemBoneBlockMod;

impl ItemBoneBlockMod {
    pub fn init(_block: &mut block_bone_block::BlockBoneBlockMod) -> Self {
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
