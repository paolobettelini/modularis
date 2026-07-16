# Sessions, routing, and visibility

Networking addresses, player identities, packet audiences, and world visibility
are separate concepts.

## Session registry

`ServerPlayerRegistry` stores:

- next player ID;
- address-to-player mapping;
- `NetworkPlayer` records;
- last-seen timestamps.

A joined player has:

```rust
NetworkPlayer {
    id,
    name,
    position,
    yaw,
    pitch,
}
```

The server sanitizes names by trimming, falling back to `Player`, and limiting
the name to 32 characters.

## Lifecycle messages

`server-player-lifecycle-events-mod` exposes:

- `ServerPlayerJoined`;
- `ServerPlayerLeft`.

Independent features listen to these messages to:

- create inventories;
- grant capabilities;
- send gravity and sun state;
- assign a dimension;
- clean up per-player state.

Session code should not call all these features directly.

## Join flow

```text
JoinRequestReceived
  -> address authenticated/registered
  -> player created or reused
  -> visible player snapshot selected
  -> JoinAccepted sent to source address
  -> ServerPlayerJoined emitted
  -> PlayerJoined sent only to allowed viewers
```

The local client does not render its own avatar because the accepted packet
identifies the local `player_id`.

## Leave and timeout

Explicit leave removes the registry entry, network client, and emits
`ServerPlayerLeft`.

`server-player-timeout-mod` expires inactive registry entries. Cleanup features
must listen to the lifecycle event rather than relying only on a leave packet,
because network disconnects are not always graceful.

## Server packet audiences

`ServerAudience` supports:

- one socket address;
- one player;
- broadcast;
- broadcast except an address;
- broadcast except a player;
- an explicit player list.

Gameplay writes `ServerPacketOut`. `server-network-router-mod` resolves player
IDs to addresses and invokes the selected transport.

Use:

- address audiences during authentication, before a stable player ID is
  available;
- player audiences after session mapping;
- explicit lists for visibility or domain audiences;
- broad broadcast only when the state is truly global.

## Player visibility

`ServerPlayerVisibility` is a replaceable closure-backed resource:

```rust
can_see(viewer, subject) -> bool
```

It also computes all viewers of one subject.

The active `server-player-visibility-world-instance-mod` compares the world
scope resolved for viewer and subject. Players in different dimensions or
provider-backed instances do not receive each other's join, movement, rotation,
or leave updates.

## Movement synchronization

Movement packets enter:

```text
ServerPlayerMovementSet::Receive
  -> Validate
  -> Apply
  -> Sync
```

The session mod collects requests, validators modify `accepted_position` or
mark them rejected, and the apply stage updates the registry.

Accepted movement is sent to visible remote players. The local player receives
a `PlayerMoved` correction only when:

- the move was rejected; or
- accepted and requested positions differ by more than the correction
  threshold.

This prevents the server from fighting local prediction every frame.

## Extending visibility

Replace the provider with a policy based on:

- distance;
- teams;
- permissions;
- stealth;
- parties;
- line of sight;
- custom instance membership.

The provider must remain deterministic enough for join snapshots and later
updates to agree.

If visibility changes without movement or dimension change, add a feature that
emits the required `PlayerJoined` and `PlayerLeft` deltas. The current API
answers visibility queries but does not maintain a general dynamic diff
engine.

## Do not confuse audience and visibility

`ServerAudience` is a concrete outbound packet target.

`ServerPlayerVisibility` is policy for one domain: player entities.

`Audience` in `audience-api` describes ownership/sharing of state such as cell
menus.

They can all resolve to player lists, but keeping them separate allows each
domain to choose its own policy.
