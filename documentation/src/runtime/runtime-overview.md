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
- inventory and cell-menu state;
- dimensions and portal state;
- capability grants;
- player admission, chat routing, and command execution;
- validation and synchronization.

The shared protocol carries intentions from client to server and authoritative
results from server to client.

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
