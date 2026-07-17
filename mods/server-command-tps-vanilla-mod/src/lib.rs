use audience_api::Audience;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_chat_api::{PublishServerChatMessage, ServerChatApi, ServerChatSet};
use server_command_api::{
    ServerCommandApi, ServerCommandRegistry, ServerCommandSource,
    brigadier::{
        builder::{argument_builder::ArgumentBuilder, literal_argument_builder::literal},
        context::CommandContext,
    },
};
use server_tick_api::{ServerTickApi, ServerTickMetrics, ServerTickRate};
use server_tick_metrics_mod::ServerTickMetricsMod;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

#[derive(Resource, Clone, Default)]
struct TpsCommandQueue(Arc<Mutex<Vec<player_network_message_types::PlayerId>>>);

pub struct ServerCommandTpsVanillaMod;

impl ServerCommandTpsVanillaMod {
    pub fn init<C: ServerCommandApi, H: ServerChatApi, T: ServerTickApi>(
        bevy: &mut BevyMod,
        _commands: &mut C,
        _chat: &mut H,
        _tick: &mut T,
        _metrics: &mut ServerTickMetricsMod,
    ) -> Self {
        let queue = TpsCommandQueue::default();
        register_command(bevy.app.world().resource::<ServerCommandRegistry>(), &queue);
        bevy.app.insert_resource(queue).add_systems(
            Update,
            apply_tps_commands.in_set(ServerChatSet::ApplyGameplay),
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_command(commands: &ServerCommandRegistry, queue: &TpsCommandQueue) {
    let queue = queue.0.clone();
    let command: ArgumentBuilder<ServerCommandSource> =
        literal("tps").executes(move |context: &CommandContext<ServerCommandSource>| {
            queue
                .lock()
                .expect("tps command queue lock poisoned")
                .push(context.source.player_id);
            1
        });
    commands.register(command);
}

fn apply_tps_commands(
    queue: Res<TpsCommandQueue>,
    rate: Res<ServerTickRate>,
    metrics: Res<ServerTickMetrics>,
    mut chat: MessageWriter<PublishServerChatMessage>,
) {
    let players = std::mem::take(&mut *queue.0.lock().expect("tps command queue lock poisoned"));
    for player_id in players {
        chat.write(PublishServerChatMessage {
            audience: Audience::personal(player_id),
            text: format!(
                "TPS: {:.1} / target {:.1}",
                metrics.measured_tps, rate.target_tps
            ),
        });
    }
}
