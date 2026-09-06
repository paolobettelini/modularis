use flate2::read::GzDecoder;
use serde::Deserialize;
use std::{
    error::Error,
    fs,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
pub struct LevelRoot {
    #[serde(rename = "Data")]
    pub data: LevelData,
}

#[derive(Debug, Deserialize)]
pub struct LevelData {
    #[serde(rename = "RandomSeed", default)]
    legacy_seed: Option<i64>,

    #[serde(rename = "WorldGenSettings", default)]
    world_gen: Option<WorldGenSettings>,

    #[serde(rename = "SpawnX", default)]
    pub spawn_x: i32,

    #[serde(rename = "SpawnY", default)]
    pub spawn_y: i32,

    #[serde(rename = "SpawnZ", default)]
    pub spawn_z: i32,
}

#[derive(Debug, Deserialize)]
struct WorldGenSettings {
    seed: i64,
}

#[derive(Debug, Deserialize)]
struct MinecraftPlayerData {
    #[serde(rename = "Pos")]
    position: Vec<f64>,
}

impl LevelData {
    pub fn seed(&self) -> Option<u64> {
        self.world_gen
            .as_ref()
            .map(|settings| settings.seed)
            .or(self.legacy_seed)
            .map(|seed| seed as u64)
    }
}

pub fn validate_input_world(input: &Path) -> Result<(), Box<dyn Error>> {
    if !input.exists() {
        return Err(format!("input world '{}' does not exist", input.display()).into());
    }
    if !input.is_dir() {
        return Err(format!("input world '{}' is not a directory", input.display()).into());
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

pub fn discover_overworld_region_files(input: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let candidates = [
        ("classic/vanilla overworld layout", input.join("region")),
        (
            "dimension-based overworld layout",
            input.join("dimensions/minecraft/overworld/region"),
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
        .map(|path| format!("  - {}", path.display()))
        .collect::<Vec<_>>()
        .join("\n");

    let diagnostic = if other_anvil_files.is_empty() {
        String::from("No other .mca or .mcr files were found inside the world directory.")
    } else {
        let mut lines = other_anvil_files
            .iter()
            .take(20)
            .map(|path| format!("  - {}", path.display()))
            .collect::<Vec<_>>();
        if other_anvil_files.len() > 20 {
            lines.push(format!("  - ... and {} more", other_anvil_files.len() - 20));
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

fn collect_region_files(directory: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = fs::read_dir(directory)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("mca"))
        .collect::<Vec<_>>();
    files.sort();
    Ok(files)
}

fn find_anvil_files(root: &Path, max_depth: usize) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    find_anvil_files_recursive(root, 0, max_depth, &mut files)?;
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
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => return Ok(()),
        Err(error) => return Err(error.into()),
    };

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            find_anvil_files_recursive(&path, depth + 1, max_depth, files)?;
            continue;
        }
        if !file_type.is_file() {
            continue;
        }

        let extension = path.extension().and_then(|value| value.to_str());
        if matches!(extension, Some("mca") | Some("mcr")) {
            files.push(path);
        }
    }

    Ok(())
}

pub fn read_level(path: &Path) -> Result<LevelRoot, Box<dyn Error>> {
    let mut bytes = Vec::new();
    GzDecoder::new(fs::File::open(path)?).read_to_end(&mut bytes)?;
    Ok(fastnbt::from_bytes(&bytes)?)
}

pub fn print_saved_player_positions(world: &Path) {
    for directory in [world.join("playerdata"), world.join("players")] {
        if directory.is_dir() {
            print_saved_player_positions_in(&directory);
            return;
        }
    }
}

fn print_saved_player_positions_in(directory: &Path) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("dat") {
            continue;
        }

        let result = (|| -> Result<MinecraftPlayerData, Box<dyn Error>> {
            let mut bytes = Vec::new();
            GzDecoder::new(fs::File::open(&path)?).read_to_end(&mut bytes)?;
            Ok(fastnbt::from_bytes(&bytes)?)
        })();

        if let Ok(player) = result {
            println!("Saved player {} position: {:?}", path.display(), player.position);
        }
    }
}

pub fn parse_region_coordinates(path: &Path) -> Result<(i32, i32), Box<dyn Error>> {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("invalid region filename '{}'", path.display()))?;

    let mut parts = name.split('.');
    if parts.next() != Some("r") {
        return Err(format!("invalid region filename '{name}'").into());
    }

    let x = parts
        .next()
        .ok_or_else(|| format!("missing X in '{name}'"))?
        .parse()?;
    let z = parts
        .next()
        .ok_or_else(|| format!("missing Z in '{name}'"))?
        .parse()?;

    if parts.next() != Some("mca") {
        return Err(format!("invalid region filename '{name}'").into());
    }

    Ok((x, z))
}
