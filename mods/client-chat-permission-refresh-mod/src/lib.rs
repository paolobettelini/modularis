use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_chat_api::{
    ClientChatApi, ClientChatComposer, ClientChatSet, ClientChatSuggestionsRequested,
};
use client_player_permission_api::{
    ClientPlayerPermissionApi, ClientPlayerPermissionSet, ClientPlayerPermissionsChanged,
};
use tokio::task::JoinHandle;

/// Generic adapter from permission-state changes to permission-dependent chat
/// completion state. It deliberately knows nothing about commands which may
/// cause a permission change.
pub struct ClientChatPermissionRefreshMod;

impl ClientChatPermissionRefreshMod {
    pub fn init<C: ClientChatApi, P: ClientPlayerPermissionApi>(
        bevy: &mut BevyMod,
        _chat: &mut C,
        _permissions: &mut P,
    ) -> Self {
        bevy.app.add_systems(
            Update,
            refresh_command_suggestions
                .in_set(ClientPlayerPermissionSet::React)
                .after(ClientChatSet::Input)
                .before(ClientChatSet::Send),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> { None }
}

fn refresh_command_suggestions(
    mut changes: MessageReader<ClientPlayerPermissionsChanged>,
    mut composer: ResMut<ClientChatComposer>,
    mut requests: MessageWriter<ClientChatSuggestionsRequested>,
) {
    // Every permission mutation reaches this path through the synchronized
    // permission snapshot. Advancing the generation invalidates responses
    // which were produced against the old authorization state.
    if changes.read().last().is_none() {
        return;
    }

    composer.latest_request_id = composer.latest_request_id.wrapping_add(1);
    composer.suggestions.clear();
    composer.selected_suggestion = None;
    info!(
        "permission state changed; invalidated command completion generation {}",
        composer.latest_request_id
    );

    if composer.input.starts_with('/') {
        requests.write(ClientChatSuggestionsRequested {
            request_id: composer.latest_request_id,
            cursor: composer.input.len(),
            input: composer.input.clone(),
        });
    }
}
