# Player movement and prediction

The movement design follows a client-predicted, server-validated model.

The client immediately moves the local player and camera. It periodically sends
its resulting position and look angles. The server validates the request and
sends corrections only when needed.

## Local player components

`client-player-controller-api` defines:

```rust
Player
PlayerVelocity(Vec3)
PreviousPlayerPosition(Vec3)
Grounded(bool)
```

Shared dimensions:

- radius: `0.3`;
- height: `1.8`;
- eye height: shared through `player-hitbox-api`.

The local player's transform represents feet position.

## Controller pipeline

```text
PlayerControllerSet::Input
  -> MovementModifiers
  -> ApplyMovementIntent
  -> GravityForces
  -> JumpForces
  -> Forces
  -> ForceOverrides
  -> MovementConstraints
  -> Movement
  -> PostMovement

Update: CameraSync -> CameraModifiers
```

Physics runs in `FixedUpdate`. The selected
`client-player-physics-tick-20hz-vanilla-mod` provides a `20 Hz` step, but the
controller depends only on `ClientPlayerPhysicsTickApi`, so another composition
can select a different provider. Camera synchronization remains in `Update` and
interpolates `PreviousPlayerPosition` to the latest simulated position using
Bevy's fixed-step overrun fraction. Rendering therefore stays smooth without
making physics frame-rate dependent.

### Input

The FPS controller projects camera forward onto the plane perpendicular to
gravity and builds `PlayerPlanarMovementIntent`.

The intent contains:

```rust
direction: Vec3
target_speed: f32
speed_multiplier: f32
```

`target_speed` is selected by the active movement mode. Walking uses base walk
speed times the server-synchronized normal speed multiplier. Flight replaces it
with base walk speed times the separate flight-speed multiplier. The intent does
not directly contain sprint or status-effect knowledge.

### Movement modifiers

Optional mods multiply or modify the intent. Sprint currently applies a `1.55`
speed multiplier while the configured key is held.

Other mods can implement:

- slow effects;
- terrain speed;
- equipment bonuses;
- temporary knockback restrictions.

### Apply intent and inertia

The controller does not replace planar velocity when WASD or camera direction
changes. The optional `client-player-inertia-vanilla-mod` treats normalized
input as acceleration added to existing planar velocity. At each fixed tick it:

1. applies acceleration in the current gravity-relative input direction;
2. lets collision move the player with the accumulated velocity;
3. applies planar drag in `PostMovement`.

The vanilla air drag is `0.91` per tick. Ground drag is `0.25`, giving normal
movement much faster stopping and reversal while keeping airborne steering
deliberate. Ground and air use separate acceleration rules that converge to the
requested target speed. Existing airborne velocity is not rotated when the
camera turns.

This behavior is policy rather than controller infrastructure. Omitting the
inertia mod leaves the intent and fixed-step contracts available for another
movement implementation.

### Forces and overrides

Gravity and jump have separate `GravityForces` and `JumpForces` phases. Other
impulses can use `Forces`; the vanilla sprint-jump mod listens to
`LocalPlayerJumped` and adds a horizontal forward impulse only when sprint is
held. Forward is projected from camera yaw onto the plane perpendicular to
gravity; changing strafe input does not rotate this impulse.

Flight runs in `ForceOverrides`, allowing it to replace the velocity component
along gravity-up after ordinary forces without hardcoding flight into gravity.
The vanilla flight policy also replaces planar velocity directly from the
current flight intent, so flight starts, stops, and turns almost immediately
instead of inheriting walking or airborne drag.

`MovementConstraints` is deliberately separate from force calculation and
collision resolution. Optional policies can reduce the final requested
displacement without owning the controller. The vanilla sneak edge-protection
mod uses this phase.

### Movement

The collision service resolves the fixed tick's movement and returns:

- resolved position;
- world contact normals and surface identities;
- walkable support information.

Inward velocity is clipped against contact planes, not global axis flags.

### Camera sync and interpolation

The camera follows:

```text
player feet + gravity up * eye height
```

It follows the locally predicted player, not the last server packet. The render
position is interpolated between the two most recent fixed physics positions,
which prevents a 20 Hz simulation from making vertical camera movement appear
stepped.

