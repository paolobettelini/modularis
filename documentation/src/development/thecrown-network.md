# TheCrown network and dynamic instances

TheCrown is a network of Modularis processes, not one special game server. It
contains four independently deployed roles:

```text
Patchwork-authenticated client
              |
              v
       thecrown-auth :9999
              |
              | PlayerWantsToJoin
              v
          NATS + Relay <----------------> standalone Leptos web application
              |
              | StartInstance, transfer tickets, whispers
              v
   N x thecrown-game processes :10000+
              |
              +-- Hub instances
              +-- Parkour instances
```

The Relay owns global routing state. A game process owns local ECS state. An
instance is a logical game space hosted by one physical game process. The
standalone web application observes and controls the network through Relay; no
HTTP or Leptos runtime is embedded in a game server.

This subsystem is also an architecture example. It shows how a custom server
can keep one application-specific orchestration mod while reusing neutral
Patchwork services and pure gameplay libraries.

## Compositions and executables

The network adds two top-level modpacks:

| Modpack | Default port | Responsibility |
| --- | ---: | --- |
| `thecrown-auth.toml` | `9999` | Authenticate the Patchwork account, ask Relay for a destination, then transfer the client |
| `thecrown-game.toml` | `10000` | Register a physical game worker and host Relay-assigned Hub and Parkour instances |

Both import `server-core.toml` and `server-patchwork-auth.toml`. They do not
import `server-base.toml` or `server-vanilla.toml`. This prevents crafting,
portals, default inventories, global chat, biome generation, and other blanket
vanilla policy from becoming accidental requirements.

The same `client.toml` connects first to auth and follows later transfer
packets. The transfer behavior is a separate client mod, so a client profile
can omit TheCrown routing while retaining ordinary Modularis networking.

The old `thecrown.toml` and embedded web server were removed. The standalone
services remain under:

```text
thecrown/
├── thecrown-relay/
└── thecrown-web/
```

Shared protocol, NATS, token, and database crates live under `mods/` as plain
Rust libraries. They are dependencies, not Bevy runtime mods.

## Configuration used by the local game worker

`thecrown-game-config-api` defines the configuration resource. The current
`thecrown-game-config-local-impl` provides development values in code:

```text
server_id                 modularis-local-<random UUID>
public address            127.0.0.1
public port               10000
NATS URL                  nats://127.0.0.1:4222
request timeout           2000 ms
Relay subject             thecrown.relay
web subject               thecrown.web
game-server prefix        thecrown.gameserver
web address               127.0.0.1:8080
```

The random server ID is suitable for local runs. A deployment should replace
this provider with a stable, externally configured ID and reachable public
address. Consumers depend on the API, not on this local implementation.

`thecrown-auth` intentionally has no game-server identity. It must never send
`RegisterServer`, `AuthUserJoin`, or `PlayerQuit`, because Relay must not treat
the entry process as an instance host or online player location.

## Entry authentication flow

The auth server performs the normal Patchwork account handshake first. This
provides the trusted account UUID and username; no client-selected game name is
used.

After `ServerPatchworkPlayerJoined`:

1. `thecrown-auth-main-mod` emits `RequestTheCrownAdmission`.
2. `thecrown-auth-relay-nats-mod` sends a NATS request:

   ```text
   PlayerWantsToJoin { PlayerIdentity { uuid, username } }
   ```

3. Relay checks bans, stores the identity, and chooses a Hub instance.
4. Relay answers with `AccomodatePlayer`.
5. `Ban` and `Unavailable` become a normal `ServerKickRequested`, so the client
   sees the reason through the existing disconnect UI.
6. `Join { transfer }` becomes a client-bound `TransferPlayer` packet.

The auth NATS adapter runs on a dedicated Tokio thread and reports results back
through ECS messages. A failed worker or request produces an explicit admission
error instead of leaving a connected player waiting forever.

## Client transfer flow

`thecrown-network-message-types` defines the Modularis transport payloads:

```rust
TransferPlayer { transfer: TransferPacketData }
AuthenticateTransfer { transfer: TransferPacketData }
TransferAuthenticated
```

`thecrown-network-messages-mod` contributes them to network codegen. It does
not own transport behavior.

