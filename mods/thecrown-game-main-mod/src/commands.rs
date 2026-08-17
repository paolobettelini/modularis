use audience_api::Audience;
use bevy::prelude::*;
use server_chat_api::PublishServerChatMessage;
use server_command_api::{
    ServerCommandRegistry, ServerCommandSource,
    brigadier::{
        arguments::string_argument_type::{get_string, greedy_string},
        builder::{argument_builder::ArgumentBuilder, literal_argument_builder::literal, required_argument_builder::Argument},
        context::CommandContext,
    },
};
use server_external_link_api::PublishServerExternalLink;
use std::sync::{Arc, Mutex};
use thecrown_game_config_api::TheCrownGameConfig;
use thecrown_game_relay_api::{
    RelayPlayerTransferFinished, RelayTransferDestination, RelayWhisperFinished,
    RelayWhisperReceived, RequestRelayPlayerTransfer, RequestRelayWhisper,
    RequestTheCrownWebLogin, TheCrownRelayRequestIds, TheCrownWebLoginFinished,
};
use thecrown_game_session_api::{TheCrownGamePlayerSessions, TransferTheCrownPlayer};
use thecrown_protocol::{DataTransferResult, GameMode};

enum Invocation {
    Transfer { player_id: u64, destination: RelayTransferDestination },
    Whisper { source: ServerCommandSource, arguments: String },
    WebLogin { player_id: u64 },
}

#[derive(Resource, Clone, Default)]
pub(crate) struct CommandQueue(Arc<Mutex<Vec<Invocation>>>);

pub fn register(app: &mut App) {
    let queue = CommandQueue::default();
    let commands = app.world().resource::<ServerCommandRegistry>();

    let hub_queue = queue.0.clone();
    commands.register(literal("hub").executes(move |context: &CommandContext<ServerCommandSource>| {
        hub_queue.lock().expect("TheCrown command queue poisoned").push(Invocation::Transfer {
            player_id: context.source.player_id, destination: RelayTransferDestination::Lobby,
        }); 1
    }));

    let play_queue = queue.0.clone();
    commands.register(literal("play").then(literal("parkour").executes(move |context: &CommandContext<ServerCommandSource>| {
        play_queue.lock().expect("TheCrown command queue poisoned").push(Invocation::Transfer {
            player_id: context.source.player_id, destination: RelayTransferDestination::Mode(GameMode::Parkour),
        }); 1
    })));

    let server_queue = queue.0.clone();
    let server_argument = ArgumentBuilder::new(Argument::<ServerCommandSource>::new("instance", Arc::new(greedy_string()), None).into())
        .executes(move |context: &CommandContext<ServerCommandSource>| {
            server_queue.lock().expect("TheCrown command queue poisoned").push(Invocation::Transfer {
                player_id: context.source.player_id,
                destination: RelayTransferDestination::Instance(get_string(context, "instance").unwrap_or_default()),
            }); 1
        });
    commands.register(literal("server").then(server_argument));

    let msg_queue = queue.0.clone();
    let msg_argument = ArgumentBuilder::new(Argument::<ServerCommandSource>::new("target_and_message", Arc::new(greedy_string()), None).into())
        .executes(move |context: &CommandContext<ServerCommandSource>| {
            msg_queue.lock().expect("TheCrown command queue poisoned").push(Invocation::Whisper {
                source: (*context.source).clone(), arguments: get_string(context, "target_and_message").unwrap_or_default(),
            }); 1
        });
    commands.register(literal("msg").then(msg_argument));

    let web_queue = queue.0.clone();
    commands.register(literal("weblogin").executes(move |context: &CommandContext<ServerCommandSource>| {
        web_queue.lock().expect("TheCrown command queue poisoned").push(Invocation::WebLogin { player_id: context.source.player_id }); 1
    }));
    app.insert_resource(queue);
}

