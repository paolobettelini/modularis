# Extension cookbook

This chapter gives concrete patterns for common extensions.

Paths and names are examples. Use a unique namespace instead of `example`.

## Create a small Bevy feature mod

Manifest:

```toml
[package]
name = "server-welcome-effect-mod"
version = "0.1.0"
edition = "2024"

[package.metadata.mod]
entry = "ServerWelcomeEffectMod"

[package.metadata.mod.dependencies]
init = ["bevy-mod", "server-player-lifecycle-events-mod"]
run = []
ownership = []

[dependencies]
bevy = "0.17.3"
bevy-mod = { path = "../bevy-mod" }
server-player-lifecycle-events-api = {
    path = "../server-player-lifecycle-events-api"
}
server-player-lifecycle-events-mod = {
    path = "../server-player-lifecycle-events-mod"
}
tokio = { version = "1.48.0", features = ["full"] }
```

Entry:

```rust
use bevy::prelude::*;
use bevy_mod::BevyMod;
use server_player_lifecycle_events_api::ServerPlayerJoined;
use server_player_lifecycle_events_mod::ServerPlayerLifecycleEventsMod;
use tokio::task::JoinHandle;

pub struct ServerWelcomeEffectMod;

impl ServerWelcomeEffectMod {
    pub fn init(
        bevy: &mut BevyMod,
        _lifecycle: &mut ServerPlayerLifecycleEventsMod,
    ) -> Self {
        bevy.app.add_systems(Update, welcome);
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}

fn welcome(mut joined: MessageReader<ServerPlayerJoined>) {
    for player in joined.read() {
        info!("player {} joined", player.player_id);
    }
}
```

Add it to the intended feature modpack, not necessarily `server-base.toml`.

## Create an exclusive API provider

API crate:

```rust
pub trait WeatherApi: Send + Sync + 'static {}

#[derive(Resource, Clone)]
pub struct WeatherService {
    pub sample: Arc<dyn Fn(Vec3) -> Weather + Send + Sync>,
}
```

Provider manifest:

```toml
[package.metadata.mod]
entry = "ClearWeatherProvider"
provides = "weather-api"
```

Provider:

```rust
impl ClearWeatherProvider {
    pub fn init(bevy: &mut BevyMod) -> Self {
        bevy.app.insert_resource(WeatherService {
            sample: Arc::new(|_| Weather::Clear),
        });
        Self
    }
}

impl WeatherApi for ClearWeatherProvider {}
```

Consumers take `W: WeatherApi` in `init` and read `WeatherService` in systems.

## Add a block

Create:

```text
mods/block-marble/
├── Cargo.toml
├── src/lib.rs
└── assets/
    ├── models/block/marble.json
    └── textures/block/marble.png
```

Cargo metadata:

```toml
[package.metadata.block]
id = "example:marble"

[package.metadata.mod.dependencies]
init = ["voxel-model-block-templates-mod"]
```

Export `BLOCK_INFO` and `RENDER_INFO` as shown in
[Blocks, items, and metadata](../world/blocks-and-items.md).

Add `"block-marble"` to `blocks.toml`, recompose both applications, and use the
generated `BlockId::Marble`. The contributor points at
`block-marble:block/marble` through `BlockRenderInfo`.

The model can reuse the generic cube template:

```json
{
  "parent": "voxel-model-block-templates-mod:block/cube_all",
  "textures": {
    "all": "block-marble:block/marble"
  }
}
```

For six different textures, inherit
`voxel-model-block-templates-mod:block/cube` and provide `east`, `west`, `up`,
`down`, `south`, and `north`. For custom geometry, provide `elements` or create
a reusable asset-only template mod. See
[JSON voxel models and textures](../world/voxel-models.md).

## Add an item

Contributor metadata:

```toml
[package.metadata.item]
id = "example:wand"
```

Export:

```rust
impl Item for WandItem {
    const INFO: ItemInfo = ItemInfo {
        id: "example:wand",
        label: "Wand",
    };
}

impl ItemRender for WandItem {
    const RENDER: ItemRenderInfo = ItemRenderInfo {
        model: Some("item-wand:item/wand"),
    };
}

pub const ITEM_INFO: ItemInfo = WandItem::INFO;
pub const ITEM_RENDER_INFO: ItemRenderInfo =
    <WandItem as ItemRender>::RENDER;
```

Add its JSON under `assets/models/item/wand.json`, its texture under
`assets/textures/item/wand.png`, declare the item template mod as a formal
dependency, add it to `items.toml`, and recompose.

The contributor defines identity/label. Item-use behavior belongs in another
mod listening to `HeldItemUseDispatched`.

## Add item metadata

Manifest:

```toml
[package.metadata.item_metadata]
id = "example:spell"
field = "spell"
type = "item-spell-meta::Spell"
```

Type:

```rust
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize
)]
pub struct Spell {
    pub id: String,
}
```

Add the metadata contributor to `items.toml`.

Instantiate:

```rust
ItemMetaSet {
    spell: Some(Spell {
        id: "example:blink".into(),
    }),
    ..Default::default()
}
```

Implement spell semantics in one or more separate feature mods.

## Add block metadata

Use `package.metadata.block_metadata` with stable `id`, generated field name,
and type path.

Ensure the metadata type derives:

- clone;
- equality;
- hash;
- serialization/deserialization.

Block metadata is used as a chunk palette key, so equality and hash semantics
must be stable.

## Add a network packet

