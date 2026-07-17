# Chat, commands, and completion

Chat is a set of small transport, state, policy, command, and presentation
mods. The client never decides who receives a message, and the transport layer
does not know whether text is public chat or a command.

```text
client input/UI
      │ ChatSubmit
      ▼
server network receive
      │ ServerChatInputReceived
      ▼
normal-chat policy OR command router
      │                     │
PublishServerChatMessage    ServerCommandRequested
      │                     │
audience resolver           Brigadier dispatcher -> gameplay events
      │                     │
      └────────── publish/sync ──────────┘
                         │ ChatMessage
                         ▼
                    client chat log
```

This split allows a server to replace global chat with local, team, dimension,
party, moderated, or no chat while keeping the same client UI and packets.

## Protocol contributions

`chat-network-message-types` owns only serializable values:

- `ChatSubmit { text }` from client to server;
- `ChatMessage { text }` from server to client;
- `CommandSuggestionsRequest { request_id, input, cursor }`;
- `CommandSuggestionsResponse { request_id, suggestions }`.

`chat-network-messages-mod` contributes those four types to network codegen.
They are not listed in a central protocol Cargo table. Removing the contributor
from a composition removes its generated packet variants.

The request ID prevents an older autocomplete response from replacing results
for newer input.

## Client state and input

`client-chat-api` defines the bounded log, current composer, ECS messages, and
ordered system sets. `client-chat-state-mod` owns their neutral state.

The chat key is a generated setting contributed by
`client-setting-chat-key`. Its default is `T`, and the existing keybinding
settings editor can change it without special chat-menu code.

`client-chat-toggle-input-mod` only opens the chat overlay while the in-game
overlay is `Playing`. Chat is a distinct `InGameOverlayState`, so movement and
other gameplay input can remain disabled while typing. Escape closes it, Enter
submits non-empty text, Backspace edits, and Tab accepts the first completion.

`client-chat-ui-bevy-mod` is presentation policy. It uses the shared UI font,
renders a bounded history at the bottom left, and renders the composer only
while chat is open. Replacing it with another renderer does not require changes
to chat state or networking.

The client sends intentions only. It never adds its submitted message directly
to the authoritative log; the message appears when the server sends a
`ChatMessage` back to an allowed recipient.

## Server ECS pipeline

`server-chat-events-mod` registers the contracts from `server-chat-api` and
orders this pipeline:

```text
Receive -> Route -> ExecuteCommands -> ApplyGameplay -> Publish -> Sync
```

The main messages are:

- `ServerChatInputReceived` for transport-independent player text;
- `ServerCommandRequested` for slash command input;
- `PublishServerChatMessage` for text plus a domain `Audience`;
- suggestion request/ready messages for asynchronous-looking request flow.

The network receiver resolves socket address to `PlayerId`, enforces the input
byte limit, and emits an ECS message. It does not format, route, execute, or
broadcast text.

## Chat delivery policy

The selected vanilla policy has two independent mods:

- `server-chat-global-vanilla-mod` formats normal input as
  `[player] message` and publishes it to `Audience::Everyone`;
- `server-chat-command-router-mod` recognizes a leading slash and emits a
  command request without the slash.

`server-chat-network-sync-mod` asks `ServerAudienceResolver` for concrete
player IDs and converts published messages into packets. The basic audience
provider sends `Everyone` and shared audiences to all online players.

A proximity chat replacement can omit the global mod and publish to a custom
shared audience. A no-chat server can omit both routing policies while keeping
commands through a different input surface.

## Command registry

`server-command-api` wraps an `azalea_brigadier::CommandDispatcher` in a Bevy
resource. Feature mods register command builders during initialization. The
provider executes parsed requests and builds completion results from the same
tree, so parsing and suggestions cannot drift into two separate command lists.

The command source currently exposes:

```rust
pub struct ServerCommandSource {
    pub player_id: PlayerId,
    pub player_name: String,
    pub online_players: Vec<CommandPlayer>,
}
```

A feature should register syntax and enqueue a domain intention. The command
closure should not directly borrow arbitrary Bevy resources, mutate gameplay,
or send network packets.

Example shape:

```rust
let queue = queue.clone();
let command = literal("heal")
    .executes(move |context| {
        queue.push(HealRequested {
            player_id: context.source.player_id,
        });
        1
    });

registry.register(command);
```

A system drains that queue in `ServerChatSet::ApplyGameplay`, validates the
request against current ECS state, emits domain events, and publishes feedback.
Permissions should be another rule/mod in that stage rather than hardcoded in
the generic dispatcher.

## Flight command example

`server-command-flight-vanilla-mod` contributes:

```text
/flight
/flight <player>
```

The player argument has a Brigadier suggestion provider backed by the online
player snapshot. The command queues a flight intention; a Bevy system resolves
the target, toggles `ServerPlayerFlightCapabilities`, emits the existing flight
capability event, and sends personal feedback through chat.

The command therefore reuses the flight API and synchronization pipeline. It
does not send the capability packet itself and it can be omitted from a server
that grants flight through roles, regions, levels, or another UI.

## Player admission and command names

Player names used by commands are unique because admission happens before
session creation. `server-player-name-unique-vanilla-mod` is one policy rule,
not part of the player registry. It rejects case-insensitive duplicates and
the client displays the server-provided reason.

The random `Player0` through `Player100` client default reduces accidental
collisions during local testing, but it is not a security or uniqueness
mechanism. Only server admission is authoritative.

## Adding a command

1. Create a feature mod that depends on `server-command-api` and the narrow
   gameplay APIs/events it needs.
2. Register one Brigadier command tree in `init`.
3. Store command invocations in an event or small queue resource.
4. Validate and apply them in a public system set.
5. Reuse domain events for effects and `PublishServerChatMessage` for text.
6. Add the feature mod to a policy modpack, not `server-base.toml`, unless the
   command is truly required infrastructure.

Keep autocomplete close to the argument that owns it. Player-name completion,
item-ID completion, and world completion should be separate providers using
the appropriate registry snapshots.

## Current limits

- there is no permission or authentication model;
- chat text is plain text with no structured style spans;
- there is no history navigation, selection, or cursor movement in the client
  composer;
- suggestions are full replacement strings and the UI displays only the first
  few;
- normal chat has no rate limiting or moderation;
- the selected global policy ignores dimension and distance;
- command registration happens at startup, matching compile-time composition.

These belong in optional policy or presentation mods. They should not be added
as special cases to transport, generated messages, or the command dispatcher.
