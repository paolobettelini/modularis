use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

#[derive(Debug)]
struct ItemDeclaration {
    id: String,
    variant: String,
    dependency_key: String,
    dependency_path: PathBuf,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("item registry codegen failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    if args.next().as_deref() != Some("generate") {
        return Err("expected the generate command".into());
    }
    let mut project = None;
    let mut output = None;
    let mut dev_crate = None;
    let mut package = "generated-item-registry".to_string();
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
    let (items, item_api_path) = collect_items(&project)?;
    write_registry(&output, &package, &version, &items, &item_api_path)?;
    if let Some(dev_crate) = dev_crate {
        write_registry(&dev_crate, &package, &version, &items, &item_api_path)?;
    }
    Ok(())
}

fn collect_items(project: &Path) -> Result<(Vec<ItemDeclaration>, PathBuf), Box<dyn Error>> {
    let manifest = read_toml(&project.join("Cargo.toml"))?;
    let dependencies = manifest
        .get("dependencies")
        .and_then(Value::as_table)
        .ok_or("composed project has no dependencies")?;
    let mut items = Vec::new();
    let mut ids = HashSet::new();
    let mut variants = HashSet::new();
    let mut item_api_path = None;
    for (dependency_key, dependency) in dependencies {
        let Some(mod_dir) = dependency_path(project, dependency)? else {
            continue;
        };
        let mod_manifest = read_toml(&mod_dir.join("Cargo.toml"))?;
        let Some(item) = mod_manifest
            .get("package")
            .and_then(|value| value.get("metadata"))
            .and_then(|value| value.get("item"))
            .and_then(Value::as_table)
        else {
            continue;
        };
        let id = required_string(item, "id")?;
        let _label = required_string(item, "label")?;
        if item_api_path.is_none() {
            item_api_path = find_path_dependency(&mod_manifest, &mod_dir, "item-api")?;
        }
        if !ids.insert(id.clone()) {
            return Err(format!("duplicate item id '{id}'").into());
        }
        let variant = pascal_identifier(id.split(':').next_back().unwrap_or(&id));
        if !variants.insert(variant.clone()) {
            return Err(format!("duplicate generated item variant '{variant}'").into());
        }
        items.push(ItemDeclaration {
            id,
            variant,
            dependency_key: dependency_key.clone(),
            dependency_path: mod_dir,
        });
    }
    items.sort_by(|left, right| left.id.cmp(&right.id));
    if items.is_empty() {
        return Err("item registry requires at least one item contributor".into());
    }
    Ok((
        items,
        item_api_path.ok_or("no item contributor exposed item-api")?,
    ))
}

fn write_registry(
    output: &Path,
    package: &str,
    version: &str,
    items: &[ItemDeclaration],
    item_api_path: &Path,
) -> Result<(), Box<dyn Error>> {
    if output.exists() {
        fs::remove_dir_all(output)?;
    }
    fs::create_dir_all(output.join("src"))?;
    let dependencies = items
        .iter()
        .map(|item| {
            format!(
                "{} = {{ path = \"{}\" }}",
                item.dependency_key,
                toml_path(&relative_path(output, &item.dependency_path))
            )
        })
        .chain(std::iter::once(format!(
            "item-api = {{ path = \"{}\" }}",
            toml_path(&relative_path(output, item_api_path))
        )))
        .chain(std::iter::once(
            "serde = { version = \"1.0\", features = [\"derive\"] }".to_string(),
        ))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        output.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{package}\"\nversion = \"{version}\"\nedition = \"2024\"\n\n[dependencies]\n{dependencies}\n"
        ),
    )?;
    fs::write(output.join("src/lib.rs"), generate_source(items))?;
    Ok(())
}

fn generate_source(items: &[ItemDeclaration]) -> String {
    let variants = items
        .iter()
        .map(|item| format!("    {},", item.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let all = items
        .iter()
        .map(|item| format!("    ItemId::{},", item.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let from_id = items
        .iter()
        .map(|item| format!("        {:?} => Some(ItemId::{}),", item.id, item.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let info = items
        .iter()
        .map(|item| {
            format!(
                "        ItemId::{} => &{}::ITEM_INFO,",
                item.variant,
                item.dependency_key.replace('-', "_")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "use item_api::ItemInfo;\n\n\
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]\n\
pub enum ItemId {{\n{variants}\n}}\n\n\
pub const ALL_ITEMS: &[ItemId] = &[\n{all}\n];\n\n\
pub fn all_items() -> &'static [ItemId] {{ ALL_ITEMS }}\n\n\
pub fn from_str(id: &str) -> Option<ItemId> {{\n    match id {{\n{from_id}\n        _ => None,\n    }}\n}}\n\n\
pub fn info(item: ItemId) -> &'static ItemInfo {{\n    match item {{\n{info}\n    }}\n}}\n\n\
pub fn id(item: ItemId) -> &'static str {{ info(item).id }}\n\
pub fn label(item: ItemId) -> &'static str {{ info(item).label }}\n"
    )
}

fn next_value(
    args: &mut impl Iterator<Item = String>,
    flag: &str,
) -> Result<String, Box<dyn Error>> {
    args.next()
        .ok_or_else(|| format!("{flag} requires a value").into())
}

fn required_string(
    table: &toml::map::Map<String, Value>,
    key: &str,
) -> Result<String, Box<dyn Error>> {
    table
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("item metadata field '{key}' must be a string").into())
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

fn find_path_dependency(
    manifest: &Value,
    crate_dir: &Path,
    name: &str,
) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let Some(value) = manifest
        .get("dependencies")
        .and_then(Value::as_table)
        .and_then(|dependencies| dependencies.get(name))
    else {
        return Ok(None);
    };
    dependency_path(crate_dir, value)
}

fn pascal_identifier(input: &str) -> String {
    input
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            chars
                .next()
                .map(|first| first.to_ascii_uppercase().to_string() + chars.as_str())
                .unwrap_or_default()
        })
        .collect()
}

fn read_toml(path: &Path) -> Result<Value, Box<dyn Error>> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}

fn relative_path(from: &Path, to: &Path) -> PathBuf {
    let from = from.canonicalize().unwrap_or_else(|_| from.to_path_buf());
    let to = to.canonicalize().unwrap_or_else(|_| to.to_path_buf());
    let from_components = from.components().collect::<Vec<_>>();
    let to_components = to.components().collect::<Vec<_>>();
    let mut common = 0;
    while common < from_components.len()
        && common < to_components.len()
        && from_components[common] == to_components[common]
    {
        common += 1;
    }
    let mut result = PathBuf::new();
    for _ in common..from_components.len() {
        result.push("..");
    }
    for component in &to_components[common..] {
        result.push(component.as_os_str());
    }
    result
}

fn toml_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
