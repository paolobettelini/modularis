# Runtime and networking

The client and server both run Bevy, but they install different plugin sets and
have different authority.

## Runtime responsibility

The client owns:

- window and renderer;
- local input;
- local movement prediction;
- client caches;
- UI;
- chunk mesh and visual entities;
- remote player visuals;
- network intentions.

The server owns:

- session identities;
- authoritative positions;
- authoritative chunks and edits;
- runtime scope membership and instance lifecycle;
- inventory and cell-menu state;
- dimensions and portal state;
- capability grants;
- player admission, chat routing, and command execution;
- validation and synchronization.

The shared protocol carries intentions from client to server and authoritative
results from server to client.

Compile-time composition and runtime scope are separate. A modpack determines
which systems and mechanics exist. The scope tree lets those selected systems
apply to one lobby, match, team, private world, or subtree. See
[Runtime scope trees and facets](../architecture/runtime-scopes.md).

## Server tick rate

The headless runner depends on the small `ServerTickApi` provider instead of
hardcoding a sleep duration. The selected
`server-tick-rate-20hz-default-impl` supplies `20` ticks per second, configures
the schedule runner, and keeps Bevy's fixed schedule at the same frequency.

`server-tick-metrics-mod` measures the observed update rate independently. The
optional `/tps` command reads `ServerTickRate` and `ServerTickMetrics`, then
publishes the result only to its caller. A different server can replace the
tick provider without modifying the runner, metrics, or command.

## High-level data flow

```text
client input
    │
    ├── local prediction / optimistic UI
    │
    └── server-bound packet
               │
          typed ECS request
               │
       server validation/apply
               │
          domain result event
               │
      audience-aware packet
               │
          client cache/UI
```

Local prediction does not remove server authority. It hides normal network
latency and is corrected by authoritative state.

## Bevy message dispatch

Both transports produce generic messages:

- `ClientPacketReceived(ClientBoundMessage)`;
- `ServerPacketReceived { source, message }`.

Generated systems run in:

```text
NetworkMessageSet::ReceivePackets
    -> NetworkMessageSet::DispatchPackets
```

After dispatch, feature mods read typed messages such as:

- `ChunkResponseReceived`;
- `PlayerMoveReceived`;
- `InventoryResetPacketReceived`;
- `SunSettingsChangedReceived`.
- `ChatSubmitReceived`;
- `CommandSuggestionsResponseReceived`.
- `KickReceived`;
- `PlayerFlightSpeedChangedReceived`.
- `PlayerWorldChangedReceived`.

Network feature systems should normally run after
`NetworkMessageSet::DispatchPackets`.

## Server outbound routing

Server gameplay does not send directly through a socket. It writes:

```rust
ServerPacketOut {
    audience: ServerAudience,
    message: ClientBoundMessage,
}
```

`server-network-router-mod` resolves addresses and uses the selected
`ServerNetworkSender`.

This keeps transport, player registry, audience choice, and gameplay meaning in
separate mods.

The next chapters cover app lifecycle, transport, sessions, chat, and commands
in detail.
