use clap::Parser;
use fastanvil::{Chunk as MinecraftChunk, JavaChunk, Region};
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    fs,
    io::Read,
    path::{Path, PathBuf},
};

const OUTPUT_SOURCE: &str = "patchwork:primary";
const FORMAT_VERSION: u16 = 2;
const CHUNK_VOLUME: usize = 16 * 16 * 16;
const REGION_EDGE_CHUNKS: i32 = 8;
const EMPTY_BLOCK_STATE_CBOR: &[u8] = &[0xa0];

#[derive(Debug, Parser)]
#[command(
    name = "thecrown-minecraft-world-converter",
    about = "Converts a Minecraft Java world to the Patchwork world format"
)]
struct Args {
    /// Path to the Minecraft Java world.
    #[arg(short, long, value_name = "PATH")]
    input: PathBuf,

    /// Path where the converted world will be written.
    #[arg(short, long, value_name = "PATH")]
    output: PathBuf,

    /// Path containing the block-* mod crates.
    #[arg(short, long, value_name = "PATH")]
    mods: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ChunkPos {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct RegionPos {
    x: i32,
    y: i32,
    z: i32,
}

impl RegionPos {
    fn from_chunk(chunk: ChunkPos) -> Self {
        Self {
            x: chunk.x.div_euclid(REGION_EDGE_CHUNKS),
            y: chunk.y.div_euclid(REGION_EDGE_CHUNKS),
            z: chunk.z.div_euclid(REGION_EDGE_CHUNKS),
        }
    }
}

struct NativeChunk {
    blocks: Vec<u32>,
}

impl NativeChunk {
    fn empty(air: u32) -> Self {
        Self {
            blocks: vec![air; CHUNK_VOLUME],
        }
    }

    fn set(&mut self, x: usize, y: usize, z: usize, block: u32) {
        self.blocks[x + z * 16 + y * 16 * 16] = block;
    }
}

#[derive(Debug, Deserialize)]
struct LevelRoot {
    #[serde(rename = "Data")]
    data: LevelData,
}

#[derive(Debug, Deserialize)]
struct LevelData {
    #[serde(rename = "RandomSeed", default)]
    legacy_seed: Option<i64>,

    #[serde(rename = "WorldGenSettings", default)]
    world_gen: Option<WorldGenSettings>,

    #[serde(rename = "SpawnX", default)]
    spawn_x: i32,

    #[serde(rename = "SpawnY", default)]
    spawn_y: i32,

    #[serde(rename = "SpawnZ", default)]
    spawn_z: i32,
}

#[derive(Debug, Deserialize)]
struct WorldGenSettings {
    seed: i64,
}

#[derive(Debug, Serialize)]
struct WorldInfo {
    seed: u64,
}

#[derive(Debug, Deserialize)]
struct MinecraftPlayerData {
    #[serde(rename = "Pos")]
    position: Vec<f64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input = args.input;
    let output = args.output;
    let mods = args.mods;

    validate_input_world(&input)?;

    if output.exists() {
        return Err(format!(
            "output '{}' already exists; remove or move it explicitly before converting again",
            output.display(),
        )
        .into());
    }

    let block_ids = discover_block_ids(&mods)?;
    let block_indices = block_ids
        .iter()
        .enumerate()
        .map(|(index, id)| (id.clone(), index as u32))
        .collect::<HashMap<_, _>>();

    let air = required_block(&block_indices, "demo:air")?;
    let stone = required_block(&block_indices, "demo:stone")?;

    let level = read_level(&input.join("level.dat"))?;

    let seed = match level
        .data
        .world_gen
        .as_ref()
        .map(|settings| settings.seed)
        .or(level.data.legacy_seed)
    {
        Some(seed) => seed as u64,

        None => {
            eprintln!(
                "Warning: '{}' contains neither WorldGenSettings.seed nor RandomSeed; using seed 0",
                input.join("level.dat").display()
            );

            0
        }
    };

    println!(
        "Minecraft source: {} (seed {seed}, spawn {}, {}, {})",
        input.display(),
        level.data.spawn_x,
        level.data.spawn_y,
        level.data.spawn_z,
    );

    print_saved_player_positions(&input);

    let region_files = discover_overworld_region_files(&input)?;

    let mut chunks = BTreeMap::<ChunkPos, NativeChunk>::new();
    let mut mappings = HashMap::<String, u32>::new();
    let mut source_counts = HashMap::<String, u64>::new();
    let mut bounds: Option<([i32; 3], [i32; 3])> = None;

    for path in region_files {
        if fs::metadata(&path)?.len() == 0 {
            eprintln!("Skipping empty region file {}", path.display());
            continue;
        }

        let (region_x, region_z) = parse_region_coordinates(&path)?;

        println!("Reading {}", path.display());

        let mut region = Region::from_stream(fs::File::open(&path)?)?;

        for source_chunk in region.iter() {
            let source_chunk = source_chunk?;
            let minecraft = JavaChunk::from_bytes(&source_chunk.data)?;

            let chunk_x = region_x * 32 + source_chunk.x as i32;
            let chunk_z = region_z * 32 + source_chunk.z as i32;

            for y in minecraft.y_range() {
                for z in 0..16 {
                    for x in 0..16 {
                        let Some(source) = minecraft.block(x, y, z) else {
                            continue;
                        };

                        let source_name = source.name();

                        *source_counts
                            .entry(source_name.to_owned())
                            .or_default() += 1;

                        let block = *mappings
                            .entry(source_name.to_owned())
                            .or_insert_with(|| {
                                map_block(source_name, &block_indices, stone)
                            });

                        if block == air {
                            continue;
                        }

                        let world_position = [
                            chunk_x * 16 + x as i32,
                            y as i32,
                            chunk_z * 16 + z as i32,
                        ];

                        match &mut bounds {
                            Some((minimum, maximum)) => {
                                for axis in 0..3 {
                                    minimum[axis] =
                                        minimum[axis].min(world_position[axis]);

                                    maximum[axis] =
                                        maximum[axis].max(world_position[axis]);
                                }
                            }

                            None => {
                                bounds = Some((world_position, world_position));
                            }
                        }

                        let position = ChunkPos {
                            x: chunk_x,
                            y: (y as i32).div_euclid(16),
                            z: chunk_z,
                        };

                        chunks
                            .entry(position)
                            .or_insert_with(|| NativeChunk::empty(air))
                            .set(
                                x,
                                (y as i32).rem_euclid(16) as usize,
                                z,
                                block,
                            );
                    }
                }
            }
        }
    }

    write_world(&output, seed, &block_ids, chunks)?;

    if let Some((minimum, maximum)) = bounds {
        println!(
            "Non-air block bounds: {minimum:?} .. {maximum:?}"
        );
    }

    let mut unsupported = mappings
        .iter()
        .filter(|(name, block)| {
            **block == stone && name.as_str() != "minecraft:stone"
        })
        .map(|(name, _)| {
            (
                name,
                source_counts
                    .get(name)
                    .copied()
                    .unwrap_or_default(),
            )
        })
        .collect::<Vec<_>>();

    unsupported.sort_by(|left, right| right.1.cmp(&left.1));

    println!(
        "Converted world written to {}",
        output.display()
    );

    println!(
        "Unsupported block kinds mapped to stone: {}",
        unsupported.len()
    );

    for (name, count) in unsupported.iter().take(30) {
        println!("  {name}: {count}");
    }

    Ok(())
}

fn validate_input_world(
    input: &Path,
) -> Result<(), Box<dyn Error>> {
    if !input.exists() {
        return Err(format!(
            "input world '{}' does not exist",
            input.display()
        )
        .into());
    }

    if !input.is_dir() {
        return Err(format!(
            "input world '{}' is not a directory",
            input.display()
        )
        .into());
    }

    let level_dat = input.join("level.dat");

    if !level_dat.is_file() {
        return Err(format!(
            "input '{}' does not look like a Minecraft world: '{}' is missing",
            input.display(),
            level_dat.display()
        )
        .into());
    }

    Ok(())
}

fn discover_overworld_region_files(
    input: &Path,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let candidates = [
        (
            "classic/vanilla overworld layout",
            input.join("region"),
        ),
        (
            "dimension-based overworld layout",
            input.join(
                "dimensions/minecraft/overworld/region"
            ),
        ),
    ];

    let mut checked = Vec::new();

    for (description, directory) in &candidates {
        checked.push(directory.clone());

        if !directory.is_dir() {
            continue;
        }

        let files = collect_region_files(directory)?;

        if !files.is_empty() {
            println!(
                "Using {description}: {} ({} region files)",
                directory.display(),
                files.len()
            );

            return Ok(files);
        }
    }

    let other_anvil_files = find_anvil_files(input, 8)?;

    let checked = checked
        .iter()
        .map(|path| {
            format!("  - {}", path.display())
        })
        .collect::<Vec<_>>()
        .join("\n");

    let diagnostic = if other_anvil_files.is_empty() {
        String::from(
            "No other .mca or .mcr files were found inside the world directory."
        )
    } else {
        let mut lines = other_anvil_files
            .iter()
            .take(20)
            .map(|path| {
                format!("  - {}", path.display())
            })
            .collect::<Vec<_>>();

        if other_anvil_files.len() > 20 {
            lines.push(format!(
                "  - ... and {} more",
                other_anvil_files.len() - 20
            ));
        }

        format!(
            "Anvil/region files exist elsewhere, but they are not overworld terrain region files:\n{}\n\
             In particular, files under an 'entities' directory contain entity data, \
             not block/chunk terrain data, and cannot be converted as JavaChunk terrain.",
            lines.join("\n")
        )
    };

    Err(format!(
        "could not find any overworld terrain region files (*.mca) for '{}'.\n\
         Checked:\n{}\n\
         {}",
        input.display(),
        checked,
        diagnostic
    )
    .into())
}

fn collect_region_files(
    directory: &Path,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = fs::read_dir(directory)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|value| value.to_str())
                == Some("mca")
        })
        .collect::<Vec<_>>();

    files.sort();

    Ok(files)
}