pub fn apply_command_requests(
    queue: Res<CommandQueue>, sessions: Res<TheCrownGamePlayerSessions>, ids: Res<TheCrownRelayRequestIds>,
    mut transfers: MessageWriter<RequestRelayPlayerTransfer>, mut whispers: MessageWriter<RequestRelayWhisper>,
    mut web: MessageWriter<RequestTheCrownWebLogin>, mut chat: MessageWriter<PublishServerChatMessage>,
) {
    let invocations = std::mem::take(&mut *queue.0.lock().expect("TheCrown command queue poisoned"));
    for invocation in invocations {
        match invocation {
            Invocation::Transfer { player_id, destination } => {
                let Some(session) = sessions.get(player_id) else { continue; };
                transfers.write(RequestRelayPlayerTransfer { request_id: ids.next(), player_id, player_uuid: session.identity.uuid, destination });
            }
            Invocation::Whisper { source, arguments } => {
                let Some((target, message)) = arguments.trim().split_once(char::is_whitespace) else {
                    personal(&mut chat, source.player_id, "Usage: /msg <player> <message>"); continue;
                };
                if message.is_empty() { personal(&mut chat, source.player_id, "Message cannot be empty"); continue; }
                let Some(sender) = sessions.get(source.player_id) else { continue; };
                whispers.write(RequestRelayWhisper {
                    request_id: ids.next(), sender_id: source.player_id, sender_uuid: sender.identity.uuid,
                    target_username: target.to_owned(), message: message.trim().to_owned(),
                });
            }
            Invocation::WebLogin { player_id } => {
                let Some(session) = sessions.get(player_id) else { continue; };
                web.write(RequestTheCrownWebLogin { request_id: ids.next(), player_id, player_uuid: session.identity.uuid });
            }
        }
    }
}

pub fn apply_relay_results(
    config: Res<TheCrownGameConfig>, mut results: MessageReader<RelayPlayerTransferFinished>,
    mut whisper_results: MessageReader<RelayWhisperFinished>, mut web_results: MessageReader<TheCrownWebLoginFinished>,
    mut outgoing: MessageWriter<TransferTheCrownPlayer>, mut chat: MessageWriter<PublishServerChatMessage>,
    mut external_links: MessageWriter<PublishServerExternalLink>,
) {
    for result in results.read() {
        match &result.result {
            Ok(DataTransferResult::Join { transfer }) => { outgoing.write(TransferTheCrownPlayer { player_id: result.player_id, transfer: transfer.clone() }); }
            Ok(DataTransferResult::NotFound) => personal(&mut chat, result.player_id, "No matching TheCrown instance is available"),
            Err(error) => personal(&mut chat, result.player_id, &format!("Relay transfer failed: {error}")),
        }
    }
    for result in whisper_results.read() {
        if !result.delivered { personal(&mut chat, result.sender_id, result.error.as_deref().unwrap_or("Player is offline")); }
    }
    for result in web_results.read() {
        match &result.result {
            Ok(token) => {
                external_links.write(PublishServerExternalLink {
                    audience: Audience::personal(result.player_id),
                    title: "TheCrown web login".to_owned(),
                    description: "Open this one-time link to sign in to the TheCrown web dashboard with your current game account.".to_owned(),
                    url: format!("http://{}/auth/{token}", config.web_address),
                });
                personal(&mut chat, result.player_id, "Your web login link is ready");
            }
            Err(error) => personal(&mut chat, result.player_id, &format!("Web login failed: {error}")),
        }
    }
}

pub fn deliver_relay_whispers(
    sessions: Res<TheCrownGamePlayerSessions>, mut received: MessageReader<RelayWhisperReceived>,
    mut chat: MessageWriter<PublishServerChatMessage>,
) {
    for whisper in received.read() {
        let Some(player_id) = sessions.player_for_uuid(whisper.target_uuid) else { continue; };
        personal(&mut chat, player_id, &format!("[{} -> you] {}", whisper.sender.username, whisper.message));
    }
}

fn personal(chat: &mut MessageWriter<PublishServerChatMessage>, player_id: u64, text: &str) {
    chat.write(PublishServerChatMessage { audience: Audience::personal(player_id), text: text.to_owned() });
}
