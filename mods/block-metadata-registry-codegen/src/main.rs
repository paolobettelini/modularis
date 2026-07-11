use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

#[derive(Debug)]
struct MetadataDeclaration {
    id: String,
    field: String,
    ty: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("block metadata registry codegen failed: {error}");
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
    let mut package = "generated-block-metadata".to_string();
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
    let (metadata, dependencies) = collect_metadata(&project)?;
    write_registry(&output, &package, &version, &metadata, &dependencies)?;
    if let Some(dev_crate) = dev_crate {
        write_registry(&dev_crate, &package, &version, &metadata, &dependencies)?;
    }
    Ok(())
}

fn collect_metadata(
    project: &Path,
) -> Result<(Vec<MetadataDeclaration>, BTreeMap<String, PathBuf>), Box<dyn Error>> {
    let manifest = read_toml(&project.join("Cargo.toml"))?;
    let dependencies = manifest
        .get("dependencies")
        .and_then(Value::as_table)
        .ok_or("composed project has no dependencies")?;
    let mut declarations = Vec::new();
    let mut dependency_paths = BTreeMap::new();
    let mut ids = HashSet::new();
    let mut fields = HashSet::new();
    for dependency in dependencies.values() {
        let Some(mod_dir) = dependency_path(project, dependency)? else {
            continue;
        };
        let mod_manifest = read_toml(&mod_dir.join("Cargo.toml"))?;
        let Some(metadata) = mod_manifest
            .get("package")
            .and_then(|value| value.get("metadata"))
            .and_then(|value| value.get("block_metadata"))
            .and_then(Value::as_table)
        else {
            continue;
        };
        let id = required_string(metadata, "id")?;
        let field = required_string(metadata, "field")?;
        let ty = required_string(metadata, "type")?;
        validate_identifier(&field)?;
        if !ids.insert(id.clone()) {
            return Err(format!("duplicate block metadata id '{id}'").into());
        }
        if !fields.insert(field.clone()) {
            return Err(format!("duplicate block metadata field '{field}'").into());
        }
        let crate_ident = ty
            .split("::")
            .next()
            .ok_or_else(|| format!("invalid metadata type '{ty}'"))?;
        let crate_key = crate_ident.replace('_', "-");
        let package_name = mod_manifest
            .get("package")
            .and_then(|value| value.get("name"))
            .and_then(Value::as_str)
            .ok_or("metadata contributor has no package name")?;
        let path = if package_name.replace('-', "_") == crate_ident {
            mod_dir.clone()
        } else {
            find_path_dependency_by_ident(&mod_manifest, &mod_dir, crate_ident)?
                .ok_or_else(|| format!("metadata type '{ty}' is not a path dependency"))?
        };
        dependency_paths.insert(crate_key, path);
        declarations.push(MetadataDeclaration { id, field, ty });
    }
    declarations.sort_by(|left, right| left.id.cmp(&right.id));
    Ok((declarations, dependency_paths))
}

fn write_registry(
    output: &Path,
    package: &str,
    version: &str,
    metadata: &[MetadataDeclaration],
    dependencies: &BTreeMap<String, PathBuf>,
) -> Result<(), Box<dyn Error>> {
    if output.exists() {
        fs::remove_dir_all(output)?;
    }
    fs::create_dir_all(output.join("src"))?;
    let dependency_lines = dependencies
        .iter()
        .map(|(key, path)| {
            format!(
                "{key} = {{ path = \"{}\" }}",
                toml_path(&relative_path(output, path))
            )
        })
        .chain(std::iter::once(
            "serde = { version = \"1.0\", features = [\"derive\"] }".to_string(),
        ))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        output.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{package}\"\nversion = \"{version}\"\nedition = \"2024\"\n\n[dependencies]\n{dependency_lines}\n"
        ),
    )?;
    let fields = metadata
        .iter()
        .map(|entry| format!("    pub {}: Option<{}>,", entry.field, entry.ty))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        output.join("src/lib.rs"),
        format!(
            "#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]\n\
pub struct BlockMetaSet {{\n{fields}\n}}\n"
        ),
    )?;
    Ok(())
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
        .ok_or_else(|| format!("block metadata field '{key}' must be a string").into())
}

fn validate_identifier(value: &str) -> Result<(), Box<dyn Error>> {
    let mut chars = value.chars();
    if !chars
        .next()
        .is_some_and(|character| character == '_' || character.is_ascii_alphabetic())
        || chars.any(|character| character != '_' && !character.is_ascii_alphanumeric())
    {
        return Err(format!("'{value}' is not a valid Rust field identifier").into());
    }
    Ok(())
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

fn find_path_dependency_by_ident(
    manifest: &Value,
    crate_dir: &Path,
    crate_ident: &str,
) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let Some(dependencies) = manifest.get("dependencies").and_then(Value::as_table) else {
        return Ok(None);
    };
    for (key, value) in dependencies {
        if key.replace('-', "_") == crate_ident {
            return dependency_path(crate_dir, value);
        }
    }
    Ok(None)
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