fn find_anvil_files(
    root: &Path,
    max_depth: usize,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();

    find_anvil_files_recursive(
        root,
        0,
        max_depth,
        &mut files,
    )?;

    files.sort();

    Ok(files)
}

fn find_anvil_files_recursive(
    directory: &Path,
    depth: usize,
    max_depth: usize,
    files: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    if depth > max_depth {
        return Ok(());
    }

    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,

        Err(error)
            if error.kind()
                == std::io::ErrorKind::PermissionDenied =>
        {
            return Ok(());
        }

        Err(error) => {
            return Err(error.into());
        }
    };

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            find_anvil_files_recursive(
                &path,
                depth + 1,
                max_depth,
                files,
            )?;

            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        let extension = path
            .extension()
            .and_then(|value| value.to_str());

        if matches!(
            extension,
            Some("mca") | Some("mcr")
        ) {
            files.push(path);
        }
    }

    Ok(())
}

fn discover_block_ids(
    mods: &Path,
) -> Result<Vec<String>, Box<dyn Error>> {
    if !mods.is_dir() {
        return Err(format!(
            "mods directory '{}' does not exist",
            mods.display()
        )
        .into());
    }

    let mut ids = Vec::new();

    for entry in fs::read_dir(mods)? {
        let entry = entry?;

        if !entry.file_type()?.is_dir()
            || !entry
                .file_name()
                .to_string_lossy()
                .starts_with("block-")
        {
            continue;
        }

        let manifest =
            entry.path().join("Cargo.toml");

        let Ok(text) =
            fs::read_to_string(manifest)
        else {
            continue;
        };

        let mut in_block_metadata = false;

        for line in text.lines() {
            let line = line.trim();

            if line.starts_with('[') {
                in_block_metadata =
                    line == "[package.metadata.block]";

                continue;
            }

            if in_block_metadata
                && line.starts_with("id")
            {
                if let Some((_, value)) =
                    line.split_once('=')
                {
                    ids.push(
                        value
                            .trim()
                            .trim_matches('"')
                            .to_owned()
                    );
                }

                break;
            }
        }
    }

    ids.sort();
    ids.dedup();

    if ids.is_empty() {
        return Err(format!(
            "no block contributors found under '{}'",
            mods.display()
        )
        .into());
    }

    Ok(ids)
}

