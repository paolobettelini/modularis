use audience_api::Audience;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use generated_permission_registry::PermissionId;
use server_chat_api::{PublishServerChatMessage, ServerChatApi, ServerChatSet};
use server_command_api::{
    OnlinePlayerSuggestions, ServerCommandApi, ServerCommandRegistry, ServerCommandSource,
    brigadier::{
        arguments::string_argument_type::{get_string, greedy_string},
        builder::{
            argument_builder::ArgumentBuilder, literal_argument_builder::literal,
            required_argument_builder::Argument,
        },
        context::CommandContext,
    },
};
use server_player_permission_api::{
    ClearPlayerPermissionGrants, ServerPlayerPermissionApi, ServerPlayerPermissionSet,
    ServerPlayerPermissions, SetPlayerPermission,
};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

const PRIVILEGE_COMMAND_OWNER: &str = "vanilla:privilege-command";

#[derive(Debug, Clone)]
struct PrivilegeInvocation {
    source: player_network_message_types::PlayerId,
    target_name: String,
}

#[derive(Resource, Clone, Default)]
struct PrivilegeCommandQueue(Arc<Mutex<Vec<PrivilegeInvocation>>>);

pub struct ServerCommandPrivilegeVanillaMod;

impl ServerCommandPrivilegeVanillaMod {
    pub fn init<
        C: ServerCommandApi,
        H: ServerChatApi,
        P: ServerPlayerRegistryApi,
        R: ServerPlayerPermissionApi,
    >(
        bevy: &mut BevyMod,
        _commands: &mut C,
        _chat: &mut H,
        _players: &mut P,
        _permissions: &mut R,
    ) -> Self {
        let queue = PrivilegeCommandQueue::default();
        register_command(bevy.app.world().resource::<ServerCommandRegistry>(), &queue);
        bevy.app.insert_resource(queue).add_systems(
            Update,
            apply_privilege_commands
                .in_set(ServerChatSet::ApplyGameplay)
                .before(ServerPlayerPermissionSet::Apply),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_command(commands: &ServerCommandRegistry, queue: &PrivilegeCommandQueue) {
    let queue = queue.0.clone();
    let player = ArgumentBuilder::new(
        Argument::<ServerCommandSource>::new(
            "player",
            Arc::new(greedy_string()),
            Some(Arc::new(OnlinePlayerSuggestions)),
        )
        .into(),
    )
    .executes(move |context: &CommandContext<ServerCommandSource>| {
        queue
            .lock()
            .expect("privilege command queue lock poisoned")
            .push(PrivilegeInvocation {
                source: context.source.player_id,
                target_name: get_string(context, "player").unwrap_or_default(),
            });
        1
    });
    commands.register_restricted(
        "privilege",
        PermissionId::Privileged,
        literal("privilege").then(player),
    );
}

fn apply_privilege_commands(
    queue: Res<PrivilegeCommandQueue>,
    players: Res<ServerPlayerRegistry>,
    permissions: Res<ServerPlayerPermissions>,
    mut clear: MessageWriter<ClearPlayerPermissionGrants>,
    mut set: MessageWriter<SetPlayerPermission>,
    mut chat: MessageWriter<PublishServerChatMessage>,
) {
    let invocations = std::mem::take(
        &mut *queue
            .0
            .lock()
            .expect("privilege command queue lock poisoned"),
    );
    for invocation in invocations {
        if !permissions.has(invocation.source, PermissionId::Privileged) {
            feedback(
                invocation.source,
                "This command is not available".to_string(),
                &mut chat,
            );
            continue;
        }
        let Some(target) = players
            .players()
            .into_iter()
            .find(|player| player.name.eq_ignore_ascii_case(invocation.target_name.trim()))
        else {
            feedback(invocation.source, "Player not found".to_string(), &mut chat);
            continue;
        };
        let enabled = !permissions.has(target.id, PermissionId::Privileged);
        if enabled {
            set.write(SetPlayerPermission {
                player_id: target.id,
                owner: PRIVILEGE_COMMAND_OWNER.to_string(),
                permission: PermissionId::Privileged,
                enabled: true,
            });
        } else {
            clear.write(ClearPlayerPermissionGrants {
                player_id: target.id,
                permission: PermissionId::Privileged,
            });
        }
        feedback(
            invocation.source,
            format!(
                "Privileged {} for {}",
                if enabled { "enabled" } else { "disabled" },
                target.name
            ),
            &mut chat,
        );
    }
}

fn feedback(
    player_id: player_network_message_types::PlayerId,
    text: String,
    chat: &mut MessageWriter<PublishServerChatMessage>,
) {
    chat.write(PublishServerChatMessage {
        audience: Audience::personal(player_id),
        text,
    });
}
