use codegen_utils::{GeneratedDependency, generate_dependency_toml_line};
use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

#[derive(Debug)]
struct BlockDeclaration {
    id: String,
    variant: String,
    dependency_key: String,
    dependency_path: PathBuf,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("block registry codegen failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let command = args.next().ok_or("missing codegen command")?;
    if command != "generate" {
        return Err(format!("unsupported command '{command}'").into());
    }

    let mut project = None;
    let mut output = None;
    let mut dev_crate = None;
    let mut package = "generated-block-registry".to_string();
    let mut version = "0.1.0".to_string();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--project" => project = Some(PathBuf::from(next_value(&mut args, &arg)?)),
            "--output-crate" => output = Some(PathBuf::from(next_value(&mut args, &arg)?)),
            "--dev-crate" => dev_crate = Some(PathBuf::from(next_value(&mut args, &arg)?)),
            "--package" => package = next_value(&mut args, &arg)?,
            "--version" => version = next_value(&mut args, &arg)?,
            "--mods-folder" | "--modpacks-folder" | "--modpack" => {
                let _ = next_value(&mut args, &arg)?;
            }
            other => return Err(format!("unknown argument '{other}'").into()),
        }
    }

    let project = project.ok_or("missing --project")?.canonicalize()?;
    let output = output.ok_or("missing --output-crate")?;
    let (blocks, api_dependencies) = collect_blocks(&project)?;
    write_registry(&output, &package, &version, &blocks, &api_dependencies)?;
    if let Some(dev_crate) = dev_crate {
        write_registry(&dev_crate, &package, &version, &blocks, &api_dependencies)?;
    }
    Ok(())
}

fn next_value(
    args: &mut impl Iterator<Item = String>,
    flag: &str,
) -> Result<String, Box<dyn Error>> {
    args.next()
        .ok_or_else(|| format!("{flag} requires a value").into())
}

fn collect_blocks(
    project: &Path,
) -> Result<(Vec<BlockDeclaration>, BTreeMap<String, GeneratedDependency>), Box<dyn Error>> {
    let manifest = read_toml(&project.join("Cargo.toml"))?;
    let dependencies = manifest
        .get("dependencies")
        .and_then(Value::as_table)
        .ok_or("composed project has no dependencies")?;

    let mut blocks = Vec::new();
    let mut ids = HashSet::new();
    let mut variants = HashSet::new();
    let mut api_dependencies = BTreeMap::new();

    for (dependency_key, dependency) in dependencies {
        let Some(mod_dir) = dependency_path(project, dependency)? else {
            continue;
        };
        let mod_manifest = read_toml(&mod_dir.join("Cargo.toml"))?;
        let Some(block) = mod_manifest
            .get("package")
            .and_then(|value| value.get("metadata"))
            .and_then(|value| value.get("block"))
            .and_then(Value::as_table)
        else {
            continue;
        };

        validate_block_metadata(block)?;
        let id = required_string(block, "id")?;
        if !ids.insert(id.clone()) {
            return Err(format!("duplicate block id '{id}'").into());
        }
        let variant = pascal_identifier(id.split(':').next_back().unwrap_or(&id));
        if !variants.insert(variant.clone()) {
            return Err(format!("duplicate generated block variant '{variant}'").into());
        }

        for api in ["block-api", "block-render-api"] {
            if !api_dependencies.contains_key(api) {
                let dependency = find_dependency(&mod_manifest, &mod_dir, api)?
                    .ok_or_else(|| format!("block contributor '{id}' does not depend on {api}"))?;
                api_dependencies.insert(api.to_string(), dependency);
            }
        }

        blocks.push(BlockDeclaration {
            id,
            variant,
            dependency_key: dependency_key.clone(),
            dependency_path: mod_dir,
        });
    }

    blocks.sort_by(|left, right| left.id.cmp(&right.id));
    if !blocks.iter().any(|block| block.id == "demo:air") {
        return Err("block registry requires demo:air".into());
    }
    Ok((blocks, api_dependencies))
}

fn validate_block_metadata(block: &toml::map::Map<String, Value>) -> Result<(), Box<dyn Error>> {
    let id = required_string(block, "id")?;
    if id.trim().is_empty() || !id.contains(':') {
        return Err(format!("block id '{id}' must be a non-empty namespaced id").into());
    }
    Ok(())
}

