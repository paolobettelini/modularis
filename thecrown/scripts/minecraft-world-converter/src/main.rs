mod mapping;
mod minecraft;
mod output;

use clap::Parser;
use fastanvil::{Chunk as MinecraftChunk, JavaChunk, Region};
use mapping::{BlockMapper, MappingConfig, MappingDecision};
use minecraft::{
    discover_overworld_region_files, parse_region_coordinates, print_saved_player_positions,
    read_level, validate_input_world,
};
use output::{ChunkPos, NativeChunk, write_world};
use serde::Serialize;
use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    fs,
    path::{Path, PathBuf},
};

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

    /// Block mapping file. Defaults to block-mappings.toml next to Cargo.toml.
    #[arg(long, value_name = "PATH")]
    mapping: Option<PathBuf>,

    /// Disable semantic nearest-block matching after explicit and material mappings.
    #[arg(long)]
    no_fuzzy: bool,

    /// Optional path for the JSON conversion report. By default it is written into the output.
    #[arg(long, value_name = "PATH")]
    report: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct ReportEntry {
    source: String,
    count: u64,
    target: String,
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<i32>,
}

#[derive(Debug, Serialize)]
struct ConversionReport {
    total_block_kinds: usize,
    total_blocks_seen: u64,
    methods: BTreeMap<String, usize>,
    mappings: Vec<ReportEntry>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    validate_input_world(&args.input)?;

    if args.output.exists() {
        return Err(format!(
            "output '{}' already exists; remove or move it explicitly before converting again",
            args.output.display()
        )
        .into());
    }

    let block_ids = discover_block_ids(&args.mods)?;
    let block_indices = block_ids
        .iter()
        .enumerate()
        .map(|(index, id)| (id.clone(), index as u32))
        .collect::<HashMap<_, _>>();

    let air = required_block(&block_indices, "demo:air")?;
    required_block(&block_indices, "demo:stone")?;

    let mapping_path = args
        .mapping
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("block-mappings.toml"));
    let mut mapping_config = MappingConfig::load(&mapping_path)?;
    if args.no_fuzzy {
        mapping_config.disable_fuzzy();
    }
    let mapper = BlockMapper::new(block_indices, mapping_config)?;

    println!("Block mappings: {}", mapping_path.display());

    let level = read_level(&args.input.join("level.dat"))?;
    let seed = match level.data.seed() {
        Some(seed) => seed,
        None => {
            eprintln!(
                "Warning: '{}' contains neither WorldGenSettings.seed nor RandomSeed; using seed 0",
                args.input.join("level.dat").display()
            );
            0
        }
    };

    println!(
        "Minecraft source: {} (seed {seed}, spawn {}, {}, {})",
        args.input.display(),
        level.data.spawn_x,
        level.data.spawn_y,
        level.data.spawn_z,
    );

    print_saved_player_positions(&args.input);
    let region_files = discover_overworld_region_files(&args.input)?;

    let mut chunks = BTreeMap::<ChunkPos, NativeChunk>::new();
    let mut mappings = HashMap::<String, MappingDecision>::new();
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
                        *source_counts.entry(source_name.to_owned()).or_default() += 1;

                        let decision = mappings
                            .entry(source_name.to_owned())
                            .or_insert_with(|| mapper.map(source_name));
                        let block = decision.block;

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
                                    minimum[axis] = minimum[axis].min(world_position[axis]);
                                    maximum[axis] = maximum[axis].max(world_position[axis]);
                                }
                            }
                            None => bounds = Some((world_position, world_position)),
                        }

                        let position = ChunkPos {
                            x: chunk_x,
                            y: (y as i32).div_euclid(16),
                            z: chunk_z,
                        };
                        chunks
                            .entry(position)
                            .or_insert_with(|| NativeChunk::empty(air))
                            .set(x, (y as i32).rem_euclid(16) as usize, z, block);
                    }
                }
            }
        }
    }

    write_world(&args.output, seed, &block_ids, chunks)?;

    if let Some((minimum, maximum)) = bounds {
        println!("Non-air block bounds: {minimum:?} .. {maximum:?}");
    }

    let report = build_report(&mappings, &source_counts);
    let report_path = args
        .report
        .unwrap_or_else(|| args.output.join("conversion-report.json"));
    if let Some(parent) = report_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(&report_path, serde_json::to_vec_pretty(&report)?)?;

    println!("Converted world written to {}", args.output.display());
    println!("Mapping report written to {}", report_path.display());
    print_mapping_summary(&report);

    Ok(())
}

fn build_report(
    mappings: &HashMap<String, MappingDecision>,
    source_counts: &HashMap<String, u64>,
) -> ConversionReport {
    let mut methods = BTreeMap::<String, usize>::new();
    let mut entries = mappings
        .iter()
        .map(|(source, decision)| {
            *methods.entry(decision.method.clone()).or_default() += 1;
            ReportEntry {
                source: source.clone(),
                count: source_counts.get(source).copied().unwrap_or_default(),
                target: decision.target.clone(),
                method: decision.method.clone(),
                score: decision.score,
            }
        })
        .collect::<Vec<_>>();

    entries.sort_by(|left, right| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| left.source.cmp(&right.source))
    });

    ConversionReport {
        total_block_kinds: entries.len(),
        total_blocks_seen: source_counts.values().sum(),
        methods,
        mappings: entries,
    }
}

fn print_mapping_summary(report: &ConversionReport) {
    println!("Mapped {} Minecraft block kinds", report.total_block_kinds);
    for (method, count) in &report.methods {
        println!("  {method}: {count}");
    }

    let fallback = report
        .mappings
        .iter()
        .filter(|entry| entry.method.starts_with("fallback"))
        .take(30)
        .collect::<Vec<_>>();

    if fallback.is_empty() {
        println!("No block kinds fell back to stone.");
    } else {
        println!("Most common remaining stone fallbacks:");
        for entry in fallback {
            println!("  {}: {}", entry.source, entry.count);
        }
    }
}

fn discover_block_ids(mods: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    if !mods.is_dir() {
        return Err(format!("mods directory '{}' does not exist", mods.display()).into());
    }

    let mut ids = Vec::new();
    for entry in fs::read_dir(mods)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir()
            || !entry.file_name().to_string_lossy().starts_with("block-")
        {
            continue;
        }

        let manifest = entry.path().join("Cargo.toml");
        let Ok(text) = fs::read_to_string(manifest) else {
            continue;
        };

        let mut in_block_metadata = false;
        for line in text.lines() {
            let line = line.trim();
            if line.starts_with('[') {
                in_block_metadata = line == "[package.metadata.block]";
                continue;
            }

            if in_block_metadata && line.starts_with("id") {
                if let Some((_, value)) = line.split_once('=') {
                    ids.push(value.trim().trim_matches('"').to_owned());
                }
                break;
            }
        }
    }

    ids.sort();
    ids.dedup();
    if ids.is_empty() {
        return Err(format!("no block contributors found under '{}'", mods.display()).into());
    }

    Ok(ids)
}

fn required_block(blocks: &HashMap<String, u32>, id: &str) -> Result<u32, Box<dyn Error>> {
    blocks
        .get(id)
        .copied()
        .ok_or_else(|| format!("required block '{id}' is unavailable").into())
}
