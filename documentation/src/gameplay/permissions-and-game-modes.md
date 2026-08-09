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
        ▼ vanilla policy adapter
permission changes + outline change
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

Administrative policy can deliberately revoke every owner through
`ClearPlayerPermissionGrants`. This is stronger than normal feature cleanup and
is used only when the requested operation means "remove this authority",
regardless of where it came from. Game-mode downgrades and `/privilege` use this
contract; independent feature mods should continue to remove only their own
owner-scoped grants.

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

## Vanilla game-mode policy

`server-player-game-mode-api` owns neutral per-player state and change events.
`server-player-game-mode-vanilla-lib` is a pure reusable policy function, while
`server-player-game-mode-vanilla-mod` is the blanket ECS glue selected by the
vanilla modpack.

The current policies are deliberately small:

| Mode | Permission policy | Outline |
| --- | --- | --- |
| Creative | grants `Privileged` | enabled |
| Survival | clears every `Privileged` grant and grants `CanInteract` | enabled |
| Adventure | clears every `Privileged` and `CanInteract` grant | disabled |

The strong clears make an explicit mode downgrade authoritative: a stale rank
or default grant cannot leave administrative commands enabled. A later server
rule may still grant a capability again after the mode policy has run, or a
custom composition may use a weaker owner-scoped policy.

`server-player-default-creative-vanilla-mod` is a separate join policy. It puts
new players in Creative and grants `Privileged`; the hierarchy then supplies
flight, interaction, and game-mode command capabilities. The generic state
provider does not choose a default. A scoped or minigame server should omit the
join policy and blanket game-mode glue, call `vanilla_game_mode_policy` only
where desired, or define a different policy over the same contracts.

## Commands and authorization

`ServerCommandSource` includes the caller's effective permission set.
`ServerCommandRegistry::register_restricted` applies the same permission to
Brigadier availability and direct execution:

- unavailable roots are absent from autocomplete;
- manually entering a restricted command returns `This command is not
  available`;
- gameplay handlers still recheck authority for stronger forms such as
  targeting another player.

The vanilla `/gamemode` command supports:

```text
/gamemode <creative|survival|adventure>
/gamemode <player> <creative|survival|adventure>
```

The first form needs `CanChangeOwnGameMode`, which `Privileged` implies. The
player-targeted form also needs `Privileged`. Switching to Survival or
Adventure removes that authority, so the player cannot switch itself back.

The vanilla administration pack also contributes:

```text
/privilege <player>
```

Only a currently privileged source can see or execute it. It toggles the
target's explicit `Privileged` authority. Revocation clears every owner so the
restricted command roots disappear from subsequent server completions and
manual execution is rejected. Public policy commands such as `/clear` and
`/tps` remain available by design.

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