fn write_registry(
    output: &Path,
    package: &str,
    version: &str,
    blocks: &[BlockDeclaration],
    api_dependencies: &BTreeMap<String, GeneratedDependency>,
) -> Result<(), Box<dyn Error>> {
    if output.exists() {
        fs::remove_dir_all(output)?;
    }
    fs::create_dir_all(output.join("src"))?;

    let mut dependencies = api_dependencies
        .values()
        .map(|dependency| generate_dependency_toml_line(output, dependency))
        .collect::<Vec<_>>();
    dependencies.push("serde = { version = \"1.0\", features = [\"derive\"] }".to_string());
    dependencies.extend(blocks.iter().map(|block| {
        generate_dependency_toml_line(
            output,
            &GeneratedDependency::path(&block.dependency_key, &block.dependency_path),
        )
    }));

    fs::write(
        output.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{package}\"\nversion = \"{version}\"\nedition = \"2024\"\n\n[dependencies]\n{}\n",
            dependencies.join("\n")
        ),
    )?;
    fs::write(output.join("src/lib.rs"), generate_source(blocks))?;
    Ok(())
}

fn generate_source(blocks: &[BlockDeclaration]) -> String {
    let variants = blocks
        .iter()
        .map(|block| format!("    {},", block.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let all = blocks
        .iter()
        .map(|block| format!("    BlockId::{},", block.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let from_id = blocks
        .iter()
        .map(|block| {
            format!(
                "        {:?} => Some(BlockId::{}),",
                block.id, block.variant
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let logical = blocks
        .iter()
        .map(|block| {
            format!(
                "        BlockId::{} => &{}::BLOCK_INFO,",
                block.variant,
                block.dependency_key.replace('-', "_")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let render = blocks
        .iter()
        .map(|block| {
            format!(
                "        BlockId::{} => &{}::RENDER_INFO,",
                block.variant,
                block.dependency_key.replace('-', "_")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "use block_api::BlockInfo;\nuse block_render_api::BlockRenderInfo;\n\n\
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]\n\
pub enum BlockId {{\n{variants}\n}}\n\n\
pub const ALL_BLOCKS: &[BlockId] = &[\n{all}\n];\n\n\
pub fn all_blocks() -> &'static [BlockId] {{ ALL_BLOCKS }}\n\n\
pub fn from_str(id: &str) -> Option<BlockId> {{\n    match id {{\n{from_id}\n        _ => None,\n    }}\n}}\n\n\
pub fn info(block: BlockId) -> &'static BlockInfo {{\n    match block {{\n{logical}\n    }}\n}}\n\n\
pub fn render_info(block: BlockId) -> &'static BlockRenderInfo {{\n    match block {{\n{render}\n    }}\n}}\n\n\
pub fn id(block: BlockId) -> &'static str {{ info(block).id }}\n\
pub fn is_air(block: BlockId) -> bool {{ info(block).is_air }}\n\
pub fn is_solid(block: BlockId) -> bool {{ info(block).solid }}\n\
pub fn is_opaque(block: BlockId) -> bool {{ info(block).opaque }}\n"
    )
}

fn required_string(
    table: &toml::map::Map<String, Value>,
    key: &str,
) -> Result<String, Box<dyn Error>> {
    table
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("block metadata field '{key}' must be a string").into())
}

fn dependency_path(base: &Path, value: &Value) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let Some(path) = value
        .as_table()
        .and_then(|table| table.get("path"))
        .and_then(Value::as_str)
    else {
        return Ok(None);
    };
    let path = PathBuf::from(path);
    Ok(Some(
        if path.is_absolute() {
            path
        } else {
            base.join(path)
        }
        .canonicalize()?,
    ))
}

fn find_dependency(
    manifest: &Value,
    crate_dir: &Path,
    name: &str,
) -> Result<Option<GeneratedDependency>, Box<dyn Error>> {
    let Some(value) = manifest
        .get("dependencies")
        .and_then(Value::as_table)
        .and_then(|dependencies| dependencies.get(name))
    else {
        return Ok(None);
    };
    GeneratedDependency::from_manifest(name, value, crate_dir).map(Some)
}

fn pascal_identifier(input: &str) -> String {
    input
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn read_toml(path: &Path) -> Result<Value, Box<dyn Error>> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}