When `client-thecrown-transfer-mod` receives `TransferPlayer`, it:

1. replaces `ClientConnectionTarget` with the supplied address and port;
2. installs the namespaced `thecrown:relay-transfer` session join gate;
3. returns to the main-menu state to tear down the old TCP session cleanly;
4. immediately starts a connection to the destination;
5. completes the normal Patchwork game-server authentication handshake;
6. sends `AuthenticateTransfer` with Relay's one-use transfer data;
7. releases the join gate only after `TransferAuthenticated` arrives.

Patchwork backend authentication and TheCrown Relay admission are intentionally
different proofs. The Relay cookie is not passed to the Patchwork backend as a
backend transfer ticket.

`ClientSessionJoinGates` is generic and namespaced. Other mods may block the
ordinary `JoinRequest` for their own asynchronous prerequisites without
depending on TheCrown.

## Game-server registration

`thecrown-game-relay-nats-mod` owns the game worker's NATS connection. Its
worker performs startup in this exact order:

1. connect to NATS;
2. subscribe to `thecrown.gameserver.<server_id>`;
3. publish `RegisterServer { server_id, address, port }`.

Subscribing first is required because core NATS does not retain messages and
Relay may publish `StartInstance` immediately during registration.

Dropping the worker sends its shutdown command, publishes
`UnregisterServer`, and joins the worker thread. This is the clean-shutdown
path; Relay also removes stale state when a server re-registers with the same
ID.

The NATS adapter only translates external protocol packets into domain ECS
messages such as `RelayStartInstance`, `AuthenticateRelayTransfer`, and
`RelayExecuteTransfer`. It does not create scopes, mutate worlds, or assign
players.

## Dynamic instance lifecycle

A game worker starts with only the root scope:

```text
patchwork:root
└── thecrown
```

It does not pre-create Hub or Parkour instances. Relay sends:

```text
StartInstance { GameInstanceSpec { instance_id, mode } }
StopInstance { instance_id }
```

The game process stores exact Relay IDs in `TheCrownRuntime.instances`; it
never derives or renames them. Starting the same ID twice is idempotent.

Only an empty instance may be stopped. Relay and the game worker both reject
removal while players remain. This preserves authoritative session location
instead of silently forgetting connected players. Stopping an empty instance:

- unbinds its world route;
- removes its RAM template and resident chunks;
- removes its scope subtree;
- despawns its ECS marker entities.

Future scaling policy belongs in Relay. The game process only exposes safe
lifecycle primitives.

## Destination admission and one-use cookies

A direct, ordinary connection to `thecrown-game` is not enough to join. The
destination must verify both independent identities:

- the Patchwork backend account bound to the TCP address;
- the Relay transfer data presented by that connection.

The destination flow is:

```text
client AuthenticateTransfer
        |
        v
AuthenticateRelayTransfer ECS message
        |
        v
Relay AuthUserJoin { player_uuid, server_id, instance_id, cookie }
        |
        v
ServeAuthResult { value }
```

Relay consumes the cookie before validation. It therefore cannot be replayed,
including after a failed attempt. `thecrown-game-session-mod` stores a short
local approval keyed by socket address. Its admission rule allows the normal
session join only while that approval exists. Binding the final player ID
removes the approval and records:

```rust
TheCrownGamePlayerSession {
    identity,
    instance_id,
    mode,
}
```

The session resource has indices by local `PlayerId` and account UUID. UUID is
used for Relay operations; `PlayerId` remains the efficient local ECS/network
handle.

If the assigned local instance is absent or has a different mode, the player
is kicked instead of being left connected without a world.

## Player exit and transfer

`thecrown-game-session-mod` is the single owner of outbound Modularis transfer
packets. Commands and externally forced transfers emit
`TransferTheCrownPlayer`; the session mod resolves that to `ServerPacketOut`.

On `ServerPlayerLeft`, it removes the UUID/player index and publishes:

```text
PlayerQuit { player_uuid, server_id, instance_id }
```

This also happens when disconnecting from the source server during a transfer.
Relay compares the supplied old location with its current location, so a late
quit cannot remove a session already admitted elsewhere.

