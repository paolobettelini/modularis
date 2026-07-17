# Protocol, transport, and framing

The active transport is TCP, but packet types and gameplay are transport
independent.

## Protocol layers

```text
domain packet structs
        │
network contributor metadata
        │
generated packet enums + ECS dispatch
        │
CBOR serialization
        │
length-prefixed framing
        │
TCP client/server implementations
```

Each layer can evolve without moving all responsibilities into one crate.

## Distributed packet ownership

Feature families own their packets:

- sessions;
- players;
- gravity and jump;
- flight capability and separate flight speed;
- generic kick/disconnect reasons;
- chunks;
- block edits;
- inventory and hotbar;
- cell menus;
- dimensions;
- sky and sun;
- portals;
- chat and command completion.

`Kick { reason }` is intentionally not session-admission-specific. The server
can address it to a socket before a `PlayerId` exists or to an admitted player,
and the same client receiver returns to the main menu and displays the reason.

For example:

```toml
[package.metadata.network.messages]
clientbound = ["dimension_network_message_types::PlayerDimensionChanged"]
serverbound = []
```

The protocol generator collects all selected contributors.

## CBOR

The generated enums expose:

```rust
message.encode_cbor()
ClientBoundMessage::decode_cbor(bytes)
```

Client and server must use compatible compositions. Changing selected packet
contributors changes the enum representation and requires rebuilding both.

For a production protocol, explicit stable discriminants and version
negotiation would be needed. The demo currently relies on matched generated
builds.

## TCP framing

TCP is a byte stream, not a packet transport. `network-framing-api` writes:

```text
4-byte big-endian payload length
CBOR payload bytes
```

The maximum frame size is `1_048_576` bytes.

The framing helpers:

- encode one frame;
- queue frames;
- flush partial nonblocking writes;
- read all currently available bytes;
- drain complete frames while keeping an incomplete tail.

This handles:

- one frame split across reads;
- several frames in one read;
- partial writes;
- `WouldBlock`;
- interrupted system calls.

## Client TCP implementation

On entering `GameState::InGame`, the client:

1. reads the configured server address;
2. opens a TCP connection;
3. enables nonblocking mode and `TCP_NODELAY`;
4. clones the stream for the writer;
5. inserts `ClientNetworkSender`;
6. inserts connection buffers and outbox state.

`ClientNetworkSender::send` serializes and queues a frame. It does not block
until the complete frame reaches the socket.

During `NetworkMessageSet::ReceivePackets`, the transport:

- flushes queued frames;
- reads available bytes;
- extracts complete frames;
- decodes client-bound messages;
- emits `ClientPacketReceived`.

Leaving `InGame` removes the sender and connection.

## Server TCP implementation

The server binds the configured address and uses a nonblocking listener.

Per connected address it stores:

- reader and writer streams;
- an outbox;
- read buffer;
- partial write buffer and offset.

Every receive stage:

1. accepts all pending connections;
2. flushes each client's outbox;
3. reads available bytes;
4. extracts and decodes frames;
5. emits `ServerPacketReceived { source, message }`;
6. removes disconnected clients.

The socket source address is not trusted as a player ID. Session systems map it
through `ServerPlayerRegistry`.

## Transport replacement

The repository still contains UDP implementation crates, but the active
modpacks select TCP.

To add another transport:

1. implement `ClientNetworkApi` and/or `ServerNetworkApi`;
2. install the same sender resources;
3. emit generic received packet messages;
4. run receive systems in `NetworkMessageSet::ReceivePackets`;
5. preserve packet generation and typed dispatch;
6. select the new provider in the modpack.

Gameplay mods should require no changes.

## Backpressure and current limits

Outboxes are in-memory queues with no explicit byte or message limit. A slow
peer can therefore accumulate queued data.

A stronger implementation should add:

- per-client queue limits;
- disconnect or drop policy;
- metrics;
- bounded chunk response scheduling;
- protocol version negotiation;
- optional compression for large payloads.

These concerns belong in transport and scheduling mods, not in gameplay packet
handlers.
