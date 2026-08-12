# Permissions and server game modes

The client does not implement Creative, Survival, or Adventure as hardcoded
runtime modes. A game mode is server state plus a replaceable policy that
changes smaller capabilities. This keeps every consequence independently
usable by a custom server.

```text
SetPlayerGameMode
        │
        ▼
ServerPlayerGameModeChanged
        │
        ▼ registered mode policy
permission grants/denials + outline change
        │                    │
        ▼                    ▼
capability adapters       SetOutline packet
```

The generic game-mode state does not know what Creative means. The vanilla
library defines the current meaning, and its glue mod applies that meaning to
every player on the vanilla server.

## Generated permission IDs

Permissions are typed generated values, not strings passed around gameplay
systems. A small support mod contributes one declaration:

```toml
[package.metadata.mod]
support = true

[package.metadata.permission]
id = "example:moderator"
implies = ["vanilla:can-flight"]
```

`permission-registry-codegen` scans the composed permission contributors and
generates:

- `PermissionId`;
- `all_permissions()`;
- namespaced string conversion;
- direct and transitive implication checks.

Composition fails for duplicate IDs, duplicate generated Rust variants,
missing implication targets, or cycles. The current vanilla hierarchy is:

```text
Privileged
  ├─ CanFlight
  ├─ CanInteract
  └─ CanChangeOwnGameMode
```

A new permission belongs in its own contributor mod and in the permission
feature pack. Do not add it to a central handwritten enum.

## Owner-scoped grants

`ServerPlayerPermissions` stores explicit grants by player, permission, and
owner. Systems change them through:

```rust
SetPlayerPermission {
    player_id,
    owner: "my-mod:role".to_string(),
    permission: PermissionId::CanInteract,
    enabled: true,
}
```

The owner is important. Removing `my-mod:role` does not revoke the same
permission when a game mode, rank, temporary effect, or another mod still
grants it. Effective permissions include every permission implied transitively
by any explicit grant.

An owner-scoped explicit denial is available for narrower policy overrides:

```rust
SetPlayerPermissionDenied {
    player_id,
    owner: "vanilla:game-mode".to_string(),
    permission: PermissionId::CanInteract,
    denied: true,
}
```

A denial of the requested permission takes precedence over direct and inherited
grants. It does not revoke the parent role. Adventure can therefore disable
world interaction while leaving an administrator `Privileged` and able to use
administrative commands. Removing that denial restores the normal hierarchy.

Administrative policy can deliberately revoke every owner through
`ClearPlayerPermissionGrants`. This is stronger than normal feature cleanup and
is used only when the requested operation means "remove this authority",
regardless of where it came from. `/privilege` uses this contract; game modes
must not use it for the independent administrative role.

The ordered server pipeline is:

```text
Apply -> DeriveCapabilities -> Sync
```

`ServerPlayerPermissionsChanged` carries the complete effective set plus added
and removed differences. Capability adapters belong in `DeriveCapabilities`.
For example, the vanilla flight adapter maps `CanFlight` to the generic flight
capability API. Network sync then sends only the effective typed set to the
affected client.

The client cache is for presentation and intention availability only. The
server always validates the permission again before applying an operation.

## Generated game-mode identities

Game modes are a generated domain, just like permissions and dimensions. Each
identity is contributed by its own support crate:

```toml
[package.metadata.game_mode]
id = "vanilla:survival"
```

`game-mode-registry-codegen` generates the `GameMode` enum, iteration, parsing,
and stable namespaced IDs. The active contributors are:

- `game-mode-creative`;
- `game-mode-survival`;
- `game-mode-adventure`.

A custom composition may add another contributor without editing a central
enum. `server-player-game-mode-api` owns only neutral per-player state,
`SetPlayerGameMode`, and change events.

## Separate mode policies

Identity, policy dispatch, mode semantics, and commands are separate concerns.
`server-player-game-mode-policy-mod` is a generic dispatcher over registered
policies. Each vanilla mode has its own reusable library and thin registration
mod:

- `server-game-mode-creative-vanilla-lib` and `-mod`;
- `server-game-mode-survival-vanilla-lib` and `-mod`;
- `server-game-mode-adventure-vanilla-lib` and `-mod`.

The command mod only parses a generated `GameMode` and emits
`SetPlayerGameMode`. It does not contain permission or outline logic.

The current policies are deliberately small:

| Mode | Permission policy | Outline |
| --- | --- | --- |
| Creative | grants `CanInteract` and removes the Adventure denial | enabled |
| Survival | grants `CanInteract` and removes the Adventure denial | enabled |
| Adventure | removes grants and explicitly denies `CanInteract` | disabled |

