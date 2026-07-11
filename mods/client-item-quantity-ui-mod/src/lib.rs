use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_ui_font_api::{ClientUiFont, ClientUiFontApi};
use inventory_events_api::{InventoryClientRenderSet, InventorySlotVisualCreated};
use inventory_events_mod::InventoryEventsMod;
use item_quantity_meta::Quantity;
use tokio::task::JoinHandle;

pub struct ClientItemQuantityUiMod;

impl ClientItemQuantityUiMod {
    pub fn init<F: ClientUiFontApi>(
        bevy: &mut BevyMod,
        _events: &mut InventoryEventsMod,
        _font: &mut F,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            render_quantities.in_set(InventoryClientRenderSet::Decorations),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn render_quantities(
    mut commands: Commands,
    font: Res<ClientUiFont>,
    mut visuals: MessageReader<InventorySlotVisualCreated>,
) {
    for event in visuals.read() {
        let Some(quantity) = event.item.metadata.quantity else {
            continue;
        };
        let label = match quantity {
            Quantity::Finite(value) => value.to_string(),
            Quantity::Infinite => "∞".to_string(),
        };
        commands.entity(event.entity).with_child((
            Text::new(label),
            TextFont {
                font: font.0.clone(),
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextShadow {
                offset: Vec2::new(1.0, 1.0),
                color: Color::BLACK,
            },
            Node {
                position_type: PositionType::Absolute,
                right: px(2),
                bottom: px(2),
                min_width: px(18),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            ZIndex(20),
            Pickable::IGNORE,
        ));
    }
}
