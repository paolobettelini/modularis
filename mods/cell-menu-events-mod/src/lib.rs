use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_api::*;
use tokio::task::JoinHandle;

pub struct CellMenuEventsMod;

impl CellMenuEventsMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .add_message::<LocalCellMenuMoveIntent>()
            .add_message::<LocalCellMenuCloseIntent>()
            .add_message::<LocalCellMenuInventoryMoveIntent>()
            .add_message::<CellMenuOpenIntent>()
            .add_message::<CellMenuOpenRequested>()
            .add_message::<CellMenuMoveRequested>()
            .add_message::<CellMenuInventoryMoveRequested>()
            .add_message::<CellMenuCloseRequested>()
            .add_message::<CellMenuOpened>()
            .add_message::<CellMenuClosed>()
            .add_message::<CellMenuCellSet>()
            .add_message::<ClientCellMenuOpened>()
            .add_message::<ClientCellMenuClosed>()
            .add_message::<ClientCellMenuCellSet>()
            .configure_sets(
                Update,
                (
                    CellMenuServerSet::ReceiveRequest,
                    CellMenuServerSet::Validate,
                    CellMenuServerSet::Apply,
                    CellMenuServerSet::Sync,
                )
                    .chain(),
            )
            .configure_sets(
                Update,
                (
                    CellMenuClientSet::ReceiveSync,
                    CellMenuClientSet::ApplyCache,
                    CellMenuClientSet::Render,
                )
                    .chain(),
            )
            .configure_sets(
                Update,
                (
                    CellMenuClientRenderSet::Layout,
                    CellMenuClientRenderSet::Decorations,
                )
                    .chain()
                    .in_set(CellMenuClientSet::Render),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
