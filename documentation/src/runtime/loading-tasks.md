# Loading tasks and progress screens

Loading progress is a small domain of its own. A networking mod, an asset
loader, a world bootstrapper, or a custom minigame can expose work without
depending on the concrete Bevy screen that presents it.

The neutral `loading-task-api` library defines a task with:

- a stable namespaced ID;
- a user-facing label;
- normalized progress from `0.0` to `1.0`;
- a status string describing the current operation;
- a `blocking` flag.

It does not depend on client UI, transport, sessions, or server policy.

## Client-owned tasks

`client-loading-api` exposes ECS messages for setting, removing, and clearing
tasks. Tasks are keyed by both authority and ID. The current authorities are
`Local` and `Server`, so a local asset loader and the connected server can use
the same task ID without overwriting each other.

```rust
commands.write(SetClientLoadingTask {
    authority: ClientLoadingAuthority::Local,
    task: LoadingTask::new(
        "example:models",
        "Models",
        0.4,
        "Loading character meshes",
    ),
});
```

`client-loading-state-mod` is the generic state provider.
`client-loading-ui-bevy-mod` is only one presentation implementation: it draws
one bar for every active task and a full-screen blocking background when at
least one task is blocking. A different client can replace the UI without
changing producers.

The ordered client pipeline is:

```text
Receive -> Apply -> React -> Render
```

## Server-owned tasks

`server-loading-api` stores tasks per player. Server systems publish
`SetServerPlayerLoadingTask` and `RemoveServerPlayerLoadingTask`; they do not
construct network packets directly. `server-loading-state-mod` applies these
intentions and emits `ServerPlayerLoadingTaskChanged`.

`server-loading-network-sync-mod` is a separate adapter from that generic ECS
event to the loading protocol. It also sends the current task snapshot when a
player becomes ready. Persistent server tasks are removed when the player
leaves.

```text
server policy or loader
  -> SetServerPlayerLoadingTask
  -> server loading state
  -> ServerPlayerLoadingTaskChanged
  -> network sync adapter
  -> client loading state
  -> selected client UI
```

Persistence, replication, and rendering are therefore separate concerns. A
headless consumer may use the same server API without selecting the Bevy UI,
and a local-only task requires no server support.

## Join loading policy

The demo composes two small policies:

- `client-join-loading-mod` reports connection, authentication/join acceptance,
  and the first received chunk as a local task;
- `server-join-loading-vanilla-mod` reports player registration, readiness, and
  the first streamed chunk as a server-authoritative task.

First-world-data completion is terminal for the current player session. The
task-removal timer is separate from the completed-player set, so later chunk
streaming cannot recreate the join task while the player moves through the
world. Both pieces of state are cleared only when that session leaves.

The client policy treats packet events as notifications, not as its only source
of truth. It also observes the authoritative `ClientSession` and
`ClientChunkCache` resources. This matters during a server transfer: a state
transition must not leave the task at 25% merely because `JoinAccepted` or the
first chunk event was emitted between two schedules. The network chunk cache is
cleared when leaving `GameState::InGame`, so data from the source server cannot
complete the destination server's loader accidentally.

Transport connection and disconnection events carry a monotonic client-side
connection ID. Cleanup systems compare this ID with their active connection.
During an auth-to-game transfer, the delayed disconnect event from the entry
server therefore cannot cancel the destination server's authentication
handshake or join loader.

These percentages describe the current vanilla join process. They are not
embedded in either state provider. A custom server can omit the vanilla policy,
publish more detailed tasks, or use runtime scopes to expose different loading
work to different players.

## Adding a loader

Choose ownership first:

1. emit client task messages when progress is known only by the client;
2. emit server per-player task messages when the server owns the work;
3. use a stable namespaced task ID owned by the producing feature;
4. update the same task as work advances;
5. remove it when complete or cancelled.

Do not make a loader depend on `client-loading-ui-bevy-mod`, and do not put
feature-specific progress rules in `client-loading-state-mod` or
`server-loading-state-mod`.
