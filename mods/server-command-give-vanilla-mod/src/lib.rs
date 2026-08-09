use audience_api::Audience;
use bevy::prelude::*;
use bevy_mod::BevyMod;
use block_manager_api::BlockManagerApi;
use inventory_core_api::Inventory;
use inventory_events_api::{
    InventorySetCellRequested, InventoryValidationSet,
};
use inventory_events_mod::InventoryEventsMod;
use item_manager_api::ItemManagerApi;
use server_chat_api::{PublishServerChatMessage, ServerChatApi, ServerChatSet};
use server_command_api::{
    CommandPlayer, ServerCommandApi, ServerCommandRegistry, ServerCommandSource,
    brigadier::{
        arguments::string_argument_type::{get_string, greedy_string},
        builder::{
            argument_builder::ArgumentBuilder, literal_argument_builder::literal,
            required_argument_builder::Argument,
        },
        context::CommandContext,
        suggestion::{SuggestionProvider, Suggestions, SuggestionsBuilder},
    },
    split_player_prefix,
};
use server_give_item_lib::{plan_give_item, vanilla_granted_item};
use server_inventory_api::{ServerInventories, ServerInventoryApi};
use server_player_registry_api::{ServerPlayerRegistry, ServerPlayerRegistryApi};
use std::{collections::HashMap, marker::PhantomData, sync::{Arc, Mutex}};
use tokio::task::JoinHandle;

const MAX_GIVE_AMOUNT: u32 = 1_000_000;

#[derive(Debug, Clone)]
struct GiveInvocation {
    source: player_network_message_types::PlayerId,
    arguments: String,
}

#[derive(Resource, Clone, Default)]
struct GiveCommandQueue(Arc<Mutex<Vec<GiveInvocation>>>);

pub struct ServerCommandGiveVanillaMod<I, B>(PhantomData<(I, B)>);

