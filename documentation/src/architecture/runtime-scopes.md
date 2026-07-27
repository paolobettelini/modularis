# Runtime scope trees and facets

Compile-time Patchwork composition decides which capabilities exist in an
executable. Runtime scopes decide where selected capabilities apply.

This distinction is important:

- a modpack can include chat support once;
- the running server can create several chat groups;
- a modpack can include one chunk world service;
- the running server can route different players to different world instances;
- a modpack can include player synchronization;
- the running server can decide that some players must not see each other.

The scope system is intended for lobbies, minigames, private maps, parties,
teams, temporary arenas, and custom server topologies. It does not prescribe
one of those designs.

## Nodes are runtime ECS entities

`server-scope-api` defines `ScopeNodeId`, `ServerScopeNode`, and
`ServerScopes`. Every node has:

- a stable string ID;
- an optional parent;
- zero or more children;
- an ECS entity;
- zero or more facet boundaries.

The ECS entity is deliberate. A feature can attach its own components without
changing the scope core:

```rust
#[derive(Component)]
struct MatchRound {
    phase: RoundPhase,
    remaining_seconds: f32,
}

let entity = scopes.spawn(
    &mut commands,
    ScopeNodeDescriptor::child("match-42", lobby_scope),
)?;
commands.entity(entity).insert(MatchRound::new());
```

The scope API knows nothing about rounds, teams, worlds, chat, scores, or game
rules. It only owns hierarchy and membership.

Ordinary gameplay entities can carry:

```rust
ServerScopeMembership {
    scope: arena_scope,
}
```

This is the common hook for future NPC, drop, projectile, machine, or entity
replication systems. Player sessions currently use the indexed player
membership in `ServerScopes` because `NetworkPlayer` records are not ECS
entities.

## One tree, independent facets

A node does not automatically isolate every subsystem. Instead, a feature
resolves a named **facet** from a node towards the root. The nearest ancestor
that owns that facet is the effective boundary.

The built-in facets are:

- `patchwork:chat`;
- `patchwork:visibility`;
- `patchwork:world`.

Mods may define more facet IDs for entities, permissions, scoreboard state,
weather, commands, damage, or any other scoped domain.

Consider:

```text
root
└── network
    └── parkour-1                    [chat]
        ├── player-7                 [world, visibility]
        └── player-12                [world, visibility]
```

Both players resolve chat to `parkour-1`, so they share chat. Each resolves
world and visibility to its own leaf, so they receive different chunks and do
not see each other's avatar.

Moving the world facet to `parkour-1` would make the map shared without changing
chat or visibility. Moving the visibility facet to `parkour-1` would expose
players to each other without changing the world. This is why the architecture
does not use one universal `InstanceId` to answer every policy question.

## Creating and assigning scopes

`server-scope-tree-mod` provides the scope API and creates
`patchwork:root`. An orchestrator may create children directly:

```rust
let match_scope = ScopeNodeId::new("parkour-1");
scopes.spawn(
    &mut commands,
    ScopeNodeDescriptor::child(match_scope.0.clone(), scopes.root())
        .with_facet(ScopeFacetId::chat()),
)?;

let player_scope = ScopeNodeId::new("parkour-1:player-7");
scopes.spawn(
    &mut commands,
    ScopeNodeDescriptor::child(player_scope.0.clone(), match_scope)
        .with_facet(ScopeFacetId::visibility()),
)?;
```

A player currently has one primary scope membership. It may be assigned
directly when immediate orchestration is required:

```rust
let previous = scopes.assign_player(player_id, player_scope.clone())?;
```

or through the ECS contract:

```rust
scope_requests.write(SetServerPlayerScope {
    player_id,
    target: player_scope,
});
```

Membership changes produce `ServerPlayerScopeChanged`. Features that maintain
client-visible state can react to this result instead of observing the request.

The one-primary-membership rule does not prevent other grouping systems.
Parties, permissions, subscriptions, and teams may be separate components or
resources. The primary scope describes the player's place in this hierarchy;
it is not intended to replace every many-to-many relationship.

## Scoped messages

Bevy messages remain the runtime integration bus. `ScopedMessage<T>` adds:

- an origin scope;
- a domain payload;
- a propagation mode.

Available propagation modes are:

- `Exact`: only the origin;
- `Descendants`: origin and its subtree;
- `Ancestors`: origin and its ancestry;
- `Lineage`: both directions along the same branch.

Example:

```rust
#[derive(Debug, Clone)]
struct RoundEnded {
    winner: PlayerId,
}

app.add_message::<ScopedMessage<RoundEnded>>();

events.write(ScopedMessage {
    origin: match_scope,
    propagation: ScopePropagation::Descendants,
    payload: RoundEnded { winner },
});
```

