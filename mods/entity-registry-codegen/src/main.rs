use codegen_utils::{GeneratedDependency, generate_dependency_toml_line};
use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

#[derive(Debug)]
struct EntityDeclaration {
    id: String,
    variant: String,
    dependency_key: String,
    dependency_path: PathBuf,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("entity registry codegen failed: {error}");
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
    let mut package = "generated-entity-registry".to_string();
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
    let (entitys, api_dependencies) = collect_entitys(&project)?;
    write_registry(&output, &package, &version, &entitys, &api_dependencies)?;
    if let Some(dev_crate) = dev_crate {
        write_registry(&dev_crate, &package, &version, &entitys, &api_dependencies)?;
    }
    Ok(())
}

fn collect_entitys(
    project: &Path,
) -> Result<(Vec<EntityDeclaration>, BTreeMap<String, GeneratedDependency>), Box<dyn Error>> {
    let manifest = read_toml(&project.join("Cargo.toml"))?;
    let dependencies = manifest
        .get("dependencies")
        .and_then(Value::as_table)
        .ok_or("composed project has no dependencies")?;

    let mut entitys = Vec::new();
    let mut ids = HashSet::new();
    let mut variants = HashSet::new();
    let mut api_dependencies = BTreeMap::new();

    for (dependency_key, dependency) in dependencies {
        let Some(mod_dir) = dependency_path(project, dependency)? else {
            continue;
        };
        let mod_manifest = read_toml(&mod_dir.join("Cargo.toml"))?;
        let Some(metadata) = mod_manifest
            .get("package")
            .and_then(|value| value.get("metadata"))
            .and_then(|value| value.get("entity"))
            .and_then(Value::as_table)
        else {
            continue;
        };

        let id = required_string(metadata, "id")?;
        if id.trim().is_empty() || !id.contains(':') {
            return Err(format!("entity id '{id}' must be a non-empty namespaced id").into());
        }
        if !ids.insert(id.clone()) {
            return Err(format!("duplicate entity id '{id}'").into());
        }
        let variant = pascal_identifier(id.split(':').next_back().unwrap_or(&id));
        if !variants.insert(variant.clone()) {
            return Err(format!("duplicate generated entity variant '{variant}'").into());
        }

        if !api_dependencies.contains_key("entity-type-api") {
            let dependency = find_dependency(&mod_manifest, &mod_dir, "entity-type-api")?
                .ok_or_else(|| format!("entity contributor '{id}' does not depend on entity-type-api"))?;
            api_dependencies.insert("entity-type-api".to_string(), dependency);
        }

        entitys.push(EntityDeclaration {
            id,
            variant,
            dependency_key: dependency_key.clone(),
            dependency_path: mod_dir,
        });
    }

    entitys.sort_by(|left, right| left.id.cmp(&right.id));
    if !api_dependencies.contains_key("entity-type-api") {
        let dependency = dependencies.get("entity-type-api").ok_or("missing entity-type-api")?;
        api_dependencies.insert("entity-type-api".into(), GeneratedDependency::from_manifest("entity-type-api", dependency, project)?);
    }
    Ok((entitys, api_dependencies))
}

fn write_registry(
    output: &Path,
    package: &str,
    version: &str,
    entitys: &[EntityDeclaration],
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
    dependencies.extend(entitys.iter().map(|entity| {
        generate_dependency_toml_line(
            output,
            &GeneratedDependency::path(&entity.dependency_key, &entity.dependency_path),
        )
    }));

    fs::write(
        output.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{package}\"\nversion = \"{version}\"\nedition = \"2024\"\n\n[dependencies]\n{}\n",
            dependencies.join("\n")
        ),
    )?;
    fs::write(output.join("src/lib.rs"), generate_source(entitys))?;
    Ok(())
}

fn generate_source(entitys: &[EntityDeclaration]) -> String {
    let variants = entitys
        .iter()
        .map(|entity| format!("    {},", entity.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let all = entitys
        .iter()
        .map(|entity| format!("    EntityKind::{},", entity.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let from_id = entitys
        .iter()
        .map(|entity| {
            format!(
                "        {:?} => Some(EntityKind::{}),",
                entity.id, entity.variant
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let info = entitys
        .iter()
        .map(|entity| {
            format!(
                "        EntityKind::{} => &{}::ENTITY_INFO,",
                entity.variant,
                entity.dependency_key.replace('-', "_")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "use entity_type_api::EntityTypeInfo;\n\n\
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]\n\
pub enum EntityKind {{\n{variants}\n}}\n\n\
pub const ALL_ENTITIES: &[EntityKind] = &[\n{all}\n];\n\n\
pub fn all_entities() -> &'static [EntityKind] {{ ALL_ENTITIES }}\n\n\
pub fn from_str(id: &str) -> Option<EntityKind> {{\n    match id {{\n{from_id}\n        _ => None,\n    }}\n}}\n\n\
pub fn info(entity: EntityKind) -> &'static EntityTypeInfo {{\n    match entity {{\n{info}\n    }}\n}}\n\n\
pub fn id(entity: EntityKind) -> &'static str {{ info(entity).id }}\n\
pub fn model_path(entity: EntityKind) -> Option<&'static str> {{ info(entity).model_path }}\n"
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
        .ok_or_else(|| format!("entity metadata field '{key}' must be a string").into())
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
