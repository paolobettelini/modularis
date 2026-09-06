use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_edit_events_api::BlockBreakRequested;
use block_edit_events_mod::BlockEditEventsMod;
use block_edit_network_message_types::BlockBreakRequest;
use client_network_api::{ClientNetworkApi, ClientNetworkSender};
use client_block_interaction_events_api::ClientBlockInteractionSet;
use generated_network_messages::ServerBoundMessage;
use tokio::task::JoinHandle;

pub struct ClientBlockEditNetworkSendMod;

impl ClientBlockEditNetworkSendMod {
    pub fn init<N: ClientNetworkApi>(
        bevy: &mut BevyMod,
        _events: &mut BlockEditEventsMod,
        _network: &mut N,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            send_block_edit_requests.after(ClientBlockInteractionSet::Raycast),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn send_block_edit_requests(
    sender: Option<Res<ClientNetworkSender>>,
    mut breaks: MessageReader<BlockBreakRequested>,
    mut last_logged_break: Local<Option<voxel_frame_api::VoxelBlockAddress>>,
) {
    let Some(sender) = sender else {
        return;
    };
    for request in breaks.read() {
        let _ = sender.send(&ServerBoundMessage::BlockBreakRequest(BlockBreakRequest {
            position: request.position,
        }));
        if *last_logged_break != Some(request.position) {
            info!("sent block-break request for {:?}", request.position);
            *last_logged_break = Some(request.position);
        }
    }
}
