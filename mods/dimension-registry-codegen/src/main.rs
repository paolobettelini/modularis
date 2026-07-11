use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

#[derive(Debug)]
struct DimensionDeclaration {
    id: String,
    variant: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("dimension registry codegen failed: {error}");
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
    let mut package = "generated-dimension-registry".to_string();
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
    let dimensions = collect_dimensions(&project)?;
    write_registry(&output, &package, &version, &dimensions)?;
    if let Some(dev_crate) = dev_crate {
        write_registry(&dev_crate, &package, &version, &dimensions)?;
    }
    Ok(())
}

fn collect_dimensions(project: &Path) -> Result<Vec<DimensionDeclaration>, Box<dyn Error>> {
    let manifest = read_toml(&project.join("Cargo.toml"))?;
    let dependencies = manifest
        .get("dependencies")
        .and_then(Value::as_table)
        .ok_or("composed project has no dependencies")?;
    let mut dimensions = Vec::new();
    let mut ids = HashSet::new();
    let mut variants = HashSet::new();
    for dependency in dependencies.values() {
        let Some(mod_dir) = dependency_path(&project, dependency)? else {
            continue;
        };
        let mod_manifest = read_toml(&mod_dir.join("Cargo.toml"))?;
        let Some(metadata) = mod_manifest
            .get("package")
            .and_then(|value| value.get("metadata"))
            .and_then(|value| value.get("dimension"))
            .and_then(Value::as_table)
        else {
            continue;
        };
        let id = metadata
            .get("id")
            .and_then(Value::as_str)
            .ok_or("dimension metadata id must be a string")?
            .to_string();
        if !ids.insert(id.clone()) {
            return Err(format!("duplicate dimension id '{id}'").into());
        }
        let variant = pascal_identifier(id.split(':').next_back().unwrap_or(&id));
        if !variants.insert(variant.clone()) {
            return Err(format!("duplicate generated dimension variant '{variant}'").into());
        }
        dimensions.push(DimensionDeclaration { id, variant });
    }
    dimensions.sort_by(|left, right| left.id.cmp(&right.id));
    if dimensions.is_empty() {
        return Err("dimension registry requires at least one contributor".into());
    }
    Ok(dimensions)
}

fn write_registry(
    output: &Path,
    package: &str,
    version: &str,
    dimensions: &[DimensionDeclaration],
) -> Result<(), Box<dyn Error>> {
    if output.exists() {
        fs::remove_dir_all(output)?;
    }
    fs::create_dir_all(output.join("src"))?;
    fs::write(
        output.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{package}\"\nversion = \"{version}\"\nedition = \"2024\"\n\n[dependencies]\nserde = {{ version = \"1.0\", features = [\"derive\"] }}\n"
        ),
    )?;
    fs::write(output.join("src/lib.rs"), generate_source(dimensions))?;
    Ok(())
}

fn generate_source(dimensions: &[DimensionDeclaration]) -> String {
    let variants = dimensions
        .iter()
        .map(|dimension| format!("    {},", dimension.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let all = dimensions
        .iter()
        .map(|dimension| format!("    Dimension::{},", dimension.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let from_id = dimensions
        .iter()
        .map(|dimension| {
            format!(
                "        {:?} => Some(Dimension::{}),",
                dimension.id, dimension.variant
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let ids = dimensions
        .iter()
        .map(|dimension| {
            format!(
                "        Dimension::{} => {:?},",
                dimension.variant, dimension.id
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]\n\
pub enum Dimension {{\n{variants}\n}}\n\n\
pub const ALL_DIMENSIONS: &[Dimension] = &[\n{all}\n];\n\n\
pub fn all_dimensions() -> &'static [Dimension] {{ ALL_DIMENSIONS }}\n\n\
pub fn from_str(id: &str) -> Option<Dimension> {{\n    match id {{\n{from_id}\n        _ => None,\n    }}\n}}\n\n\
pub fn id(dimension: Dimension) -> &'static str {{\n    match dimension {{\n{ids}\n    }}\n}}\n"
    )
}

fn next_value(
    args: &mut impl Iterator<Item = String>,
    flag: &str,
) -> Result<String, Box<dyn Error>> {
    args.next()
        .ok_or_else(|| format!("{flag} requires a value").into())
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