fn required_block(
    blocks: &HashMap<String, u32>,
    id: &str,
) -> Result<u32, Box<dyn Error>> {
    blocks
        .get(id)
        .copied()
        .ok_or_else(|| {
            format!(
                "required block '{id}' is unavailable"
            )
            .into()
        })
}

fn read_level(
    path: &Path,
) -> Result<LevelRoot, Box<dyn Error>> {
    let mut bytes = Vec::new();

    GzDecoder::new(
        fs::File::open(path)?
    )
    .read_to_end(&mut bytes)?;

    Ok(fastnbt::from_bytes(&bytes)?)
}

fn print_saved_player_positions(
    world: &Path,
) {
    let candidates = [
        world.join("playerdata"),
        world.join("players"),
    ];

    for directory in candidates {
        if directory.is_dir() {
            print_saved_player_positions_in(
                &directory
            );

            return;
        }
    }
}

fn print_saved_player_positions_in(
    directory: &Path,
) {
    let Ok(entries) =
        fs::read_dir(directory)
    else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path
            .extension()
            .and_then(|value| value.to_str())
            != Some("dat")
        {
            continue;
        }

        let result =
            (|| -> Result<
                MinecraftPlayerData,
                Box<dyn Error>,
            > {
                let mut bytes = Vec::new();

                GzDecoder::new(
                    fs::File::open(&path)?
                )
                .read_to_end(&mut bytes)?;

                Ok(
                    fastnbt::from_bytes(
                        &bytes
                    )?
                )
            })();

        if let Ok(player) = result {
            println!(
                "Saved player {} position: {:?}",
                path.display(),
                player.position
            );
        }
    }
}

