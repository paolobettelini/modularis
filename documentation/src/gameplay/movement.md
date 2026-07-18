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
- hit X/Y/Z flags;
- grounded information.

Blocked velocity components are cleared.

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

## Collision service

`collision-api` exposes a closure-backed `CollisionService`.

Besides full overlap and movement resolution, the service exposes a support
probe used by grounded and sneak policies. A collision provider can install a
specialized support query with `with_support_query`; otherwise the service
falls back to a tiny movement resolution. The block-AABB implementation scans
only the leading face of the hitbox, which avoids repeatedly traversing the
whole volume of very large scaled players.

The active implementation reads block instances from `ClientChunkCache`, asks
`BlockShapeService` for each solid block's local AABB union, and resolves the
player against the translated boxes. The server validator uses the same shape
contract against its authoritative world route.

The resolver:

1. depenetrates an already overlapping player;
2. resolves the height axis first;
3. resolves X;
4. resolves Z;
5. applies a small skin distance;
6. falls back to binary search for difficult contacts.

Resolving height first lets a rising player clear a ledge before planar motion
is tested. This avoids alternating vertical/side corrections when jumping onto
a block.

Partial blocks use their actual top and side coordinates. Standing on a lower
slab-like element therefore grounds the player near `y + 0.5`, not `y + 1.0`.
The collision algorithm is independent from JSON: replacing
`BlockShapeService` replaces the geometry source without changing movement.

## Grounded state

Grounding uses two sources:

- a real blocked movement into the gravity direction;
- a very small contact probe that preserves an existing stable grounded state.

The probe cannot turn an arbitrary airborne player into grounded. It also does
not keep a player grounded while velocity moves away from the surface.

This distinction prevents repeated tiny jumps caused by a probe cancelling a
real upward or falling velocity.

## Movement network flow

The client sends a `PlayerMove` about every `0.05` seconds:

```text
position
yaw
pitch
```

The server collects:

```rust
PendingServerPlayerMove {
    source,
    player_id,
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

The apply stage updates the registry and synchronizes visible remote players.

## Corrections

The server sends the local player a correction only when rejected or when the
accepted position differs from requested position by more than `0.15`.

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
