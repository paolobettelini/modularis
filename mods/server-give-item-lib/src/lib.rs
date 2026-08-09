use block_manager_api::BlockManagerApi;
use inventory_core_api::{
    Inventory, InventoryCell, InventorySectionLayout, InventorySectionRole,
};
use inventory_quantity_operations_lib::merge_compatible_items;
use item_instance_api::{ItemId, ItemInstance};
use item_manager_api::ItemManagerApi;
use item_place_block_meta::PlaceBlock;
use item_portal_igniter_meta::PortalIgniter;
use item_quantity_meta::Quantity;

#[derive(Debug, Clone)]
pub struct GiveItemPlan {
    pub cell: InventoryCell,
    pub item: ItemInstance,
}

/// Builds the vanilla metadata attached to an item granted by `/give`.
///
/// This is deliberately a library function rather than command policy: a
/// custom server can call it conditionally, replace it, or construct custom
/// metadata before planning the inventory insertion.
pub fn vanilla_granted_item<I: ItemManagerApi, B: BlockManagerApi>(
    item: ItemId,
    amount: u32,
) -> ItemInstance {
    let mut metadata = generated_item_metadata::ItemMetaSet {
        quantity: Some(Quantity::Finite(amount)),
        ..Default::default()
    };
    let item_id = I::id(item);
    if item_id == "demo:flint-and-steel" {
        metadata.portal_igniter = Some(PortalIgniter::new());
    }
    if let Some(block) = block_for_item::<B>(item_id) {
        metadata.place_block = Some(PlaceBlock { block });
    }
    ItemInstance::with_metadata(item, metadata)
}

/// Plans one atomic cell update, preferring an existing compatible stack and
/// then an empty storage cell before the hotbar.
pub fn plan_give_item(
    inventory: &Inventory,
    granted: ItemInstance,
) -> Result<GiveItemPlan, &'static str> {
    let sections = ordered_sections(inventory);
    for section in &sections {
        for index in 0..section.cells {
            let cell = InventoryCell {
                section: section.id.clone(),
                index,
            };
            let Some(existing) = inventory.get(&cell) else {
                continue;
            };
            if let Some(item) = merge_compatible_items(&granted, existing) {
                return Ok(GiveItemPlan { cell, item });
            }
        }
    }
    for section in sections {
        for index in 0..section.cells {
            let cell = InventoryCell {
                section: section.id.clone(),
                index,
            };
            if inventory.get(&cell).is_none() {
                return Ok(GiveItemPlan {
                    cell,
                    item: granted,
                });
            }
        }
    }
    Err("Target inventory is full")
}

fn ordered_sections(inventory: &Inventory) -> Vec<&InventorySectionLayout> {
    let mut sections = inventory.layout.sections.iter().collect::<Vec<_>>();
    sections.sort_by_key(|section| match section.role {
        InventorySectionRole::Storage => 0,
        InventorySectionRole::Hotbar => 1,
    });
    sections
}

fn block_for_item<B: BlockManagerApi>(item_id: &str) -> Option<block_manager_api::BlockId> {
    let (namespace, local) = item_id.split_once(':')?;
    let direct = format!("{namespace}:{}", local.replace('_', "-"));
    if let Some(block) = B::from_string(&direct) {
        return Some(block);
    }
    let local = local.strip_suffix("_block")?;
    B::from_string(&format!("{namespace}:{}", local.replace('_', "-")))
}