The following commands ask Relay for a transfer instead of choosing a local
destination directly:

| Command | Relay request |
| --- | --- |
| `/hub` | `PlayerEnterLobby` |
| `/play parkour` | `PlayerEnterMode { Parkour }` |
| `/server parkour2` | `PlayerEnterSpecificServer { instance_id }` |

`DataTransferResult::Join` uses the same central transfer event as every other
source. `NotFound` is shown as a personal chat message.

The standalone website may ask Relay to move a player. Relay sends
`GameServerPacket::ExecuteTransfer` to the physical process currently holding
the UUID. The game session index finds the local player across all instances
and sends the supplied transfer packet.

## Scope topology

Each logical instance has its own scope node. Facets decide which services are
shared; there is no single hardcoded meaning of an “instance.”

Hub:

```text
thecrown
└── thecrown:instance:hub1      [chat, visibility, shared world route]
    ├── player 1
    └── player 2
```

Parkour:

```text
thecrown
└── thecrown:instance:parkour1  [chat]
    ├── ...:player:1            [visibility, private world route]
    └── ...:player:2            [visibility, private world route]
```

Hub players share chat, world, and avatar visibility. Parkour players share
instance chat but have private worlds and cannot see each other's avatars.
Chat, visibility, and chunk routing remain separate facets, so another custom
mode can combine them differently without changing the scope core.

## Hub world

`thecrown-world-template-api` stores complete, palette-compressed `Chunk`
templates in RAM. `server-chunk-provider-thecrown-mod` is the selected primary
provider and clones the requested in-memory chunk, or returns an air chunk when
the template has no data at that position.

The current Hub loader reads:

```text
thecrown/data/hub/
  info.json
  data/chunk/index.bin
  data/chunk/regions/...
```

This is the same binary, globally indexed, region-grouped chunk format used by
the normal filesystem world storage. `thecrown-game-main-mod::hub` hardcodes
the application-owned path, decodes it once when an instance starts, and
installs the chunks in RAM. Every Hub instance gets a separate
`WorldInstanceId`, while all players inside that Hub use its shared route.

The standalone Rust tool under
`thecrown/scripts/minecraft-world-converter` reads Java Edition Anvil/NBT data
from `thecrown/scripts/minecraft_world`. It maps matching Minecraft block IDs
to composed `demo:*` IDs, maps unsupported blocks and fluids to stone, and
does not import entities, block entities, ticks, biomes, or other world
domains. Minecraft parsing therefore stays outside the game runtime and the
generic provider.

Hub admission also applies two instance policies:

- gravity `(0, -5, 0)`;
- a fresh pseudo-random model scale in `[0.4, 5.0]` for every Hub admission,
  including reconnects by the same account.

The template API, state provider, chunk provider, and TheCrown loader are
separate concerns. A future map format or database loader can replace only the
loader/state side.

## Parkour instances

Every admitted Parkour player gets a child scope and private
`WorldInstanceId`. `parkour-gameplay-lib` remains a plain, pure Rust library:
it receives positions and returns frame plans, block edits, score changes, and
optional teleports. It knows nothing about NATS, packets, audiences, chunk
providers, or storage. The library uses the generic voxel-frame value types,
but the TheCrown adapter owns the actual scoped frame registry and ECS motion
messages.

`thecrown-game-main-mod::parkour` is the application-specific adapter. It:

- applies generated edits to the player's routed world;
- creates one independent voxel frame for every parkour block;
- starts authoritative frame trajectories through `SetVoxelFrameMotion`;
- publishes generic block-edit events for client synchronization;
- requests authoritative relocation on reset;
- publishes personal score messages;
- emits a personal spatial note-block sound for successful checkpoints;
- loads and submits the player's persistent personal best through the Relay;
- drops the transient world and scope when the player leaves.

Parkour admission applies its own instance policy:

- gravity `(0, -20, 0)`;
- fixed player model and hitbox scale `1.0`;
- flight capability disabled.

These values are stored as part of the runtime instance policy rather than as
global TheCrown defaults. Hub and Parkour can therefore configure the same
generic gravity and scale services differently, and another mode can define a
third policy without changing either service provider.

