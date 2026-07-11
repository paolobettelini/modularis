use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

#[derive(Debug)]
struct Setting {
    id: String,
    label: String,
    kind: String,
    input: String,
    default: Value,
    variant: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("settings registry codegen failed: {error}");
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
    let mut package = "generated-client-settings-registry".to_string();
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
    let (settings, schema_path) = collect_settings(&project)?;
    write_registry(&output, &package, &version, &settings, &schema_path)?;
    if let Some(dev_crate) = dev_crate {
        write_registry(&dev_crate, &package, &version, &settings, &schema_path)?;
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

fn collect_settings(project: &Path) -> Result<(Vec<Setting>, PathBuf), Box<dyn Error>> {
    let manifest = read_toml(&project.join("Cargo.toml"))?;
    let dependencies = manifest
        .get("dependencies")
        .and_then(Value::as_table)
        .ok_or("composed project has no dependencies")?;

    let mut settings = Vec::new();
    let mut ids = HashSet::new();
    let mut variants = HashSet::new();
    let mut schema_path = None;

    for dependency in dependencies.values() {
        let Some(mod_dir) = dependency_path(project, dependency)? else {
            continue;
        };
        let mod_manifest = read_toml(&mod_dir.join("Cargo.toml"))?;
        let Some(setting) = mod_manifest
            .get("package")
            .and_then(|value| value.get("metadata"))
            .and_then(|value| value.get("setting"))
            .and_then(Value::as_table)
        else {
            continue;
        };

        let id = required_string(setting, "id")?;
        let label = required_string(setting, "label")?;
        let kind = required_string(setting, "type")?;
        if !matches!(kind.as_str(), "bool" | "i32" | "f32" | "string") {
            return Err(format!("setting '{id}' has unsupported type '{kind}'").into());
        }
        let default = setting
            .get("default")
            .cloned()
            .ok_or_else(|| format!("setting '{id}' has no default"))?;
        validate_default(&id, &kind, &default)?;
        let input = setting
            .get("input")
            .and_then(Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| kind.clone());

        if !ids.insert(id.clone()) {
            return Err(format!("duplicate setting id '{id}'").into());
        }
        let variant = pascal_identifier(&id);
        if !variants.insert(variant.clone()) {
            return Err(format!("duplicate generated setting variant '{variant}'").into());
        }

        if schema_path.is_none() {
            schema_path = find_path_dependency(&mod_manifest, &mod_dir, "settings-schema-api")?;
        }

        settings.push(Setting {
            id,
            label,
            kind,
            input,
            default,
            variant,
        });
    }

    settings.sort_by(|left, right| left.id.cmp(&right.id));
    let schema_path = schema_path.ok_or("no setting contributor exposed settings-schema-api")?;
    Ok((settings, schema_path))
}

fn validate_default(id: &str, kind: &str, value: &Value) -> Result<(), Box<dyn Error>> {
    let valid = match kind {
        "bool" => value.as_bool().is_some(),
        "i32" => value
            .as_integer()
            .and_then(|v| i32::try_from(v).ok())
            .is_some(),
        "f32" => value.as_float().is_some() || value.as_integer().is_some(),
        "string" => value.as_str().is_some(),
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(format!("setting '{id}' default does not match type '{kind}'").into())
    }
}

fn write_registry(
    output: &Path,
    package: &str,
    version: &str,
    settings: &[Setting],
    schema_path: &Path,
) -> Result<(), Box<dyn Error>> {
    if output.exists() {
        fs::remove_dir_all(output)?;
    }
    fs::create_dir_all(output.join("src"))?;
    let relative_schema = relative_path(output, schema_path);
    fs::write(
        output.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{package}\"\nversion = \"{version}\"\nedition = \"2024\"\n\n[dependencies]\nsettings-schema-api = {{ path = \"{}\" }}\n",
            toml_path(&relative_schema)
        ),
    )?;
    fs::write(output.join("src/lib.rs"), generate_source(settings)?)?;
    Ok(())
}

fn generate_source(settings: &[Setting]) -> Result<String, Box<dyn Error>> {
    let variants = settings
        .iter()
        .map(|setting| format!("    {},", setting.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let all = settings
        .iter()
        .map(|setting| format!("    SettingKey::{},", setting.variant))
        .collect::<Vec<_>>()
        .join("\n");
    let definitions = settings
        .iter()
        .map(|setting| {
            format!(
                "const DEF_{}: SettingDefinition = SettingDefinition {{ id: {:?}, label: {:?}, kind: SettingType::{}, input: {:?}, default: {} }};",
                setting.variant.to_uppercase(),
                setting.id,
                setting.label,
                kind_variant(&setting.kind),
                setting.input,
                default_code(&setting.kind, &setting.default)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let definition_match = settings
        .iter()
        .map(|setting| {
            format!(
                "        SettingKey::{} => &DEF_{},",
                setting.variant,
                setting.variant.to_uppercase()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let from_id = settings
        .iter()
        .map(|setting| {
            format!(
                "        {:?} => Some(SettingKey::{}),",
                setting.id, setting.variant
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(format!(
        "use settings_schema_api::{{SettingDefault, SettingDefinition, SettingType, SettingValue}};\n\n\
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n\
pub enum SettingKey {{\n{variants}\n}}\n\n\
pub const ALL_SETTINGS: &[SettingKey] = &[\n{all}\n];\n\n\
{definitions}\n\n\
pub fn all_settings() -> &'static [SettingKey] {{ ALL_SETTINGS }}\n\n\
pub fn definition(key: SettingKey) -> &'static SettingDefinition {{\n    match key {{\n{definition_match}\n    }}\n}}\n\n\
pub fn key_from_id(id: &str) -> Option<SettingKey> {{\n    match id {{\n{from_id}\n        _ => None,\n    }}\n}}\n\n\
pub fn id(key: SettingKey) -> &'static str {{ definition(key).id }}\n\n\
pub fn default_value(key: SettingKey) -> SettingValue {{ definition(key).default.to_value() }}\n"
    ))
}

fn kind_variant(kind: &str) -> &'static str {
    match kind {
        "bool" => "Bool",
        "i32" => "I32",
        "f32" => "F32",
        "string" => "String",
        _ => unreachable!(),
    }
}

fn default_code(kind: &str, value: &Value) -> String {
    match kind {
        "bool" => format!("SettingDefault::Bool({})", value.as_bool().unwrap()),
        "i32" => format!("SettingDefault::I32({})", value.as_integer().unwrap()),
        "f32" => {
            let value = value
                .as_float()
                .unwrap_or_else(|| value.as_integer().unwrap() as f64);
            format!("SettingDefault::F32({value:?}f32)")
        }
        "string" => format!("SettingDefault::String({:?})", value.as_str().unwrap()),
        _ => unreachable!(),
    }
}

fn required_string(
    table: &toml::map::Map<String, Value>,
    key: &str,
) -> Result<String, Box<dyn Error>> {
    table
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("setting metadata field '{key}' must be a string").into())
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
