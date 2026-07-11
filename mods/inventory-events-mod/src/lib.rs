use bevy::prelude::*;
use bevy_mod::BevyMod;
use inventory_events_api::*;
use tokio::task::JoinHandle;

pub struct InventoryEventsMod;

impl InventoryEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .add_message::<LocalInventoryMoveIntent>()
            .add_message::<LocalHotbarSelectIntent>()
            .add_message::<LocalUseHeldItemIntent>()
            .add_message::<InventoryMoveRequested>()
            .add_message::<InventoryMoveHandled>()
            .add_message::<HotbarSelectRequested>()
            .add_message::<UseHeldItemRequested>()
            .add_message::<InventorySyncRequested>()
            .add_message::<InventoryResetRequested>()
            .add_message::<InventoryResizeRequested>()
            .add_message::<InventorySetCellRequested>()
            .add_message::<InventoryResetApplied>()
            .add_message::<InventoryResized>()
            .add_message::<InventoryCellSet>()
            .add_message::<HotbarSelectionSet>()
            .add_message::<HeldItemUseDispatched>()
            .add_message::<ItemUseSucceeded>()
            .add_message::<ClientInventoryReset>()
            .add_message::<ClientInventoryResized>()
            .add_message::<ClientInventoryCellSet>()
            .add_message::<ClientHotbarSelectionSet>()
            .add_message::<InventorySlotVisualCreated>()
            .configure_sets(
                Update,
                (
                    InventoryServerSet::ReceiveRequest,
                    InventoryServerSet::Validate,
                    InventoryServerSet::DispatchUse,
                    InventoryServerSet::ApplyWorldEffects,
                    InventoryServerSet::ApplyConsumption,
                    InventoryServerSet::Sync,
                )
                    .chain(),
            )
            .configure_sets(
                Update,
                (
                    InventoryValidationSet::Initialize,
                    InventoryValidationSet::Stack,
                    InventoryValidationSet::MoveOrSwap,
                    InventoryValidationSet::Other,
                )
                    .chain()
                    .in_set(InventoryServerSet::Validate),
            )
            .configure_sets(
                Update,
                (
                    InventoryClientSet::ReceiveSync,
                    InventoryClientSet::ApplyCache,
                    InventoryClientSet::Render,
                )
                    .chain(),
            )
            .configure_sets(
                Update,
                (
                    InventoryClientCacheSet::AuthoritativeSync,
                    InventoryClientCacheSet::OptimisticPreview,
                )
                    .chain()
                    .in_set(InventoryClientSet::ApplyCache),
            )
            .configure_sets(
                Update,
                (
                    InventoryClientRenderSet::Layout,
                    InventoryClientRenderSet::Decorations,
                )
                    .chain()
                    .in_set(InventoryClientSet::Render),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