This is deliberately monolithic policy inside TheCrown, while the mechanics it
reuses remain independent libraries and services.

### Parkour obstacle progression

Nominal path generation runs first and is unchanged: it chooses the next
integer block position from the previous nominal position. A separate RNG
stream then plans the obstacle frame. Consequently, adding or changing visual
motion does not perturb the sequence of nominal positions produced by an
existing parkour seed.

Each block receives one of these frame behaviors:

- `Normal`;
- `StaticTilt`;
- `Slider`;
- `Rotating`;
- `SliderTilted`;
- `SliderRotating`.

Progression uses a clamped smoothstep from score 15 to score 150. At maximum
difficulty, normal blocks retain roughly a quarter of the random weight. The
planner also forces recovery blocks every four to seven obstacles, after a very
difficult obstacle, or after a streak of three difficult obstacles.

The planner first estimates nominal jump cost from horizontal distance,
positive vertical travel, and total distance. Tilt, translation, and rotation
then share only the remaining accessory budget. They are not independently
maximized. This keeps a hard nominal jump from also receiving extreme movement
and rotation.

Sliders use one axis, mostly X or Z; rare Y movement is capped separately.
Static tilt is capped near 30 degrees and dynamic rotation near 12 degrees.
Animated endpoints use one shared half-traversal duration and cubic ease-in-out
with `PingPong`. A deterministic phase offset is stored in the authoritative
trajectory, so server collision and client presentation sample the same phase.

The block remains at frame-local `(0, 0, 0)`. Because voxel-frame coordinates
are corner based, each endpoint compensates its translation by the rotated
half-block vector. The visible/collidable block therefore rotates around its
center instead of orbiting around the local grid corner.

Checkpoint recognition consumes `ServerPlayerMovementApplied::support_surface`,
the frame identity selected by the same authoritative capsule/OBB solve that
accepted the movement. It does not compare the player against the nominal
world-space `BlockPos`. If an accepted packet has no final support marker, the
adapter falls back to the pure parkour library's swept foot test from the
previous to the accepted position, using current authoritative frame poses and
checking only future checkpoints. This covers clients that already resolved a
landing locally without letting the current checkpoint mask the next one. Both
paths remain independent from a concrete collision provider. The adapter also
checks the final authoritative registry position every server tick: a client
that stops sending input immediately after landing does not need to jump again
to produce another movement event. The selected
server and client surface-motion adapters keep a grounded player attached to
moving frame collision geometry.

To replace this progression without replacing TheCrown networking, keep the
same output boundary: deterministic frame plans plus local block edits. A
different planner can choose other probabilities or budgets while the adapter
continues to own scopes, frame registration, motion ECS messages, replication,
score publication, and cleanup.

### Persistent parkour records

The game worker never opens the player database directly. On Parkour admission
it emits `RequestRelayParkourRecord`; the NATS provider translates that ECS
request to `RelayPacket::GetParkourRecord`. The Relay owns the database query
and returns the current personal best. The player receives it as a personal
chat message after joining the arena.

Checkpoint progress stays only in the current `ParkourRun`. When a fall ends
the run, the update carries `completed_score`; only that run boundary may emit
`SubmitRelayParkourRecord`. If the initial record query is still in flight, the
best completed score is queued until the query returns. The database update is
monotonic and transactional: a delayed lower submission cannot replace a newer
higher score. A confirmed improvement produces a personal `New parkour record`
chat message. The standalone website already reads the same player row, so its
record view reflects the saved value without access to the game worker's Bevy
world.

This split keeps persistence out of `parkour-gameplay-lib`. The pure library
still owns only course and score rules; TheCrown policy chooses that this score
is persisted and at which lifecycle boundary, Relay transports it, and the
database owns consistency.

## Chat, whispers, and web login

Normal text resolves the nearest chat facet and is published to
`Audience::Shared(scope)`. This gives Hub-wide or Parkour-instance chat without
changing the neutral chat pipeline.

`/msg <name> <message>` sends `WhisperCommandByName` to Relay. Relay resolves
the persistent account name through its database, locates the online UUID, and
forwards `GameServerPacket::WhisperCommand` to the correct physical process.
The destination process resolves UUID to its local player ID and publishes a
personal chat message.