No game-mode policy changes `Privileged`. Administrative role assignment is an
independent concern and only `/privilege` changes it in the current vanilla
composition. This also means a privileged administrator may switch between
Creative, Survival, and Adventure without locking itself out of `/gamemode`.

`server-player-default-creative-vanilla-mod` is a separate join policy. It puts
new players in Creative and grants `Privileged`; the hierarchy then supplies
flight, interaction, and game-mode command capabilities. The generic state
provider does not choose a default. A scoped or minigame server can omit that
join policy, omit selected vanilla mode policies, register policies with
different semantics, or emit mode/capability requests from its own
orchestrator.

## Commands and authorization

`ServerCommandSource` includes the caller's effective permission set.
`ServerCommandRegistry::register_restricted` applies the same permission to
Brigadier availability and direct execution:

- unavailable roots are absent from autocomplete;
- manually entering a restricted command returns `This command is not
  available`;
- gameplay handlers still recheck authority for stronger forms such as
  targeting another player.

At this command boundary, `Privileged` is an explicit superuser role: it
satisfies every registered command requirement, including permissions added by
future mods that are not part of the vanilla implication tree. This bypass is
local to command authorization; it does not silently bypass unrelated gameplay
validators.

The vanilla `/gamemode` command derives accepted values and completion from the
generated game-mode registry. With the current contributors it supports:

```text
/gamemode <creative|survival|adventure>
/gamemode <player> <creative|survival|adventure>
```

The first form needs `CanChangeOwnGameMode`, which `Privileged` implies. The
player-targeted form also needs `Privileged`. Switching modes does not alter
that role; only `/privilege` can remove it in the vanilla composition.

The vanilla administration pack also contributes:

```text
/privilege <player>
```

Only a currently privileged source can see or execute it. It toggles the
target's explicit `Privileged` authority. Revocation clears every owner so the
restricted command roots disappear from subsequent server completions and
manual execution is rejected. Public policy commands such as `/clear` and
`/tps` remain available by design.

Command refresh is driven by permission state, not by `/privilege` or any other
specific command. Every accepted `SetPlayerPermission`,
`SetPlayerPermissionDenied`, or `ClearPlayerPermissionGrants` intention emits
`ServerPlayerPermissionsChanged`. The generic network adapter synchronizes the
effective snapshot, and the client state emits `ClientPlayerPermissionsChanged`.

`client-chat-permission-refresh-mod` listens only to that client-side domain
event. It invalidates the current completion request, clears stale roots
immediately, and requests completion again for the current input. An old
in-flight response cannot restore removed commands because the client advances
the request generation at the same time. A rank mod, game-mode policy, timed
effect, operator console, or future custom rule therefore gets identical
refresh behavior without depending on the chat UI.

Mods must request permission mutations through the permission ECS messages.
Writing directly into the state resource bypasses validation, derivation,
network synchronization, and downstream reactions and is not a supported
extension path.

Administrative vanilla commands such as teleport, give, kick, speed, flight
speed, scale, and gravity require `Privileged`. `/flight` requires
`CanFlight`, while its other-player form additionally requires `Privileged`.
Personal `/clear` and read-only `/tps` remain public policy choices.

## Interaction authorization

The vanilla block-breaking, item-placement, crafting-table, and portal-ignite
glue checks effective `CanInteract` on the server. The reusable mechanic
libraries remain permission-free: a custom orchestrator can apply its own
scope, team, region, or minigame rule before calling them.

This distinction is intentional:

- a library provides a mechanic;
- a vanilla glue mod decides when it always applies;
- permission state represents reusable facts;
- the final server policy may combine permissions with arbitrary ECS state.

Do not put a permission check inside a generic world, transport, or storage
provider.

## Outline capability

Outline visibility has a separate state and packet family:

```rust
SetPlayerOutline { player_id, enabled }
SetOutline(bool)
```

The client receiver updates `ClientBlockOutlineEnabled`. The renderer hides or
restores existing owner-keyed outline roots without deleting their logical
state. This lets Adventure mode disable looked-block presentation while other
modes restore it immediately, and lets custom server rules use the same
capability without introducing a game-mode packet.

## Adding a permission-driven capability

1. Contribute a typed permission in a support mod.
2. Add the contributor to a feature pack imported by both protocol sides.
3. Keep authoritative grants in `ServerPlayerPermissions` with a stable owner.
4. Listen to `ServerPlayerPermissionsChanged` in
   `ServerPlayerPermissionSet::DeriveCapabilities`.
5. Change a narrow capability API or emit a domain event.
6. Synchronize only the client state required for presentation or prediction.
7. Recheck the permission at the authoritative gameplay boundary.

This supports roles, temporary effects, world-specific policies, and custom
servers without turning game modes into a central switch statement.