fn parse_region_coordinates(
    path: &Path,
) -> Result<(i32, i32), Box<dyn Error>> {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            format!(
                "invalid region filename '{}'",
                path.display()
            )
        })?;

    let mut parts = name.split('.');

    if parts.next() != Some("r") {
        return Err(format!(
            "invalid region filename '{name}'"
        )
        .into());
    }

    let x = parts
        .next()
        .ok_or_else(|| {
            format!(
                "missing X in '{name}'"
            )
        })?
        .parse()?;

    let z = parts
        .next()
        .ok_or_else(|| {
            format!(
                "missing Z in '{name}'"
            )
        })?
        .parse()?;

    if parts.next() != Some("mca") {
        return Err(format!(
            "invalid region filename '{name}'"
        )
        .into());
    }

    Ok((x, z))
}

fn map_block(
    source: &str,
    blocks: &HashMap<String, u32>,
    stone: u32,
) -> u32 {
    let alias = match source {
        "minecraft:air"
        | "minecraft:cave_air"
        | "minecraft:void_air" => {
            Some("demo:air")
        }

        "minecraft:grass_block" => {
            Some("demo:grass")
        }

        "minecraft:snow_block" => {
            Some("demo:snow")
        }

        _ => None,
    };

    if let Some(alias) = alias {
        return blocks
            .get(alias)
            .copied()
            .unwrap_or(stone);
    }

    let Some(path) =
        source.strip_prefix("minecraft:")
    else {
        return stone;
    };

    blocks
        .get(&format!(
            "demo:{}",
            path.replace('_', "-")
        ))
        .copied()
        .unwrap_or(stone)
}

fn write_world(
    output: &Path,
    seed: u64,
    block_ids: &[String],
    chunks: BTreeMap<
        ChunkPos,
        NativeChunk,
    >,
) -> Result<(), Box<dyn Error>> {
    let chunk_root =
        output.join("data/chunk");

    let region_root = chunk_root
        .join("regions")
        .join(storage_source_component(
            OUTPUT_SOURCE
        ));

    fs::create_dir_all(&region_root)?;

    fs::write(
        output.join("info.json"),
        serde_json::to_vec_pretty(
            &WorldInfo { seed }
        )?,
    )?;

    fs::write(
        chunk_root.join("index.bin"),
        encode_global_index(block_ids)?,
    )?;

    let mut regions =
        BTreeMap::<
            RegionPos,
            BTreeMap<
                ChunkPos,
                Vec<u8>,
            >,
        >::new();

    for (position, chunk) in chunks {
        regions
            .entry(
                RegionPos::from_chunk(
                    position
                )
            )
            .or_default()
            .insert(
                position,
                encode_chunk(&chunk)?,
            );
    }

    let mut chunk_count = 0usize;

    for (position, chunks) in regions {
        chunk_count += chunks.len();

        fs::write(
            region_root.join(
                format!(
                    "r.{}.{}.{}.bin",
                    position.x,
                    position.y,
                    position.z
                )
            ),
            encode_region(&chunks)?,
        )?;
    }

    println!(
        "Wrote {chunk_count} non-empty chunks"
    );

    Ok(())
}

