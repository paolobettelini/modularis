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

Admission policy is separate from registry storage. Before creating a new
player, `server-player-session-mod` builds a `ServerJoinCandidate` and asks all
rules in `ServerPlayerAdmissionRules` to validate it. A rule can reject a join
without modifying the session implementation.

The selected `server-player-name-unique-vanilla-mod` rejects duplicate names
case-insensitively. A rejected admission emits
`ServerKickRequested::Address` because no `PlayerId` exists yet. The generic
kick pipeline sends `Kick { reason }`; the client enters a dedicated
disconnected screen, displays the reason in the center, and
drops its TCP connection as it exits the in-game state. The user explicitly
returns home with the `Back to home` button. The same packet and client behavior
are used for an admitted player kicked later by another server rule.

The client setting still owns the chosen name. The optional
`client-player-name-random-default-mod` changes only the untouched default
`Player` to `Player0` through `Player100` at startup. It does not overwrite a
name saved by the user, and uniqueness is still authoritative on the server.

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
  -> sanitize candidate
  -> composable admission rules
  -> reject through generic kick, or register address/player
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

`server-player-kick-mod` performs the same authoritative cleanup for a kick. It
accepts either a pre-admission socket address or an admitted `PlayerId`, sends a
bounded reason, removes session/network membership when present, emits the
lifecycle event, and sends `PlayerLeft` only to viewers that could see the
player. Policies such as moderation, duplicate-name admission, bans, timeouts,
or commands should emit `ServerKickRequested` instead of duplicating this
sequence.

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
leave, gravity, or model-scale updates.

Gravity and scale synchronization are subject-oriented. On join, the new
client receives attributes for itself and every subject it may currently see;
existing viewers receive the joining subject's attributes. Later changes go to
the subject plus `viewers_of(subject)`. This avoids leaking per-player state
across dimensions and prevents one player's gravity from rotating every avatar
on another client.

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
menus and chat. It currently has personal, shared-ID, and everyone variants.
`server-audience-api` turns that domain-level value into player IDs. The basic
provider maps `Everyone` and `Shared` to all online players and `Personal` to
one online player. Servers can replace it with team, permission, distance, or
world-scope policy without changing chat packet code.

They can all resolve to player lists, but keeping them separate allows each
domain to choose its own policy.
