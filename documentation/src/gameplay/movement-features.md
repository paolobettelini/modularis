# Gravity, jump, sprint, sneak, and flight

Movement features are independent mods layered onto the controller and server
pipelines.

## Gravity

`player-gravity-api` defines:

```rust
pub struct Gravity(pub Vec3);
```

The default is:

```text
(0, -20, 0)
```

Helpers provide:

- normalized gravity direction;
- gravity-relative up;
- rotation aligning world Y to gravity up;
- projection onto the gravity plane.

Gravity is a vector, not a scalar. Camera, planar movement, jump direction, and
flight direction use these helpers.

The client still exposes the controlled player's current gravity as the small
`Gravity` resource used by prediction and camera math. Server authority is
keyed by `PlayerId` through `ServerPlayerGravities`: it stores a default vector
plus optional per-player overrides.

### Gravity mods

- `player-gravity-vanilla-mod`: default resource provider;
- `server-player-gravity-api`: per-player state and change contracts;
- `server-player-gravity-state-mod`: neutral authoritative storage;
- gravity network message contributors;
- `server-player-gravity-network-sync-mod`: visibility-scoped join/runtime synchronization;
- `client-player-gravity-map-state-mod`: cache keyed by the subject `PlayerId`;
- `client-player-gravity-network-receive-mod`: updates the subject cache and applies only the local subject to the prediction resource;
- `client-player-gravity-prediction-vanilla-mod`: local force integration.

Server gameplay rules query the actor's gravity rather than a global policy.
This includes jump direction, gravity integration, reach origin, block
placement checks, and movement validation. Server authority chooses gravity;
client prediction applies the synchronized local value immediately.

`PlayerGravityChanged` contains both `player_id` and `gravity`. This is
important: a client can render two visible players with different up axes. A
gravity update for a remote subject never overwrites the controlled player's
local prediction resource. The Blocky and fallback renderers read
`ClientPlayerGravities` by avatar ID for rotation, walk-plane detection, and
name-label placement.

## Player model scale

Scale uses a parallel but independent set of contracts:

- `player-scale-api` owns the controlled client's `PlayerScale`, default `1`;
- `server-player-scale-api` owns per-player authoritative values and ordered
  set/change events;
- `server-player-scale-state-mod` stores state without choosing who may change
  it;
- the scale packet contributor adds `PlayerScaleChanged { player_id, scale }`;
- server sync sends the subject's value only to that player and currently
  visible viewers;
- `client-player-scale-map-state-mod` caches values for rendered subjects;
- `client-player-scale-network-receive-mod` updates both the map and, when the
  subject is local, `PlayerScale`;
- `client-player-scale-camera-vanilla-mod` applies the scaled eye height after
  normal camera synchronization.

Renderers multiply their asset-defined model scale by the synchronized player
scale. Name-label height is scaled and gravity-relative as well.

Physical dimensions remain a separate contract even though the selected
vanilla policy links them to model scale:

- `player-hitbox-api` defines `PlayerHitbox { radius, height, eye_height }`;
- `client-player-hitbox-state-mod` owns the controlled player's neutral hitbox;
- `client-player-hitbox-scale-vanilla-mod` derives it from `PlayerScale`;
- `server-player-hitbox-api` exposes per-player authoritative hitboxes and
  set/change events;
- `server-player-hitbox-state-mod` stores those values;
- `server-player-hitbox-scale-vanilla-mod` converts applied server scale
  changes into hitbox changes.

The FPS controller, grounded probe, sneak edge protection, server movement and
jump validation, reach eye origin, player occupancy checks, and portal overlap
all use the resulting dimensions. A player at scale `0.5` is therefore `0.9`
blocks high and can fit below a one-block ceiling. A custom composition can
omit either scale-to-hitbox adapter and install a different size policy without
changing model replication or collision algorithms.

## Base movement speed

`player-speed-api` defines the local `PlayerSpeedMultiplier`; `1` is normal
base speed. The FPS controller applies it before optional movement modifiers
such as sprint.

On the server, `ServerPlayerSpeeds` stores the default and per-player
overrides. `SetServerPlayerSpeed` is the write contract and
`ServerPlayerSpeedChanged` is the applied-result contract. Separate state and
network-sync mods apply changes and synchronize only the affected player.

The collision validator scales its permitted movement delta by the same
authoritative multiplier. A command, role system, status effect, region rule,
or custom progression mod can therefore change speed without depending on the
controller or transport implementation.

Flight speed deliberately does not reuse this state. `player-flight-speed-api`
defines the local `PlayerFlightSpeedMultiplier`, whose default is `2`, while
`server-player-flight-speed-api` owns default/per-player authoritative values
and ordered change events. Dedicated network contributor, server sync, client
receive, and state mods replicate it. `/speed` and `/flightspeed` therefore
change independent values, and a server may install different policies for
each.

## Jump

`JumpConfig` provides speed, a rearm interval, and an input-buffer duration.
Vanilla prediction uses a vertical impulse equivalent to `0.42` block per 20 Hz
tick.

The client jump mod:

1. reads the generated jump key setting in `Update` and buffers the press;
2. consumes that intention from the fixed physics schedule;
3. requires local `Grounded`;
4. removes existing up-axis velocity;
5. applies jump speed opposite gravity;
6. clears grounded state and starts a short rearm gate;
7. sends `PlayerJumpRequest`.

The buffer prevents a short key press from being lost between two 20 Hz physics
ticks and briefly preserves presses made just before landing. The rearm gate
and grounded requirement prevent spacebar spam from repeatedly retriggering a
jump at one contact boundary.