`/weblogin` requests `GenerateAuthToken { player_uuid }` from
`thecrown.web`. The returned token becomes a personal structured external-link
prompt whose URL is:

```text
http://127.0.0.1:8080/auth/<token>
```

TheCrown only publishes `PublishServerExternalLink`. The generic server network
adapter, client receiver and Bevy presenter own replication, the modal and OS
browser launching. This keeps web-login policy independent from one concrete
client UI. The presenter uses the generic exclusive `Modal` overlay state, so
opening a link releases gameplay input without spawning the pause menu behind
the prompt.

The web application is a separate Actix/Leptos executable. It has no access to
the Bevy `World`; all cross-process state goes through the shared protocol and
NATS.

## Crate boundaries

The main boundaries are:

| Crate | Kind | Role |
| --- | --- | --- |
| `thecrown-protocol` | plain library | Serializable Relay, game-server, web, identity, instance, and transfer types |
| `thecrown-common` | plain library | NATS client, token, and shared configuration helpers |
| `thecrown-database` | plain library | Relay/web persistence |
| `thecrown-network-message-types` | plain library | Modularis client/server transfer payloads |
| `thecrown-network-messages-mod` | contributor mod | Adds those payloads to network codegen |
| `thecrown-auth-relay-api` | API mod | Entry-server admission ECS contract |
| `thecrown-auth-relay-nats-mod` | provider mod | NATS implementation of entry admission |
| `thecrown-auth-main-mod` | policy mod | Patchwork identity to Relay admission and transfer/kick |
| `thecrown-game-config-api` | API mod | Physical game-worker configuration |
| `thecrown-game-relay-api` | API mod | Relay lifecycle, transfer, whisper, and web ECS contracts |
| `thecrown-game-relay-nats-mod` | provider mod | Dedicated NATS worker and protocol adapter |
| `thecrown-game-session-api` | API mod | Local UUID/player/instance session index and transfer event |
| `thecrown-game-session-mod` | implementation mod | Relay admission rule, one-use binding, quit, and transfer output |
| `thecrown-world-template-api` | API mod | Sparse RAM map templates |
| `thecrown-world-template-state-mod` | provider mod | Template resource implementation |
| `server-chunk-provider-thecrown-mod` | provider mod | Chunk generation from the selected template |
| `thecrown-game-main-mod` | application policy mod | Dynamic scopes, Hub/Parkour rules, and commands |

No Relay adapter contains gameplay logic, and no world provider understands
NATS or player admission.

## Local startup

Start NATS, then Relay and the standalone web process. Compose the two
Modularis servers and the client:

```sh
patchwork compose --modpack thecrown-auth --modpacks-folder ./modpacks \
  --mods-folder ./mods --cache ./build-thecrown-auth

patchwork compose --modpack thecrown-game --modpacks-folder ./modpacks \
  --mods-folder ./mods --cache ./build-thecrown-game

patchwork compose --modpack client --modpacks-folder ./modpacks \
  --mods-folder ./mods --cache ./build-client
```

Run the game worker before joining through auth so Relay has Hub instances to
select:

```sh
cargo run --manifest-path build-thecrown-game/thecrown-game/Cargo.toml
cargo run --manifest-path build-thecrown-auth/thecrown-auth/Cargo.toml
cargo run --manifest-path build-client/client/Cargo.toml
```

Point the client at `127.0.0.1:9999`. Relay transfers it to the selected game
worker at `127.0.0.1:10000`.

## Extension rules

When adding another TheCrown mode:

1. extend the shared protocol mode identity;
2. teach Relay how and when to allocate that mode;
3. add one local instance constructor that creates its scope facets and world
   route;
4. keep pure game rules in a reusable library when useful;
5. adapt those results to generic ECS events in TheCrown orchestration;
6. keep transfer, authentication, chat, and chunk providers unchanged unless
   the new mode genuinely needs a different provider.

A mode may share a world but isolate chat, share chat but isolate visibility,
or create one child world per player. Those choices belong in its runtime scope
topology, not in TCP, Relay, or the chunk core.
