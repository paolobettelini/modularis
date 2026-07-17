use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_inventory_ui_api::InventoryItemNameVisual;
use inventory_events_api::{InventoryClientRenderSet, InventorySlotVisualCreated};
use inventory_events_mod::InventoryEventsMod;
use item_manager_api::ItemManagerApi;
use std::marker::PhantomData;
use tokio::task::JoinHandle;
use voxel_model_api::{VoxelModelApi, VoxelModelService};

pub struct ClientItemModelUiMod<I>(PhantomData<I>);

impl<I: ItemManagerApi> ClientItemModelUiMod<I> {
    pub fn init<M: VoxelModelApi>(
        bevy: &mut BevyMod,
        _events: &mut InventoryEventsMod,
        _inventory_ui: &mut impl client_inventory_ui_api::ClientInventoryUiApi,
        _items: &mut I,
        _models: &mut M,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            render_item_models::<I>.in_set(InventoryClientRenderSet::Decorations),
        );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn render_item_models<I: ItemManagerApi>(
    mut commands: Commands,
    assets: Res<AssetServer>,
    models: Res<VoxelModelService>,
    mut visuals: MessageReader<InventorySlotVisualCreated>,
    children: Query<&Children>,
    names: Query<Entity, With<InventoryItemNameVisual>>,
) {
    for event in visuals.read() {
        // Instance metadata is an explicit runtime override. The separate
        // favicon mod owns that path when it is present.
        if event.item.metadata.favicon.is_some() {
            continue;
        }
        let Some(model_id) = I::render_info(event.item.item).model else {
            continue;
        };
        let Ok(model) = models.bake(model_id) else {
            continue;
        };
        let Some(texture) = model.first().map(|quad| &quad.texture) else {
            continue;
        };

        if let Ok(children) = children.get(event.entity) {
            for child in children.iter() {
                if names.get(child).is_ok() {
                    commands.entity(child).despawn();
                }
            }
        }

        let texture = VoxelModelService::texture_asset_path(texture);
        commands.entity(event.entity).with_child((
            ImageNode::new(assets.load(texture)),
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