Create a message type crate:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerManaChanged {
    pub current: u32,
    pub maximum: u32,
}
```

Create a contributor mod:

```toml
[package.metadata.network.messages]
clientbound = [
    "player-mana-network-message-types::PlayerManaChanged"
]
serverbound = []
```

Add the contributor to `network.toml`.

After recomposition, codegen provides:

- `ClientBoundMessage::PlayerManaChanged`;
- `PlayerManaChangedReceived`;
- event registration and dispatch.

Server sync:

```rust
packets.write(ServerPacketOut {
    audience: ServerAudience::Player(player_id),
    message: ClientBoundMessage::PlayerManaChanged(
        PlayerManaChanged { current, maximum },
    ),
});
```

Client receive:

```rust
fn receive(
    mut packets: MessageReader<PlayerManaChangedReceived>,
) {
    // Update client cache/resource.
}
```

Do not add the type directly to `network-protocol-mod`.

## Add a server block validator

Depend on block edit events and add a system to:

```text
ServerBlockEditSet::Validate
```

Modify pending requests, or add a richer shared validation result if required.

Do not decode packets again and do not mutate the world in validation.

## Add an item-use effect

Listen in:

```text
InventoryServerSet::ApplyWorldEffects
```

Check your metadata and target:

```rust
fn use_wand(
    mut uses: MessageReader<HeldItemUseDispatched>,
    mut success: MessageWriter<ItemUseSucceeded>,
) {
    for item_use in uses.read() {
        let Some(spell) = &item_use.item.metadata.spell else {
            continue;
        };

        if apply_spell(spell, &item_use.target) {
            success.write(ItemUseSucceeded {
                player_id: item_use.player_id,
                cell: item_use.cell.clone(),
                item_before_use: item_use.item.clone(),
            });
        }
    }
}
```

Quantity consumption remains reusable.

If more than one world effect may consume the same use, add an explicit claim
or result contract to avoid duplicate success.

## Add a setting

Contributor:

```toml
[package.metadata.setting]
id = "graphics.fog_distance"
label = "Fog distance"
type = "f32"
input = "f32"
default = 64.0
```

Add it to `client.toml`, recompose, and read the generated key.

For a new editor, create and select a setting input provider mod.

## Add a chunk provider

Implement `ServerChunkProvider`, register a unique `ChunkProviderId`, then
select it through routing or a dimension definition.

Provider generation must:

- return a chunk at `request.position`;
- use complete `BlockInstance` values;
- avoid blocking operations;
- return uniform chunks early when possible;
- remain deterministic unless mutable source state is intentional.

If it depends on viewer identity, document whether edits are shared or
viewer-specific. Routing and instance keys must match that policy.

## Add a biome

Create a small identity contributor:

```toml
[package.metadata.biome]
id = "example:crystal-fields"
```

Add it to the appropriate `biomes-<dimension>.toml` pack and recompose to
generate `BiomeId::CrystalFields`.

Create a separate server definition mod that registers `BiomeDefinition` in
`ServerBiomeRegistry`. Set its generated `Dimension`, climate targets, terrain
blocks, visual data, and ordered feature IDs in that Rust definition.

If the biome needs new generation behavior, create a feature mod implementing
`ServerBiomeFeature`, register a namespaced `BiomeFeatureId`, and give it a
public phase plus a conservative vertical range. Make the definition mod depend
on the feature mod so registration happens first.

The vanilla selector automatically considers definitions for the provider's
dimension. Replace `server-biome-selection-api` for another placement policy.
Replace one dimension's chunk provider when its desired geometry does not fit
the current heightmap, Nether ground, or floating-island model. A replacement
provider may reuse `server-biome-sampling-api` without inheriting any of those
geometry rules.

The complete contract and examples are in
[Biomes and world-generation features](../world/biomes.md).

## Add a dimension

Create:

1. dimension contributor;
2. terrain provider or provider selection;
3. server registration mod;
4. travel behavior;
5. packet-compatible client/server builds.

Do not add special cases to `ServerChunkWorld`.

## Add a cell-menu feature

Client specific handler:

- recognize the block/action;
- send generic open request;
- mark local block use handled.

Server validator:

- recognize `CellMenuOpenIntent.kind`;
- validate actor, anchor, scope, and permissions;
- build stable menu ID and audience;
- emit `CellMenuOpenRequested`.

Optional semantics:

- persistence;
- recipe processing;
- periodic machine updates;
- damage/effects on move.

Keep each semantic in a separate mod.

## Add a graphics stage

For composable brightness:

```rust
pipeline.register_face_stage(my_face_stage);
pipeline.register_ambient_occlusion_stage(my_ao_stage);
```

For a replacement mesher or renderer, implement the exclusive API instead.

## Replace player assets

Provide `client-player-blocky-model-paths-api`:

```rust
ClientPlayerBlockyModelPaths {
    model_path: "my-player-assets/player.blockymodel",
    texture_path: Some("my-player-assets/player.png"),
    texture_size: Some(UVec2::new(256, 128)),
    idle_animation_path: "my-player-assets/idle.blockyanim",
    walk_animation_path: Some(
        "my-player-assets/walk.blockyanim"
    ),
    model_scale: 1.0,
    primitive_scale: 1.0 / 64.0,
    yaw_offset_radians: 0.0,
    vertical_animation_locked_nodes: &["Pelvis"],
}
```

Own all files in that mod's `assets/` directory and select only one provider.

## Build a custom server profile

Start from the smallest useful composition:

```toml
name = "Custom server"
description = "A server with selected rules."
modpacks = ["server-core"]
ignore = []

mods = [
    # Add only neutral feature pipelines that this server uses.
    "server-chat-events-mod",
    "server-chat-network-receive-mod",
    "server-chat-network-sync-mod",
    "server-chunk-provider-registry-mod",
    "my-chunk-provider-mod",
    "my-chunk-routing-mod",
    "server-chunk-world-dynamic-impl",
    "my-player-visibility-mod",
]
```

Add only desired neutral pipelines and vanilla feature mods. Import
`server-base.toml` instead when its complete convenience bundle is useful. Do
not import `server.toml`, because it already chooses the demo world and vanilla
pack.

Compose early. Missing API errors reveal which neutral contracts still need a
provider.
