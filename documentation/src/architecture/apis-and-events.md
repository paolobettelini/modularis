# APIs, providers, and ECS contracts

The project uses three related extension mechanisms:

1. compile-time API providers;
2. runtime service resources;
3. ECS messages and ordered system sets.

They solve different problems and are often used together.

## Marker API traits

Many API crates expose a marker trait:

```rust
pub trait ServerChunkWorldApi: Send + Sync + 'static {}
```

A provider implements it:

```rust
impl ServerChunkWorldApi for ServerChunkWorldDynamicImpl {}
```

Consumers use the generic bound in `init`:

```rust
pub fn init<W: ServerChunkWorldApi>(
    bevy: &mut BevyMod,
    _world: &mut W,
) -> Self
```

This gives Patchwork a compile-time edge and allows the modpack to select the
implementation.

## Runtime service resources

The actual runtime service is often type-erased:

```rust
#[derive(Resource, Clone)]
pub struct ServerChunkWorld(Arc<dyn ServerChunkWorldBackend>);
```

or closure-backed:

```rust
#[derive(Resource, Clone)]
pub struct ServerChunkRouter {
    route: Arc<
        dyn Fn(ChunkViewer, ChunkPos) -> Option<ServerChunkRoute>
            + Send
            + Sync
    >,
}
```

This pattern has two advantages:

- systems do not need generic parameters;
- a provider can install any backend that satisfies the runtime contract.

Use it when systems need to call a service repeatedly.

## Registries for multiple providers

Some domains are not singletons. The server may have multiple chunk providers
at the same time:

```rust
ServerChunkProviderRegistry
    ChunkProviderId -> Arc<dyn ServerChunkProvider>
```

The router chooses one provider per request. This is better than declaring the
whole chunk-provider API as an exclusive singleton because dimensions need
Overworld, Nether, and Aether providers simultaneously.

General rule:

- use a singleton provider for one selected implementation;
- use a keyed registry when several implementations must coexist.

## Messages as intentions and results

Messages should express domain meaning, not implementation callbacks.

Inventory uses:

```text
LocalInventoryMoveIntent
InventoryMoveRequested
InventoryMoveHandled
InventoryCellSet
ClientInventoryCellSet
```

These names show where the message sits:

- `Local*Intent`: local client input;
- `*Requested`: authoritative operation request;
- `*Handled`: a specialized feature claimed the operation;
- domain result without `Client`: server-side applied result;
- `Client*`: client-side synchronization contract.

This vocabulary is not perfectly universal, but following it makes pipelines
easier to read.

## Ordered `SystemSet` pipelines

Public `SystemSet` values are one of the main extension tools.

The block edit pipeline is:

```text
Receive -> Collect -> Validate -> Apply -> Sync
```

The inventory pipeline is:

```text
ReceiveRequest
  -> Validate
  -> DispatchUse
  -> ApplyWorldEffects
  -> ApplyConsumption
  -> Sync
```

The player movement pipeline is:

```text
Receive -> Validate -> Apply -> Sync
```

The client controller pipeline is:

```text
Input
  -> MovementModifiers
  -> ApplyMovementIntent
  -> Forces
  -> ForceOverrides
  -> Movement
  -> CameraSync
```

A new mod can insert a system into the correct stage without editing the
existing implementation.

## Claim-and-fallback behavior

Some interactions have several possible handlers. Right-clicking a block may
open a crafting table, activate another block-specific feature, or fall back to
using the held item.

The client block-use pipeline uses:

```text
Raycast -> SpecificHandlers -> Fallback
```

Every operation has an ID. A specific handler emits
`LocalBlockUseHandled { operation_id }`. The fallback collects handled IDs and
forwards only unclaimed operations to held-item use.

This pattern is useful when:

- several independent mods may recognize the same input;
- one handler should stop fallback behavior;
- handlers should not directly depend on one another.

## Mutable pending requests

Validation pipelines often store mutable pending entries in a resource:

```rust
pub struct PendingBlockBreak {
    pub player_id: u64,
    pub position: BlockPos,
    pub allowed: bool,
}
```

A validator can deny or modify an operation before the apply stage.

Use this shape for:

- permissions;
- reach checks;
- protected regions;
- movement clamping;
- rate limits;
- custom server rules.

For more complex validation, prefer a richer decision type over many unrelated
boolean fields.

## Append-only stage registries

The chunk vertex lighting pipeline allows several mods to register
multiplicative stages:

```rust
pipeline.register_face_stage(face_brightness);
pipeline.register_ambient_occlusion_stage(ao_brightness);
```

The mesher takes one snapshot per chunk and applies all selected stages. This
supports additive composition when selecting more than one provider is useful.

Use an append-only pipeline when behaviors should combine. Use an exclusive API
provider when one behavior should replace another.

## Event contract design rules

A good cross-mod event:

- includes stable IDs rather than entity references when crossing the network;
- carries the actor when permissions or attribution matter;
- carries the scope when the same coordinates can exist in several worlds;
- carries the previous and new value when reaction logic needs both;
- does not expose a concrete implementation's private cache type;
- is emitted only after the operation stage its name claims.

For example, `ServerBlockPlaced` includes player, scope, position, new block,
and replaced block. This is enough for synchronization, logging, achievements,
or custom reactions without querying hidden state.