fn encode_global_index(
    ids: &[String],
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(b"PWBI");

    write_u16(
        &mut bytes,
        FORMAT_VERSION
    );

    write_u32(
        &mut bytes,
        ids.len().try_into()?
    );

    for id in ids {
        write_u16(
            &mut bytes,
            id.len().try_into()?
        );

        bytes.extend_from_slice(
            id.as_bytes()
        );
    }

    Ok(bytes)
}

fn encode_chunk(
    chunk: &NativeChunk,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut palette =
        Vec::<u32>::new();

    let mut reverse =
        HashMap::<u32, u32>::new();

    let mut entries =
        Vec::with_capacity(CHUNK_VOLUME);

    for block in &chunk.blocks {
        let next =
            palette.len() as u32;

        let local =
            *reverse
                .entry(*block)
                .or_insert_with(|| {
                    palette.push(*block);
                    next
                });

        entries.push(local);
    }

    let bits =
        bits_required(palette.len());

    let words =
        pack_entries(&entries, bits);

    let mut bytes = Vec::new();

    write_u16(
        &mut bytes,
        palette.len().try_into()?
    );

    for global_index in palette {
        write_u32(
            &mut bytes,
            global_index
        );

        write_u32(
            &mut bytes,
            EMPTY_BLOCK_STATE_CBOR
                .len()
                .try_into()?,
        );

        bytes.extend_from_slice(
            EMPTY_BLOCK_STATE_CBOR
        );
    }

    bytes.push(bits);

    write_u32(
        &mut bytes,
        words.len().try_into()?
    );

    for word in words {
        bytes.extend_from_slice(
            &word.to_le_bytes()
        );
    }

    Ok(bytes)
}

fn encode_region(
    chunks: &BTreeMap<
        ChunkPos,
        Vec<u8>,
    >,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(b"PWCR");

    write_u16(
        &mut bytes,
        FORMAT_VERSION
    );

    write_u32(
        &mut bytes,
        chunks.len().try_into()?
    );

    for (position, payload) in chunks {
        for coordinate in [
            position.x,
            position.y,
            position.z,
        ] {
            bytes.extend_from_slice(
                &coordinate.to_le_bytes()
            );
        }

        write_u32(
            &mut bytes,
            payload.len().try_into()?
        );

        bytes.extend_from_slice(
            payload
        );
    }

    Ok(bytes)
}

fn bits_required(
    value_count: usize,
) -> u8 {
    if value_count <= 1 {
        0
    } else {
        (
            usize::BITS
                - (value_count - 1)
                    .leading_zeros()
        ) as u8
    }
}

fn pack_entries(
    entries: &[u32],
    bits: u8,
) -> Vec<u64> {
    if bits == 0 {
        return Vec::new();
    }

    let mut words = vec![
        0_u64;
        (
            entries.len()
                * bits as usize
        )
            .div_ceil(64)
    ];

    let mask =
        (1_u64 << bits) - 1;

    for (index, entry) in
        entries
            .iter()
            .copied()
            .enumerate()
    {
        let bit_index =
            index * bits as usize;

        let word_index =
            bit_index / 64;

        let bit_offset =
            bit_index % 64;

        let value =
            u64::from(entry) & mask;

        words[word_index] |=
            value << bit_offset;

        if bit_offset
            + bits as usize
            > 64
        {
            words[word_index + 1] |=
                value
                    >> (64 - bit_offset);
        }
    }

    words
}

fn storage_source_component(
    source: &str,
) -> String {
    source
        .as_bytes()
        .iter()
        .map(|byte| {
            format!("{byte:02x}")
        })
        .collect()
}

fn write_u16(
    bytes: &mut Vec<u8>,
    value: u16,
) {
    bytes.extend_from_slice(
        &value.to_le_bytes()
    );
}

fn write_u32(
    bytes: &mut Vec<u8>,
    value: u32,
) {
    bytes.extend_from_slice(
        &value.to_le_bytes()
    );
}