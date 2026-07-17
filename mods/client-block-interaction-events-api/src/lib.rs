use bevy::prelude::*;
use item_use_api::ItemUseTarget;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientBlockInteractionSet {
    Raycast,
    /// Optional rules may route an intent before block-specific handlers see
    /// it. Sneak uses this phase to prefer the held item action.
    RoutingRules,
    SpecificHandlers,
    Fallback,
}

#[derive(Message, Debug, Clone)]
pub struct LocalBlockUseIntent {
    pub operation_id: u64,
    pub target: ItemUseTarget,
}

#[derive(Message, Debug, Clone)]
pub struct LocalBlockUseHandled {
    pub operation_id: u64,
}