impl<I: ItemManagerApi, B: BlockManagerApi> ServerCommandGiveVanillaMod<I, B> {
    #[allow(clippy::too_many_arguments)]
    pub fn init<C: ServerCommandApi, H: ServerChatApi, P: ServerPlayerRegistryApi, V: ServerInventoryApi>(
        bevy: &mut BevyMod,
        _commands: &mut C,
        _chat: &mut H,
        _players: &mut P,
        _inventories: &mut V,
        _events: &mut InventoryEventsMod,
        _items: &mut I,
        _blocks: &mut B,
        _metadata: &mut item_metadata_registry_codegen::ItemMetadataRegistryCodegenMod,
        _quantity: &mut item_quantity_meta::ItemQuantityMetaMod,
        _place_block: &mut item_place_block_meta::ItemPlaceBlockMetaMod,
        _portal_igniter: &mut item_portal_igniter_meta::ItemPortalIgniterMetaMod,
    ) -> Self {
        let queue = GiveCommandQueue::default();
        register_command::<I>(bevy.app.world().resource::<ServerCommandRegistry>(), &queue);
        bevy.app.insert_resource(queue).add_systems(
            Update,
            apply_give_commands::<I, B>
                .in_set(ServerChatSet::ApplyGameplay)
                .before(InventoryValidationSet::Other),
        );
        Self(PhantomData)
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn register_command<I: ItemManagerApi>(commands: &ServerCommandRegistry, queue: &GiveCommandQueue) {
    let queue = queue.0.clone();
    let arguments = ArgumentBuilder::new(
        Argument::<ServerCommandSource>::new(
            "arguments",
            Arc::new(greedy_string()),
            Some(Arc::new(GiveSuggestions::<I>(PhantomData))),
        )
        .into(),
    )
    .executes(move |context: &CommandContext<ServerCommandSource>| {
        queue
            .lock()
            .expect("give command queue lock poisoned")
            .push(GiveInvocation {
                source: context.source.player_id,
                arguments: get_string(context, "arguments").unwrap_or_default(),
            });
        1
    });
    let command: ArgumentBuilder<ServerCommandSource> = literal("give").then(arguments);
    commands.register(command);
}

struct GiveSuggestions<I>(PhantomData<I>);

impl<I: ItemManagerApi> SuggestionProvider<ServerCommandSource> for GiveSuggestions<I> {
    fn get_suggestions(
        &self,
        context: CommandContext<ServerCommandSource>,
        builder: SuggestionsBuilder,
    ) -> Suggestions {
        let raw = builder.remaining();
        let leading = raw.len() - raw.trim_start().len();
        let remaining = raw.trim_start();

        if let Some((player, item_prefix)) =
            split_player_prefix(remaining, &context.source.online_players)
        {
            let after_player = &remaining[player.name.len()..];
            if after_player.chars().next().is_some_and(char::is_whitespace) {
                let whitespace = after_player.len() - after_player.trim_start().len();
                let start = builder.start() + leading + player.name.len() + whitespace;
                return suggest_items::<I>(builder.create_offset(start), item_prefix);
            }
        }

        if remaining.chars().any(char::is_whitespace) {
            return builder.build();
        }
        let lower = remaining.to_lowercase();
        let mut builder = builder.create_offset(builder.start() + leading);
        for item in I::all() {
            let id = I::id(*item);
            if id.to_lowercase().starts_with(&lower) {
                builder = builder.suggest(id);
            }
        }
        for player in &context.source.online_players {
            if player.name.to_lowercase().starts_with(&lower) {
                builder = builder.suggest(&player.name);
            }
        }
        builder.build()
    }
}

fn suggest_items<I: ItemManagerApi>(
    mut builder: SuggestionsBuilder,
    prefix: &str,
) -> Suggestions {
    if prefix.chars().any(char::is_whitespace) {
        return builder.build();
    }
    let lower = prefix.to_lowercase();
    for item in I::all() {
        let id = I::id(*item);
        if id.to_lowercase().starts_with(&lower) {
            builder = builder.suggest(id);
        }
    }
    builder.build()
}

fn apply_give_commands<I: ItemManagerApi, B: BlockManagerApi>(
    queue: Res<GiveCommandQueue>,
    players: Res<ServerPlayerRegistry>,
    inventories: Res<ServerInventories>,
    mut requests: MessageWriter<InventorySetCellRequested>,
    mut chat: MessageWriter<PublishServerChatMessage>,
) {
    let invocations = std::mem::take(
        &mut *queue.0.lock().expect("give command queue lock poisoned"),
    );
    let online = command_players(&players);
    let mut staged = HashMap::<player_network_message_types::PlayerId, Inventory>::new();
    for invocation in invocations {
        let parsed = parse_give::<I>(invocation.source, &invocation.arguments, &online);
        let Ok((target, item, amount)) = parsed else {
            feedback(invocation.source, parsed.unwrap_err(), &mut chat);
            continue;
        };
        let Some(server_inventory) = inventories.get(target) else {
            feedback(invocation.source, "Target inventory is unavailable".to_string(), &mut chat);
            continue;
        };
        let inventory = staged
            .entry(target)
            .or_insert_with(|| server_inventory.inventory.clone());
        let granted = vanilla_granted_item::<I, B>(item, amount);
        let Ok(plan) = plan_give_item(inventory, granted) else {
            feedback(invocation.source, "Target inventory is full".to_string(), &mut chat);
            continue;
        };
        inventory
            .set(plan.cell.clone(), Some(plan.item.clone()))
            .expect("give planner only returns valid cells");
        requests.write(InventorySetCellRequested {
            player_id: target,
            cell: plan.cell,
            item: Some(plan.item),
        });
        let target_name = players.player(target).map(|player| player.name.as_str()).unwrap_or("unknown player");
        feedback(
            invocation.source,
            format!("Gave {amount} {} to {target_name}", I::label(item)),
            &mut chat,
        );
    }
}

fn parse_give<I: ItemManagerApi>(
    source: player_network_message_types::PlayerId,
    arguments: &str,
    online: &[CommandPlayer],
) -> Result<(player_network_message_types::PlayerId, item_manager_api::ItemId, u32), String> {
    let arguments = arguments.trim();
    let first = arguments.split_whitespace().next().unwrap_or_default();
    let (target, item_arguments) = if I::from_string(first).is_some() {
        (source, arguments)
    } else if let Some((player, remainder)) = split_player_prefix(arguments, online) {
        (player.id, remainder)
    } else {
        return Err("Usage: /give [player] <item id> [amount]".to_string());
    };
    let mut parts = item_arguments.split_whitespace();
    let item_id = parts.next().unwrap_or_default();
    let Some(item) = I::from_string(item_id) else {
        return Err(format!("Unknown item '{item_id}'"));
    };
    let amount = match parts.next() {
        Some(value) => value
            .parse::<u32>()
            .map_err(|_| "Amount must be a positive integer".to_string())?,
        None => 1,
    };
    if amount == 0 || amount > MAX_GIVE_AMOUNT || parts.next().is_some() {
        return Err(format!("Amount must be between 1 and {MAX_GIVE_AMOUNT}"));
    }
    Ok((target, item, amount))
}

fn command_players(players: &ServerPlayerRegistry) -> Vec<CommandPlayer> {
    players
        .players()
        .into_iter()
        .map(|player| CommandPlayer { id: player.id, name: player.name })
        .collect()
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