`CameraModifiers` runs after this absolute synchronization. Camera effects can
therefore add a frame-local offset without accumulating it or changing the
controller. The vanilla sneak camera mod uses this phase to lower eye height.

## Collision service and grounded state

The active character path uses a gravity-oriented capsule and shape-aware
root/frame geometry. Ground detection, sliding and step-up operate relative to
player up. See [character collisions](character-collisions.md) for the contracts,
algorithm, scale handling, surface attachment and authoritative frame motion.

The older closure API remains available to providers, but the default player
controller uses `resolve_character` and `character_support`.

## Movement network flow

The client sends a `PlayerMove` about every `0.05` seconds:

```text
movement_epoch
sequence
optional support-relative position
position
yaw
pitch
```

`sequence` is strictly increasing inside one movement epoch. The server rejects
duplicate and reordered sequences before gameplay validation. If one server
update drains several TCP frames for the same player, the session policy keeps
only the newest valid sequence. It therefore validates one coherent
displacement from the current authoritative registry position instead of
validating every queued packet from the same stale origin.

The server collects:

```rust
PendingServerPlayerMove {
    surface,
    source,
    player_id,
    movement_epoch,
    sequence,
    current_position,
    requested_position,
    accepted_position,
    yaw,
    pitch,
    rejected,
}
```

Validators can change `accepted_position` or set `rejected`.

The vanilla collision validator:

- limits one request delta to 2 world units;
- resolves the requested movement against server blocks;
- preserves world-scope-aware block queries.

The calculation is exposed separately by
`server-player-movement-collision-lib`. The vanilla validator applies it to
every pending move. A server with different rules per runtime scope can omit
that blanket validator and call the same resolver only for selected players.

The apply stage updates the registry and synchronizes visible remote players.

## Relocation and movement epochs

A teleport, respawn, dimension change, or world-instance change is not an
ordinary movement. It calls `ServerPlayerRegistry::relocate_player`, which
updates the authoritative position, increments that player's `movement_epoch`,
and clears sequence state. Packets produced before the relocation retain the
old epoch and are discarded even if TCP buffering makes them reach gameplay
systems later.

All relocation policies use the public `ServerPlayerRelocationSet` boundary:

```text
ServerPlayerMovementSet::Apply
  -> ServerPlayerRelocationSet::Apply
  -> ServerPlayerRelocationSet::Sync
```

Portal and command mods emit relocation intentions before this boundary. The
dimension/world state mods own the authoritative mutation and include the new
epoch in their client-bound update. A custom server must use this contract for
every discontinuous position change; writing `NetworkPlayer.position`
directly would bypass stale-packet invalidation.

The client resets its outgoing sequence when it accepts a newer epoch. Older
movement acknowledgements and older dimension/world updates are ignored.

## Corrections

The server acknowledges the newest applied sequence to the local player. The
same packet carries a `correction` flag. Normal acknowledgements advance
client bookkeeping without replacing the predicted position; `correction` is
set only when the move was rejected or when accepted and requested positions
differ by more than `0.15`. A relocation always carries a corrective position
in its new epoch.

The client treats each correction as a one-shot sample:

- snap when error is large;
- apply part of a small error;
- consume the target immediately.

It must not keep an old server position as a target every frame. Doing so makes
the target fight local gravity and causes vertical camera jitter.

Look rotation corrections are smoothed.

## Movement while overlays are open

Planar input and jump controls run only in `InGameOverlayState::Playing`.

Core movement and server/network updates run in `GameState::InGame`. Opening an
inventory does not pause the server or unload the player.

The local controller clears planar velocity on leaving `Playing`, while forces
such as gravity can continue.

## Replacing movement policy

Possible independent replacements:

- input backend;
- planar controller;
- fixed-tick provider;
- acceleration and drag policy;
- movement modifier;
- gravity provider;
- collision service;
- server validator;
- correction strategy;
- movement send rate.

Avoid replacing the whole movement stack when only one stage changes.

## Security limit

The current server validator checks displacement, block collisions, normal
speed, flight capability, and the separate flight-speed limit. It does not yet
maintain a complete per-player velocity model or validate acceleration, exact
input timing, sprint state, or knockback.

A stronger server can add validators to `ServerPlayerMovementSet::Validate`
without changing the session transport.
