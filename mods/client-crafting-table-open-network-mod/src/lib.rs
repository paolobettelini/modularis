use bevy::prelude::*;
use bevy_mod::BevyMod;
use cell_menu_network_message_types::{CellMenuOpenRequest, CellMenuRequest};
use client_block_interaction_events_api::{
    ClientBlockInteractionSet, LocalBlockUseHandled, LocalBlockUseIntent,
};
use client_block_interaction_events_mod::ClientBlockInteractionEventsMod;
use client_chunk_cache_api::{ClientChunkCache, ClientChunkCacheApi};
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use generated_block_registry::BlockId;
use generated_network_messages::ServerBoundMessage;
use item_use_api::ItemUseTarget;
use network_protocol_mod::NetworkProtocolMod;
use std::collections::HashSet;
use tokio::task::JoinHandle;

const CRAFTING_TABLE_KIND: &str = "demo:crafting-table";

pub struct ClientCraftingTableOpenNetworkMod;

impl ClientCraftingTableOpenNetworkMod {
    pub fn init<N: ClientNetworkApi, C: ClientChunkCacheApi>(
        bevy: &mut BevyMod,
        _interaction_events: &mut ClientBlockInteractionEventsMod,
        _cache: &mut C,
        _network: &mut N,
        _protocol: &mut NetworkProtocolMod,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            request_crafting_table_menu.in_set(ClientBlockInteractionSet::SpecificHandlers),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn request_crafting_table_menu(
    cache: Res<ClientChunkCache>,
    sender: Option<Res<ClientNetworkSender>>,
    mut intents: MessageReader<LocalBlockUseIntent>,
    mut handled: ParamSet<(
        MessageReader<LocalBlockUseHandled>,
        MessageWriter<LocalBlockUseHandled>,
    )>,
) {
    let Some(sender) = sender else {
        return;
    };
    let already_handled = handled
        .p0()
        .read()
        .map(|event| event.operation_id)
        .collect::<HashSet<_>>();
    for intent in intents.read() {
        if already_handled.contains(&intent.operation_id) {
            continue;
        }
        let ItemUseTarget::Block { hit, .. } = intent.target else {
            continue;
        };
        if !cache
            .block(hit)
            .is_some_and(|block| block.block == BlockId::CraftingTable)
        {
            continue;
        }
        let _ = sender.send(&ServerBoundMessage::CellMenuRequest(CellMenuRequest::Open(
            CellMenuOpenRequest {
                kind: CRAFTING_TABLE_KIND.to_string(),
                anchor: Some(hit),
            },
        )));
        handled.p1().write(LocalBlockUseHandled {
            operation_id: intent.operation_id,
        });
    }
}
