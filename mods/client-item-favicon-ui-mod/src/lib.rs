use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_inventory_ui_api::InventoryItemNameVisual;
use inventory_events_api::{InventoryClientRenderSet, InventorySlotVisualCreated};
use inventory_events_mod::InventoryEventsMod;
use tokio::task::JoinHandle;

pub struct ClientItemFaviconUiMod;

impl ClientItemFaviconUiMod {
    pub fn init(
        bevy: &mut BevyMod,
        _events: &mut InventoryEventsMod,
        _inventory_ui: &mut impl client_inventory_ui_api::ClientInventoryUiApi,
        _favicon: &mut item_favicon_meta::ItemFaviconMetaMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            render_favicons.in_set(InventoryClientRenderSet::Decorations),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn render_favicons(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut visuals: MessageReader<InventorySlotVisualCreated>,
    children: Query<&Children>,
    names: Query<Entity, With<InventoryItemNameVisual>>,
) {
    for event in visuals.read() {
        let Some(favicon) = event.item.metadata.favicon.as_ref() else {
            continue;
        };
        if let Ok(children) = children.get(event.entity) {
            for child in children.iter() {
                if let Ok(name) = names.get(child) {
                    commands.entity(name).despawn();
                }
            }
        }
        commands.entity(event.entity).with_child((
            ImageNode::new(assets.load(favicon.path.clone())),
            Node {
                width: percent(78),
                height: percent(78),
                ..default()
            },
            ZIndex(2),
            Pickable::IGNORE,
        ));
    }
}
