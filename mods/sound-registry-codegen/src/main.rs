use codegen_utils::{GeneratedDependency, generate_dependency_toml_line};
use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

#[derive(Debug)]
struct SoundDeclaration {
    id: String,
    variant: String,
    dependency_key: String,
    dependency_path: PathBuf,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("sound registry codegen failed: {error}");
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
    let mut package = "generated-sound-registry".to_string();
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
    let (sounds, api_dependencies) = collect_sounds(&project)?;
    write_registry(&output, &package, &version, &sounds, &api_dependencies)?;
    if let Some(dev_crate) = dev_crate {
        write_registry(&dev_crate, &package, &version, &sounds, &api_dependencies)?;
    }
    Ok(())
}

fn collect_sounds(
    project: &Path,
) -> Result<(Vec<SoundDeclaration>, BTreeMap<String, GeneratedDependency>), Box<dyn Error>> {
    let manifest = read_toml(&project.join("Cargo.toml"))?;
    let dependencies = manifest
        .get("dependencies")
        .and_then(Value::as_table)
        .ok_or("composed project has no dependencies")?;

    let mut sounds = Vec::new();
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
            .and_then(|value| value.get("sound"))
            .and_then(Value::as_table)
        else {
            continue;
        };

        let id = required_string(metadata, "id")?;
        if id.trim().is_empty() || !id.contains(':') {
            return Err(format!("sound id '{id}' must be a non-empty namespaced id").into());
        }
        if !ids.insert(id.clone()) {
            return Err(format!("duplicate sound id '{id}'").into());
        }
        let variant = pascal_identifier(id.split(':').next_back().unwrap_or(&id));
        if !variants.insert(variant.clone()) {
            return Err(format!("duplicate generated sound variant '{variant}'").into());
        }

        if !api_dependencies.contains_key("sound-api") {
            let dependency = find_dependency(&mod_manifest, &mod_dir, "sound-api")?
                .ok_or_else(|| format!("sound contributor '{id}' does not depend on sound-api"))?;
            api_dependencies.insert("sound-api".to_string(), dependency);
        }

        sounds.push(SoundDeclaration {
            id,
            variant,
            dependency_key: dependency_key.clone(),
            dependency_path: mod_dir,
        });
    }

    sounds.sort_by(|left, right| left.id.cmp(&right.id));
    if sounds.is_empty() {
        return Err("sound registry requires at least one contributor".into());
    }
    Ok((sounds, api_dependencies))
}

fn write_registry(
    output: &Path,
    package: &str,
    version: &str,
    sounds: &[SoundDeclaration],
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
    dependencies.extend(sounds.iter().map(|sound| {
        generate_dependency_toml_line(
            output,
            &GeneratedDependency::path(&sound.dependency_key, &sound.dependency_path),
        )
    }));

    fs::write(
        output.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{package}\"\nversion = \"{version}\"\nedition = \"2024\"\n\n[dependencies]\n{}\n",
            dependencies.join("\n")
        ),
    )?;
    fs::write(output.join("src/lib.rs"), generate_source(sounds))?;
    Ok(())
}

fn generate_source(sounds: &[SoundDeclaration]) -> String {
    let variants = sounds
        .iter()
        .map(|sound| format!("    {},", sound.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let all = sounds
        .iter()
        .map(|sound| format!("    SoundId::{},", sound.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let from_id = sounds
        .iter()
        .map(|sound| {
            format!(
                "        {:?} => Some(SoundId::{}),",
                sound.id, sound.variant
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let info = sounds
        .iter()
        .map(|sound| {
            format!(
                "        SoundId::{} => &{}::SOUND_INFO,",
                sound.variant,
                sound.dependency_key.replace('-', "_")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "use sound_api::SoundInfo;\n\n\
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]\n\
pub enum SoundId {{\n{variants}\n}}\n\n\
pub const ALL_SOUNDS: &[SoundId] = &[\n{all}\n];\n\n\
pub fn all_sounds() -> &'static [SoundId] {{ ALL_SOUNDS }}\n\n\
pub fn from_str(id: &str) -> Option<SoundId> {{\n    match id {{\n{from_id}\n        _ => None,\n    }}\n}}\n\n\
pub fn info(sound: SoundId) -> &'static SoundInfo {{\n    match sound {{\n{info}\n    }}\n}}\n\n\
pub fn id(sound: SoundId) -> &'static str {{ info(sound).id }}\n\
pub fn asset_path(sound: SoundId) -> &'static str {{ info(sound).asset_path }}\n"
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
        .ok_or_else(|| format!("sound metadata field '{key}' must be a string").into())
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
