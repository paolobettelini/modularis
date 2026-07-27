use item_instance_api::ItemInstance;
use item_quantity_meta::Quantity;

pub fn merge_compatible_items(
    source: &ItemInstance,
    target: &ItemInstance,
) -> Option<ItemInstance> {
    if source.item != target.item {
        return None;
    }
    let (Some(source_quantity), Some(target_quantity)) =
        (source.metadata.quantity, target.metadata.quantity)
    else {
        return None;
    };
    let mut source_without_quantity = source.clone();
    let mut target_without_quantity = target.clone();
    source_without_quantity.metadata.quantity = None;
    target_without_quantity.metadata.quantity = None;
    if source_without_quantity.metadata != target_without_quantity.metadata {
        return None;
    }
    let mut merged = target.clone();
    merged.metadata.quantity = Some(source_quantity.merge(target_quantity));
    Some(merged)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantityConsumption {
    NotApplicable,
    Unchanged,
    Remove,
    Replace(ItemInstance),
}

pub fn consume_one(current: &ItemInstance, item_before_use: &ItemInstance) -> QuantityConsumption {
    if current.item != item_before_use.item {
        return QuantityConsumption::NotApplicable;
    }
    let Some(quantity) = current.metadata.quantity else {
        return QuantityConsumption::NotApplicable;
    };
    match quantity.after_one_use() {
        Some(Quantity::Infinite) => QuantityConsumption::Unchanged,
        Some(quantity) => {
            let mut next = current.clone();
            next.metadata.quantity = Some(quantity);
            QuantityConsumption::Replace(next)
        }
        None => QuantityConsumption::Remove,
    }
}
