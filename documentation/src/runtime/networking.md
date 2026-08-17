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
optional per-connection frame security
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
- chat and command completion;
- structured external-link prompts.

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

## External link prompts

A server does not need to encode a clickable URL inside chat. The generic
external-link domain is split into four layers:

```text
PublishServerExternalLink { audience, title, description, url }
  -> server-external-link-network-sync-mod
  -> ShowExternalLink packet
  -> client-external-link-network-receive-mod
  -> ShowClientExternalLink
  -> client-external-link-ui-bevy-mod
```

The client presenter pauses the in-game overlay, displays the title,
description and complete URL, and offers `Close` and `Open in browser`. Only
HTTP and HTTPS URLs are accepted by the browser launcher. The OS integration
uses `rundll32` on Windows, `open` on macOS, and `xdg-open` on Unix desktop
systems.

The server event accepts an `Audience`, so personal account links and shared
documentation links use the same mechanism without placing audience policy in
the packet or UI mod.

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

1. reads `ClientConnectionTarget`, normally edited by the main-menu server
   address field;
2. passes its trimmed `host:port` value to `TcpStream::connect`;
3. records the resolved peer address with `peer_addr`;
4. enables nonblocking mode and `TCP_NODELAY`;
5. clones the stream for the writer;
6. inserts `ClientNetworkSender`;
7. inserts connection buffers and outbox state.

The target is stored as a string rather than eagerly parsed as `SocketAddr`.
This deliberately lets the standard library resolve DNS names as well as
numeric IPv4/IPv6 endpoints. An empty target, malformed endpoint, failed DNS
lookup, or refused connection emits the normal disconnect-state transition and
returns from the connect system; it does not panic because parsing failed.

`ClientNetworkSender::send` serializes and queues a frame. It does not block
until the complete frame reaches the socket.

During `NetworkMessageSet::ReceivePackets`, the transport:

- flushes queued frames;
- reads available bytes;
- extracts complete frames;
- decodes client-bound messages;
- emits `ClientPacketReceived`.

The selected client transport also publishes connection and disconnection
messages. Optional authentication mods use these contracts without being
hardcoded into TCP.

Leaving `InGame` removes the sender and connection.

The alternate UDP client follows the same connection-target contract:
`UdpSocket::connect` receives the `host:port` string and `peer_addr` records the
resolved socket address used by runtime network events. A replacement transport
should preserve this behavior if it wants the main-menu address field to remain
transport independent.

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

`ServerTransportDisconnectRequested` supports disconnect-after-flush. This is
important for authentication and other pre-session failures: a typed reason can
be queued before the socket is closed.

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

## Optional secure frames

`network-frame-security-api` sits between CBOR and length framing. A connection
can be plaintext, temporarily paused during a handshake, AES-GCM protected, or
failed. Servers that do not select an authentication policy stay in plaintext
mode; TheCrown switches every accepted connection to a fresh secure channel.

For a secure connection, sequence assignment and insertion into the TCP outbox
share the same per-connection critical section. This is required because Bevy
systems may send in parallel: encrypting frames N and N+1 under one lock but
queueing them later under another could reverse their wire order and cause a
valid frame to fail AES-GCM authentication. The transport owns this invariant;
packet-producing gameplay mods do not coordinate with each other.

See [Patchwork account authentication](./patchwork-authentication.md) for the
handshake, key derivation, account binding, and composition boundaries.

## TheCrown connection transfer

TheCrown adds a protocol contributor, not a second transport. A source server
sends `TransferPlayer { TransferPacketData }`; the client changes
`ClientConnectionTarget`, tears down the old TCP session through game state,
and opens a normal TCP connection to the destination.

The destination still performs the Patchwork secure-frame handshake. A
namespaced client session gate delays `JoinRequest` until the client has sent
the Relay proof and received `TransferAuthenticated`. The game server redeems
the one-use cookie with Relay before its admission rule allows the session.

The client retains the entry endpoint while a transfer is active. A normal
return to Home restores that endpoint; otherwise the next Play attempt would
connect directly to the destination game worker without a fresh one-use Relay
ticket.

This keeps responsibilities independent:

- TCP owns framing, connection lifetime, and secure frame ordering;
- Patchwork authentication owns account identity and frame keys;
- TheCrown Relay owns destination and one-use instance admission;
- the generic session layer owns final `PlayerId` creation.

See [TheCrown network and dynamic instances](../development/thecrown-network.md)
for the full cross-process flow.

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