A listener calls `event.reaches(&scopes, &listener_scope)` before handling the
payload. The envelope does not clone events into per-node queues and does not
hide routing work in a global dispatcher. This keeps normal Bevy scheduling,
message lifetime, and ordering visible.

Use a scoped message when propagation through the hierarchy is part of the
domain rule. Use an ordinary message carrying a `ScopeNodeId` when consumers
need exact routing logic of their own.

## Scoped world routing

`server-scope-world-api` associates a `ServerChunkRoute` with a scope node. A
route contains:

- `WorldInstanceId`;
- `ChunkProviderId`.

Binding a route adds the world facet to that node. Children inherit the nearest
bound route:

```rust
scope_worlds.bind(
    &scopes,
    arena_scope.clone(),
    ServerChunkRoute {
        instance: WorldInstanceId::new("parkour:7"),
        provider: ChunkProviderId::primary(),
    },
)?;
```

`server-chunk-routing-scopes-mod` turns this lookup into the existing
`ServerChunkRouter` API. Chunk generation, storage, residency, requests, and
world mutation therefore remain unchanged. They see an ordinary route after
the scope provider has selected it.

This supports several useful layouts:

- one parent route inherited by all children for a shared map;
- one route per player for private maps;
- one provider shared by many instances;
- different providers below the same parent;
- a shared immutable source with independent chat or entity facets.

## World context is not dimension

`Dimension` describes a generated domain such as Overworld or Nether.
`WorldInstanceId` identifies a concrete runtime world. Two parkour arenas may
both use Overworld rules while containing different blocks.

For this reason, `PlayerWorldChanged` is separate from the dimension packet.
The server sends:

- world instance ID;
- authoritative transition position.

The client:

1. stores a revisioned `ClientWorldContext`;
2. clears chunk cache and active render state;
3. waits until the local player entity exists;
4. applies the authoritative position and resets velocity.

The latest position is persistent state, not only a transient event. This
prevents an initial world assignment from being lost when it arrives before
the local player entity is spawned.

## Audience and player visibility

Two scope providers adapt existing domain contracts:

- `server-audience-scope-impl` resolves `Audience::Shared(scope_id)` to online
  members in that subtree;
- `server-player-visibility-scope-impl` lets two players see each other only
  when their nearest visibility facet is the same.

These remain different policies. Chat or a shared cell menu uses an
`Audience`; player entity synchronization uses `ServerPlayerVisibility`.

`server-player-visibility-scope-sync-mod` computes `PlayerJoined` and
`PlayerLeft` deltas when a live player changes visibility boundary. Initial
join and final leave stay owned by the session pipeline.

## Join ordering

Scope assignment often affects the very first packets a client receives. The
session pipeline therefore exposes:

```text
Receive
  -> Validate
  -> Register
  -> Initialize
  -> Sync
  -> Cleanup
```

`ServerPlayerJoined` is emitted in `Register`. Instance orchestrators run in
`Initialize`, where they can:

- create or select a node;
- assign membership;
- bind a world route;
- initialize scoped gameplay state;
- request a world transition.

Only after that does `Sync` build the join snapshot using the selected
visibility provider. `ServerPlayerReady` is emitted after `JoinAccepted` has
been queued and is appropriate for welcome messages or later synchronization.

Avoid assigning the initial scope after `Sync`: the client could briefly
receive players or state from the wrong instance.

## Cleanup and transient worlds

Removing a dynamic arena is a policy decision owned by its orchestrator.
A normal cleanup sequence is:

1. remove the player or subtree membership;
2. unbind scope world routes;
3. discard transient world data;
4. remove the scope subtree;
5. despawn returned ECS entities.

`ServerChunkWorld::discard_instance` clears resident chunks and asks storage to
discard that instance. The in-memory storage provider deletes its data.
Filesystem storage intentionally keeps durable files.

This distinction prevents a generic scope deletion from silently destroying a
persistent world.

## A custom monolithic orchestrator is valid

Patchwork modularity does not require every custom game's control flow to be a
different mod. A server developer may write one `main` feature mod that:

- validates players;
- assigns them to dynamic nodes;
- calls reusable mechanics conditionally;
- attaches game-specific ECS components;
- creates and destroys instances;
- chooses audiences and world routes.

That is not a god mod when domain mechanisms still live behind reusable
contracts and libraries. It is the application's policy boundary.

## Process boundaries

The current scope tree is in-process. It can model many runtime instances in
one executable, but it is not a cluster scheduler or message broker.

A future multi-machine deployment can place a global allocator above several
server processes. Each process can still use the same scope model for the
instances it hosts. Cross-process scope migration will need explicit durable
identity, serialization, handoff, and broker contracts; these concerns should
not be hidden inside the local ECS tree.
