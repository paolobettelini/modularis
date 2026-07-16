# Bevy application lifecycle

`bevy-mod` is intentionally neutral:

```rust
pub struct BevyMod {
    pub app: App,
}

pub fn init() -> Self {
    Self { app: App::new() }
}
```

It does not install window plugins, rendering, or a runner.

## Client app

`client-bevy-default-plugins-mod` installs Bevy `DefaultPlugins` with:

- nearest-neighbor image sampling;
- the selected log filter;
- a resizable `1280x720` primary window;
- the window title from `client-config-api`.

The client bootstrap depends on the essential selected APIs and integration
mods, then owns `bevy-mod` and runs the app.

The bootstrap does not spawn gameplay entities or choose lighting. Those
responsibilities belong to independent mods.

## Server app

`server-bevy-runner-mod` installs:

```rust
MinimalPlugins.set(
    ScheduleRunnerPlugin::run_loop(Duration::from_millis(16))
)
```

This provides a headless update loop without a window.

`server-game-bootstrap-mod` only owns and runs the final app. Server gameplay
systems register themselves before the bootstrap consumes the app.

## Client state

The main state is:

```rust
pub enum GameState {
    MainMenu,
    SettingsMenu,
    InGame,
}
```

`InGame` has a substate:

```rust
pub enum InGameOverlayState {
    Playing,
    PauseMenu,
    Settings,
    Inventory,
}
```

The overlay is a substate rather than a replacement for `InGame`. Therefore
opening the pause menu or inventory does not destroy:

- the player entity;
- chunk cache and meshes;
- network connection;
- server synchronization;
- gravity and world updates.

Input-specific systems may run only in `Playing`, while simulation and network
systems continue in `InGame`.

## State commands

UI and input mods write commands:

```rust
GameStateCommand::StartGame
InGameOverlayCommand::Pause
InGameOverlayCommand::OpenInventory
```

`client-game-state-bevy-impl` is the only mod that converts these commands into
`NextState`.

This separates menu widgets and keybindings from concrete state mutation.

## Entity lifetime

Many client entities use:

```rust
DespawnOnExit(GameState::InGame)
```

Examples:

- local player;
- camera;
- crosshair;
- chunk entities;
- Blocky player models;
- sun light and sun disc.

Subsystems with additional indices or caches also clear their resources in
`OnExit`. Despawning an entity without clearing a resource map would leave stale
entity IDs.

## Startup ordering

Some registries must be populated before menus are built. Settings input uses:

```text
SettingInputStartupSet::RegisterInputs
    -> SettingInputStartupSet::BuildMenus
```

General rule: if initialization order matters at runtime, expose a public
`SystemSet` order. Do not depend on incidental plugin insertion order.

## Adding a client startup feature

Choose the narrowest lifecycle:

- `Startup`: application-wide immutable registration;
- `OnEnter(GameState::InGame)`: spawn world-specific entities;
- `Update.run_if(in_state(...))`: active behavior;
- `OnExit`: clean up entities and indexed state.

If a feature owns no world entity, it may only need to initialize a resource.

## Adding a server startup feature

Server features usually:

- register resources in `init`;
- add systems to public server pipelines;
- listen to `ServerPlayerJoined` if they need per-player state;
- remove that state on `ServerPlayerLeft`;
- avoid placing behavior directly in the bootstrap.

The bootstrap should remain stable as server feature packs change.
