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
  -> Forces
  -> ForceOverrides
  -> Movement
  -> CameraSync
```

### Input

The FPS controller projects camera forward onto the plane perpendicular to
gravity and builds `PlayerPlanarMovementIntent`.

The intent contains:

```rust
direction: Vec3
speed_multiplier: f32
```

It does not directly contain sprint or status-effect knowledge.

### Movement modifiers

Optional mods multiply or modify the intent. Sprint currently applies a `1.55`
speed multiplier while the configured key is held.

Other mods can implement:

- slow effects;
- terrain speed;
- equipment bonuses;
- temporary knockback restrictions.

### Apply intent

The controller preserves the velocity component along gravity-up and replaces
the planar component with configured walking velocity.

### Forces and overrides

Gravity and jump run in `Forces`.

Flight runs in `ForceOverrides`, allowing it to replace the velocity component
along gravity-up after ordinary forces without hardcoding flight into gravity.

### Movement

The collision service resolves the frame's movement and returns:

- resolved position;
- hit X/Y/Z flags;
- grounded information.

Blocked velocity components are cleared.

### Camera sync

The camera follows:

```text
player feet + gravity up * eye height
```

It follows the locally predicted player, not the last server packet.

## Collision service

`collision-api` exposes a closure-backed `CollisionService`.

The active implementation reads blocks from `ClientChunkCache` and uses shared
player/block AABB helpers.

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
- movement modifier;
- gravity provider;
- collision service;
- server validator;
- correction strategy;
- movement send rate.

Avoid replacing the whole movement stack when only one stage changes.

## Security limit

The current server validator checks displacement and block collisions. It does
not yet maintain a complete per-player velocity model or validate acceleration,
flight capability, timing, or knockback.

A stronger server can add validators to `ServerPlayerMovementSet::Validate`
without changing the session transport.
