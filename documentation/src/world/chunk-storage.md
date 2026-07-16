# Chunk coordinates and storage

Chunks are cubic and vertically unbounded.

## Coordinate types

`voxel-math-api` defines:

```rust
pub const CHUNK_SIZE: i32 = 16;
pub const CHUNK_VOLUME: usize = 4096;

pub struct BlockPos { x: i32, y: i32, z: i32 }
pub struct ChunkPos { x: i32, y: i32, z: i32 }
pub struct LocalBlockPos { x: u8, y: u8, z: u8 }
```

World-to-chunk conversion uses Euclidean division:

```rust
chunk = coordinate.div_euclid(16)
local = coordinate.rem_euclid(16)
```

This is essential for negative coordinates. Ordinary truncating division would
map negative blocks to the wrong chunks.

Local indexing is:

```text
x + z * 16 + y * 16 * 16
```

## Chunk structure

A `Chunk` contains:

```text
Chunk
├── position: ChunkPos
└── section: ChunkSection
```

The current chunk contains one section because a chunk itself is `16x16x16`.

The API exposes:

- `filled`;
- `get`;
- `set`;
- `iter`;
- `uniform_block`;
- access to compact section data.

## Palette representation

`ChunkSection` stores:

```text
palette:     palette index -> BlockInstance
reverseMap:  BlockInstance -> palette index
entries:     packed palette indices
```

The palette stores complete `BlockInstance` values. Metadata is part of equality
and can create a distinct palette entry even when `BlockId` is the same.

On `set`:

1. look up the block instance in `reverse_map`;
2. add a palette entry if it is new;
3. increase bits per entry if required;
4. repack existing indices;
5. write the new palette index.

## Packed bit array

`PackedBitArray` supports entries crossing `u64` boundaries.

Bits required are:

```text
palette size 1 -> 0 bits
palette size 2 -> 1 bit
palette size 3-4 -> 2 bits
palette size 5-8 -> 3 bits
...
```

A zero-bit array has no data words. Every entry implicitly contains palette
index zero.

This makes a uniform air chunk or uniform stone chunk very small:

```text
palette = [one BlockInstance]
bits_per_entry = 0
data = []
```

The representation is used both in memory and in CBOR chunk payloads.

## Serialization

`ChunkSection` serializes only:

- palette;
- packed entries.

`reverse_map` is derived and rebuilt during deserialization.

Deserialization validates:

- non-empty palette;
- exactly 4096 logical entries;
- every packed index fits the palette.

Do not serialize the reverse map. It duplicates data and can become inconsistent.

## Uniform fast paths

`uniform_block()` returns a block instance when the palette has one entry.

The project uses this information in several places:

- terrain providers return uniform chunks without looping over 4096 cells;
- air chunks have no packed payload;
- receiving an air chunk does not invalidate neighbor meshes;
- air chunks create no render mesh;
- opaque chunks surrounded by uniform opaque neighbors create no mesh.

These optimizations preserve the generic chunk format. A custom provider does
not need a separate "empty chunk packet".

## Editing and palette growth

Editing a uniform chunk introduces a second palette entry and repacks from zero
to one bit per block. Further unique block instances may cause more repacks.

This is simple and correct for the demo. A high-edit workload may prefer:

- palette compaction after many removals;
- a minimum bits-per-entry policy;
- section-local copy-on-write;
- batch edits that repack once;
- specialized sparse representation before a density threshold.

Those optimizations belong in the chunk storage implementation, not in terrain
or networking mods.

## Why chunks carry position

The chunk payload includes `ChunkPos`. This lets:

- response packets be self-identifying;
- cache insertion verify position;
- providers generate from one request type;
- tests round-trip chunks independently.

A production protocol may also include revision or scope identity. The current
client resets its cache on dimension changes instead of keying the cache by
scope.
