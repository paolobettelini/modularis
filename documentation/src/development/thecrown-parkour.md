# TheCrown multi-instance parkour server

`thecrown.toml` is an alternate server composition. It is both a playable
parkour prototype and an architecture test: the same client connects to a
server whose world, visibility, chat, lifecycle, and game rules differ from the
vanilla composition.

## Goals

The server demonstrates that one process can host several runtime game
instances where:

- every player is assigned automatically;
- each parkour instance has a separate chat;
- players in the same chat still cannot see each other's avatars;
- each player has a private generated block course;
- player courses use transient in-memory chunk storage;
- block breaking and vanilla item placement are unavailable;
- idle instances can be created and removed dynamically.

It also demonstrates that a custom server may keep one monolithic policy mod
while reusing small Patchwork libraries.

The composition starts from `server-core.toml`, not the larger
`server-base.toml` umbrella. It then selects only the neutral chat, outbound
block-edit, chunk-request, flight, and sun pipelines it actually uses.

## Runtime topology

At startup, `thecrown-main-mod` creates:

```text
patchwork:root
└── thecrown
    ├── thecrown:parkour-1       [chat]
    └── thecrown:parkour-2       [chat]
```

Two instances are prewarmed. Each accepts up to 16 players. When all current
instances are full, another parent scope is created. Assignment uses a stable
mixed player ID so available instances are selected without always filling the
first one.

For player 7 in parkour instance 2:

```text
thecrown:parkour-2
└── thecrown:parkour-2:player-7  [world, visibility]
```

The leaf ECS entity carries:

- `TheCrownPlayerArena`;
- `ParkourRun`.

The parent carries `TheCrownParkourInstance`.

## Join flow

The main mod listens to `ServerPlayerJoined` in
`ServerPlayerSessionSet::Initialize`.

For each new player it:

1. chooses or creates a parkour parent;
2. creates a private player leaf;
3. binds a unique `WorldInstanceId` to the leaf;
4. assigns the player to the leaf before the join snapshot;
5. initializes a deterministic `ParkourRun`;
6. writes the initial course into that player's routed world;
7. requests a `PlayerWorldChanged` transition to the spawn.

The later session sync sees the final visibility scope. `JoinAccepted`
therefore contains only the local player, and no remote avatar is spawned.

`ServerPlayerReady` is used for the personal welcome message because it occurs
after the accepted packet has been queued.

## World composition

TheCrown selects:

- `server-chunk-provider-parkour-empty-mod`;
- `server-chunk-routing-scopes-mod`;
- `server-chunk-storage-memory-impl`;
- `server-chunk-world-dynamic-impl`.

The provider returns empty chunks. `ParkourRun` adds only the course blocks.
Each player's route uses the same provider but a different world instance ID,
so equal coordinates do not share state.

The normal chunk request, residency, mutation, and block-edit sync systems do
not know that this is a minigame. They operate on the route selected for the
player.

The filesystem provider is not selected. When a player leaves,
`discard_instance` deletes both resident and in-memory stored chunks for that
private world.

## Parkour mechanic library

`parkour-gameplay-lib` is not a Patchwork mod. It contains pure game state and
rules:

```rust
pub struct ParkourRun {
    blocks: VecDeque<ParkourBlock>,
    score: i32,
    combo: i32,
    // time and deterministic RNG state
}
```

It receives a position and returns a `ParkourUpdate`:

```rust
pub struct ParkourUpdate {
    pub edits: Vec<ParkourBlockEdit>,
    pub teleport: Option<[f32; 3]>,
    pub score_changed: bool,
    pub score: i32,
    pub combo: i32,
}
```

The library does not:

- mutate chunks;
- send packets;
- choose audiences;
- read player sessions;
- create scope nodes.

This makes it usable from TheCrown's monolithic main, a future smaller glue mod,
a test, or a different runtime host.

On reset it creates ten blocks and teleports the player above the first block.
Landing farther along the queue:

- removes passed blocks;
- appends the same number of new random blocks;
- increments score;
- updates combo based on elapsed time.

Falling more than the configured distance below the start resets the run.

The original prototype played a pitched note on every checkpoint. The demo
does not yet have a generic server-to-client sound contract, so the main mod
contains a clear `TODO(audio)` at the result boundary. Sound should later be a
separate event and network feature, not a direct call inside the parkour
library.

## Chat and visibility are intentionally different

Normal chat input resolves the player's nearest chat facet and publishes:

```rust
Audience::shared(chat_scope)
```

`server-audience-scope-impl` expands that parent subtree, so all players in one
parkour instance receive the message. Players in another parkour instance do
not.

Player visibility resolves the nearest visibility facet. Because every leaf is
its own boundary, no two players share one and no remote player model is
synchronized.

This is the key architecture test: “same chat” does not imply “same world” or
“same entity visibility.”

## Why blocks cannot be broken

The network and event infrastructure still accepts a break intention, but
TheCrown does not select:

- `server-block-edit-world-mod`;
- `server-place-block-item-use-mod`;
- the default inventory loadout;
- crafting-table behavior.

No authoritative world effect claims the request, so the block remains
unchanged. The server did not need a hardcoded adventure-mode branch in its
session or world implementation.

## Reused blanket policies

TheCrown selects the vanilla collision and jump glue because those rules apply
to every current TheCrown arena. Their mechanics are still available as
libraries. If a later TheCrown lobby needs different movement, the composition
can omit the blanket glue and the main mod can call the libraries only for
parkour leaf scopes.

This is a useful rule:

- select a vanilla glue mod when its policy is valid everywhere in the
  application;
- call its mechanic library conditionally when policy differs by runtime
  scope.

## Cleanup

On `ServerPlayerLeft`, the main mod:

1. removes its player record;
2. unbinds the player's world route;
3. discards the transient world instance;
4. removes and despawns the player scope subtree;
5. decrements the parent population;
6. removes an empty dynamically created parent when more than the prewarmed
   count remains.

Persistent worlds would use a different lifecycle policy and must not call
destructive transient cleanup.

## Compose and run

From the project root:

```sh
patchwork compose \
  --modpack thecrown \
  --modpacks-folder ./modpacks \
  --mods-folder ./mods \
  --cache ./build-thecrown
```

Then:

```sh
cargo run --manifest-path build-thecrown/thecrown/Cargo.toml
```

Use the normal `client` composition to connect.

## Extension examples

### Shared course, hidden players

Bind the world route on the parkour parent but keep visibility facets on player
leaves. Every player sees the same blocks while avatars remain hidden.

### Visible racers with private chat

Put the visibility facet on the match parent and chat facets on team children.
All racers see each other, but each team has separate chat.

### Different rules per arena

Attach a component such as:

```rust
#[derive(Component)]
struct ArenaRules {
    allow_flight: bool,
    gravity: Vec3,
    allow_crafting: bool,
}
```

Custom systems resolve the player's arena entity and call reusable mechanic
libraries only when the component permits them.

### Shared immutable lobby map

Bind one world route to a lobby parent. Create child scopes for separate chat
or visibility groups. All children read the same resident chunks without
duplicating the map.

### External instance allocator

A future allocator can decide which process hosts a match and then instruct
that process to create the local subtree. The current implementation does not
provide broker transport, cross-process migration, or durable matchmaking.
Those should be separate infrastructure rather than hidden in
`thecrown-main-mod`.