The server jump mod receives the intent and checks authoritative ground contact
against the player's world. The actual vertical position still arrives through
the normal client movement request and is checked by the movement pipeline.

Ground probing and jump validation are exposed by
`server-player-jump-lib`. `server-player-jump-vanilla-mod` is the always-on
adapter; a custom game can call the library only in scopes where jumping is
enabled.

`LocalPlayerJumped` is a narrow client ECS result event. The independent
`client-player-sprint-jump-vanilla-mod` listens to it and, while sprint is held,
adds an impulse equivalent to `0.20` block per tick in the gravity-relative
camera-forward direction. Servers or clients that do not want this rule can
omit that mod without replacing jump itself.

A stronger server can track velocity and make jump intent directly update an
authoritative physics state.

## Sprint

Sprint is a client movement modifier, not part of the input backend or FPS
controller.

`client-player-sprint-vanilla-mod` runs in:

```text
PlayerControllerSet::MovementModifiers
```

It reads the generated sprint key, left Control by default, and multiplies the
current planar intent.

Removing the mod removes sprint while preserving movement.

If sprint affects server rules, add a server-visible sprint intent and
validator. The current demo validates only movement displacement/collision.

## Sneak

Sneak is not a boolean hardcoded into the input backend or controller. The
vanilla composition assembles it from independent mods:

| Mod | Responsibility |
| --- | --- |
| `client-setting-sneak-key` | contributes `controls.sneak_key`, defaulting to left Shift |
| `client-player-sneak-state-mod` | provides the neutral `LocalPlayerSneak` resource and change event |
| `client-player-sneak-input-vanilla-mod` | maps the configured held key to local sneak state |
| `client-player-sneak-speed-vanilla-mod` | multiplies planar speed while sneaking |
| `client-player-sneak-edge-protection-vanilla-mod` | constrains supported movement before collision resolution |
| `client-player-sneak-camera-vanilla-mod` | quickly interpolates the local camera to a lower eye offset, proportional to player scale, after normal camera synchronization |
| `client-player-sneak-block-interaction-bypass-vanilla-mod` | routes right-click directly to the held item before block handlers |

The speed multiplier, camera offset/transition time, and edge-probe constants
live in the mods that own those policies. A client may keep configurable sneak
input but replace only its movement speed or camera presentation.

Edge protection applies only while grounded. It samples the requested planar
path along two gravity-relative axes and clamps each component at the last
supported point. Jumping and flight are not converted into grounded movement.

Those samples use `CollisionService::has_support`, not full movement
resolution. Collision providers may attach an optimized support query; the
block-AABB provider scans only the thin gravity-facing surface of the hitbox.
This keeps sneak usable for strongly scaled players because the probe cost
grows with hitbox surface area instead of its complete volume. Providers that
do not implement the optional optimization transparently use the generic
resolver fallback.

The interaction behavior matches the usual voxel-game convention: sneaking
does not disable block breaking. It bypasses right-click block activation, so a
held placeable item can be used against a crafting table instead of opening its
menu.

## Flight architecture

Flight is split into:

1. authoritative capability;
2. capability synchronization;
3. client state;
4. optional vanilla controls;
5. grant policy.

### Server capability

`server-player-flight-api` contains:

```rust
SetPlayerFlightCapability
ServerPlayerFlightCapabilityChanged
ServerPlayerFlightCapabilities
ServerPlayerFlightSet::{Apply, Sync}
```

`server-player-flight-capability-mod` is neutral. Capability defaults to off.

It removes state when a player leaves.

`server-player-flight-network-sync-mod` sends grants/revocations to the affected
player.

### Grant policy

`server-player-flight-grant-all-vanilla-mod` listens to join events and writes:

```rust
SetPlayerFlightCapability {
    player_id,
    enabled: true,
}
```

Only this policy is vanilla. A custom server can:

- omit it;
- grant by permission;
- grant by level;
- revoke in a region;
- change capability at runtime.

### Client state and controls

`LocalPlayerFlight` stores:

- `capability_enabled`;
- `flying`.

Revocation immediately exits flight.

The vanilla control mod:

- detects a double-tap of the jump key within `0.3` seconds;
- toggles flight only when capability is enabled;
- uses jump to ascend;
- uses the configured sneak key to descend;
- uses the separate synchronized flight-speed multiplier for both planar and
  vertical speed;
- runs in `ForceOverrides`;
- clears grounded state.

When an overlay is open, vertical flight input becomes zero, but flight state
can remain enabled.

## Adding a capability policy

Create a server feature mod that emits `SetPlayerFlightCapability`.

Example:

```rust
fn grant_admin_flight(
    mut joined: MessageReader<ServerPlayerJoined>,
    permissions: Res<Permissions>,
    mut changes: MessageWriter<SetPlayerFlightCapability>,
) {
    for event in joined.read() {
        changes.write(SetPlayerFlightCapability {
            player_id: event.player_id,
            enabled: permissions.is_admin(event.player_id),
        });
    }
}
```

The policy does not send packets directly. The existing sync mod observes the
applied change.

## Adding a gravity, scale, or speed policy

A policy mod should emit `SetServerPlayerGravity`, `SetServerPlayerScale`,
`SetServerPlayerSpeed`, or `SetServerPlayerFlightSpeed`. It should not mutate
the maps or send packets directly. Examples include dimension defaults, random
gravity on join, level-based scale or speed, temporary effects, and admin
commands.

Gravity and model scale have visibility-scoped subject caches because they
change remote avatar presentation. Ground and flight speed remain local-only
protocol state because remote rendering does not currently need them. Keep the
controlled-player resources separate from the keyed presentation maps.
