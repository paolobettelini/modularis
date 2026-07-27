# Sounds, audiences, and spatial playback

Sounds use the same distributed ownership model as blocks, items, dimensions,
and network messages. A sound mod owns its ID, runtime definition, and assets.
The final modpack determines which variants exist in the generated `SoundId`
enum.

## Sound contributors

A contributor declares only its stable namespaced ID in Cargo metadata:

```toml
[package.metadata.sound]
id = "demo:note-block-bass"
```

Its Rust code owns the actual definition:

```rust
pub const SOUND_INFO: SoundInfo = SoundInfo {
    id: "demo:note-block-bass",
    asset_path: "sound-note-block-bass/note_block_bass.mp3",
};
```

The asset belongs to the same mod:

```text
mods/sound-note-block-bass/
├── Cargo.toml
├── assets/
│   └── note_block_bass.mp3
└── src/lib.rs
```

Patchwork copies it to
`assets/sound-note-block-bass/note_block_bass.mp3` in the composed project.

`sound-registry-codegen` scans the selected contributors and generates:

- `SoundId`;
- `ALL_SOUNDS`;
- string conversion;
- lookup of each contributor's `SoundInfo`;
- asset-path lookup.

Adding a sound therefore does not require editing a central enum.

## Server contract

Server gameplay does not depend on Bevy audio or on a concrete transport. It
publishes `PlayServerSound`:

```rust
sounds.write(PlayServerSound {
    audience: Audience::personal(player_id),
    playback: SoundPlayback::new(SoundId::NoteBlockBass)
        .with_volume(1.0)
        .with_pitch(1.0)
        .at([x, y, z]),
});
```

The request contains:

- an `Audience`, which decides who receives the sound;
- a generated `SoundId`;
- volume;
- pitch;
- an optional world-space emitter position.

When the position is `None`, the client uses non-spatial playback. When it is
`Some`, the client hears a spatial emitter at that coordinate. Audience and
position are independent: a sound can be sent to one player, a scoped group,
or everyone while still originating from one world position.

The publisher should run in `ServerSoundSet::Publish`. The generic network
bridge runs later in `ServerSoundSet::Sync`, resolves the audience, validates
the numeric values, and sends `PlaySoundPacket` only to the selected players.

## Client contract and Bevy backend

`client-sound-network-receive-mod` converts the packet into the local
`PlayClientSound` ECS message. It does not play audio directly.

`client-sound-bevy-audio-impl` is the selected backend. It:

- enables MP3 support in Bevy;
- preloads every selected sound through `AssetServer`;
- attaches one `SpatialListener` to `PlayerCamera`;
- spawns one-shot `AudioPlayer` entities;
- despawns each audio entity after playback.

Another client can replace this backend while keeping the generated IDs,
network packets, and gameplay publishers unchanged.

## TheCrown example

The parkour rules remain inside `parkour-gameplay-lib` and know nothing about
audio. TheCrown's orchestration mod reacts to a successful checkpoint and
publishes the bass note for that player only:

```rust
let pitch = 0.9 + (update.combo - 1) as f32 * 0.05;
```

The emitter position is the player's accepted movement position. A fall reset
does not play the checkpoint sound.

## Adding another sound

1. Create a small sound contributor crate.
2. Add `[package.metadata.sound]` with a unique namespaced ID.
3. Export `SOUND_INFO` from its Rust code.
4. Put the audio file in that mod's `assets/` directory.
5. Add the contributor to a sound modpack.
6. Recompose client and server.
7. Use the new generated `SoundId` from gameplay glue.

Keep selection policy outside the sound definition. A sound crate describes an
asset; gameplay mods decide when, where, and for whom it plays.
