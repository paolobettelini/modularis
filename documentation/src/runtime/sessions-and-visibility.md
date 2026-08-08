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

`JoinRequest` carries no player name. When a request arrives,
`server-player-session-mod` creates a temporary server-side fallback name such
as `Player1`, `Player2`, and so on. The fallback exists only so compositions
without an identity provider still have a usable display name; the client never
chooses or transmits it.

Admission policy is separate from registry storage. Before creating a new
player, `server-player-session-mod` builds a mutable `ServerJoinCandidate`.
`ServerPlayerAdmissionRules` first runs the `prepare` phase of every rule, then
runs the `validate` phase of every rule. Identity providers can therefore
replace the fallback before uniqueness, bans, or other validators inspect the
candidate. The final prepared name is what the registry stores in
`NetworkPlayer`.

TheCrown adds the Patchwork authenticated-account admission rule. Its prepare
phase maps the socket to a backend-redeemed account and replaces the candidate
name with the backend nickname. After the session is admitted, the created
`PlayerId` is bound to the complete authenticated account, including the stable
account UUID. Other servers can omit this rule or install a different identity
policy over the same admission seam. See
[Patchwork account authentication](./patchwork-authentication.md).

The selected `server-player-name-unique-vanilla-mod` rejects duplicate names
case-insensitively. A rejected admission emits
`ServerKickRequested::Address` because no `PlayerId` exists yet. The generic
kick pipeline sends `Kick { reason }`; the client enters a dedicated
disconnected screen, displays the reason in the center, and
drops its TCP connection as it exits the in-game state. The user explicitly
returns home with the `Back to home` button. The same packet and client behavior
are used for an admitted player kicked later by another server rule.

On a server without an identity provider, the session fallback remains the
player's display name. On an authenticated Patchwork server, the backend
nickname replaces that fallback before validation. In both cases uniqueness is
still authoritative on the server, and no client-supplied nickname participates
in admission.

## Lifecycle messages

`server-player-lifecycle-events-mod` exposes:

- `ServerPlayerJoined`;
- `ServerPlayerReady`;
- `ServerPlayerLeft`.

Independent features listen to these messages to:

- create inventories;
- grant capabilities;
- send gravity and sun state;
- assign a dimension;
- clean up per-player state.

Session code should not call all these features directly.

`ServerPlayerJoined` means the registry entry exists. Systems in
`ServerPlayerSessionSet::Initialize` may now assign scopes, worlds, inventory,
and capabilities before the initial snapshot is built.

`ServerPlayerReady` means `JoinAccepted` has been queued. It is suitable for
welcome messages and state that should follow the accepted packet.

## Join flow

```text
JoinRequestReceived
  -> create server-side fallback candidate
  -> prepare all admission rules (identity may replace the name)
  -> validate all admission rules
  -> reject through generic kick, or register address/player
  -> ServerPlayerJoined
  -> Initialize: scope/world/game state assignment
  -> visible player snapshot selected
  -> JoinAccepted sent to source address
  -> PlayerJoined sent only to allowed viewers
  -> ServerPlayerReady
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

The vanilla server selects
`server-player-visibility-world-instance-mod`, which compares the chunk world
scope resolved for viewer and subject. Players in different dimensions or
provider-backed instances do not receive each other's join, movement, rotation,
leave, gravity, or model-scale updates.

TheCrown selects `server-player-visibility-scope-impl`. It compares the nearest
`visibility` facet in each player's scope ancestry. This lets entity visibility
vary independently from chat and chunks.

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

For scope-based visibility,
`server-player-visibility-scope-sync-mod` reacts to
`ServerPlayerScopeChanged` and emits the required `PlayerJoined` and
`PlayerLeft` deltas for live migrations. The session pipeline still owns the
initial join and final leave.

Another visibility provider must provide equivalent transition
synchronization when its policy changes at runtime.

## Do not confuse audience and visibility

`ServerAudience` is a concrete outbound packet target.

`ServerPlayerVisibility` is policy for one domain: player entities.

`Audience` in `audience-api` describes ownership/sharing of state such as cell
menus and chat. It currently has personal, shared-ID, and everyone variants.
`server-audience-api` turns that domain-level value into player IDs.

The vanilla basic provider maps `Everyone` and `Shared` to all online players
and `Personal` to one online player. The scope provider treats a shared ID as a
scope node and resolves online members in that subtree. Servers can replace
either with team, permission, distance, subscription, or application-specific
policy without changing chat packet code.

They can all resolve to player lists, but keeping them separate allows each
domain to choose its own policy.

See [Runtime scope trees and facets](../architecture/runtime-scopes.md) for
hierarchical membership and migration.
