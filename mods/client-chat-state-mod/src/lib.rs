use bevy::prelude::*;
use bevy_mod::BevyMod;
use client_chat_api::{
    ClientChatApi, ClientChatComposer, ClientChatLog, ClientChatMessageReceived, ClientChatSet,
    ClientChatSubmitRequested, ClientChatSuggestionsReceived, ClientChatSuggestionsRequested,
};
use tokio::task::JoinHandle;

pub struct ClientChatStateMod;

impl ClientChatStateMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app
            .init_resource::<ClientChatLog>()
            .init_resource::<ClientChatComposer>()
            .add_message::<ClientChatSubmitRequested>()
            .add_message::<ClientChatSuggestionsRequested>()
            .add_message::<ClientChatMessageReceived>()
            .add_message::<ClientChatSuggestionsReceived>()
            .configure_sets(
                Update,
                (
                    ClientChatSet::Receive,
                    ClientChatSet::Apply,
                    ClientChatSet::Input,
                    ClientChatSet::Send,
                    ClientChatSet::Render,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (apply_messages, apply_suggestions).in_set(ClientChatSet::Apply),
            );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

impl ClientChatApi for ClientChatStateMod {}

fn apply_messages(
    mut received: MessageReader<ClientChatMessageReceived>,
    mut log: ResMut<ClientChatLog>,
) {
    for message in received.read() {
        log.push(message.0.clone());
    }
}

fn apply_suggestions(
    mut received: MessageReader<ClientChatSuggestionsReceived>,
    mut composer: ResMut<ClientChatComposer>,
) {
    for suggestions in received.read() {
        if suggestions.request_id != composer.latest_request_id {
            continue;
        }
        composer.suggestions.clone_from(&suggestions.suggestions);
    }
}
