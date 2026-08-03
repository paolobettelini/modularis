use codegen_utils::{GeneratedDependency, generate_dependency_toml_line};
use std::collections::{BTreeMap, HashMap, HashSet};
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
    min: Option<f64>,
    max: Option<f64>,
    variant: String,
    section: Option<String>,
    section_label: Option<String>,
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
    let (settings, schema_dependency) = collect_settings(&project)?;
    write_registry(&output, &package, &version, &settings, &schema_dependency)?;
    if let Some(dev_crate) = dev_crate {
        write_registry(
            &dev_crate,
            &package,
            &version,
            &settings,
            &schema_dependency,
        )?;
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

fn collect_settings(project: &Path) -> Result<(Vec<Setting>, GeneratedDependency), Box<dyn Error>> {
    let manifest = read_toml(&project.join("Cargo.toml"))?;
    let dependencies = manifest
        .get("dependencies")
        .and_then(Value::as_table)
        .ok_or("composed project has no dependencies")?;

    let mut settings = Vec::new();
    let mut ids = HashSet::new();
    let mut variants = HashSet::new();
    let mut section_labels = HashMap::<String, String>::new();
    let mut schema_dependency = None;

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
        let min = optional_numeric_bound(setting, "min", &id, &kind)?;
        let max = optional_numeric_bound(setting, "max", &id, &kind)?;
        validate_numeric_range(&id, &kind, &default, min, max)?;
        let input = setting
            .get("input")
            .and_then(Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| kind.clone());
        let section = setting
            .get("section")
            .and_then(Value::as_str)
            .map(str::to_string);
        let section_label = setting
            .get("section_label")
            .and_then(Value::as_str)
            .map(str::to_string);
        if section_label.is_some() && section.is_none() {
            return Err(format!("setting '{id}' has section_label but no section").into());
        }
        if let Some(section) = &section {
            validate_section_id(&id, section)?;
            let label = section_label
                .as_deref()
                .map(str::to_string)
                .unwrap_or_else(|| humanize_section_segment(section));
            if label.is_empty() {
                return Err(format!("setting '{id}' has an empty section label").into());
            }
            if let Some(previous) = section_labels.insert(section.clone(), label.clone()) {
                if previous != label {
                    return Err(format!(
                        "settings section '{section}' has conflicting labels '{previous}' and '{label}'"
                    )
                    .into());
                }
            }
        }

        if !ids.insert(id.clone()) {
            return Err(format!("duplicate setting id '{id}'").into());
        }
        let variant = pascal_identifier(&id);
        if !variants.insert(variant.clone()) {
            return Err(format!("duplicate generated setting variant '{variant}'").into());
        }

        if schema_dependency.is_none() {
            schema_dependency = find_dependency(&mod_manifest, &mod_dir, "settings-schema-api")?;
        }

        settings.push(Setting {
            id,
            label,
            kind,
            input,
            default,
            min,
            max,
            variant,
            section,
            section_label,
        });
    }

    settings.sort_by(|left, right| left.id.cmp(&right.id));
    let schema_dependency =
        schema_dependency.ok_or("no setting contributor exposed settings-schema-api")?;
    Ok((settings, schema_dependency))
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

fn optional_numeric_bound(
    setting: &toml::map::Map<String, Value>,
    field: &str,
    id: &str,
    kind: &str,
) -> Result<Option<f64>, Box<dyn Error>> {
    let Some(value) = setting.get(field) else {
        return Ok(None);
    };
    let bound = match kind {
        "i32" => value
            .as_integer()
            .and_then(|value| i32::try_from(value).ok())
            .map(|value| value as f64),
        "f32" => value
            .as_float()
            .or_else(|| value.as_integer().map(|value| value as f64))
            .filter(|value| value.is_finite()),
        _ => {
            return Err(format!(
                "non-numeric setting '{id}' cannot declare numeric field '{field}'"
            )
            .into());
        }
    };
    bound
        .map(Some)
        .ok_or_else(|| format!("setting '{id}' has an invalid '{field}' bound").into())
}

fn validate_numeric_range(
    id: &str,
    kind: &str,
    default: &Value,
    min: Option<f64>,
    max: Option<f64>,
) -> Result<(), Box<dyn Error>> {
    if let (Some(min), Some(max)) = (min, max)
        && min > max
    {
        return Err(format!("setting '{id}' has min {min} greater than max {max}").into());
    }
    if !matches!(kind, "i32" | "f32") {
        return Ok(());
    }
    let value = default
        .as_float()
        .or_else(|| default.as_integer().map(|value| value as f64))
        .expect("numeric defaults were validated before their ranges");
    if min.is_some_and(|min| value < min) || max.is_some_and(|max| value > max) {
        return Err(format!("setting '{id}' default {value} is outside its declared range").into());
    }
    Ok(())
}

fn write_registry(
    output: &Path,
    package: &str,
    version: &str,
    settings: &[Setting],
    schema_dependency: &GeneratedDependency,
) -> Result<(), Box<dyn Error>> {
    if output.exists() {
        fs::remove_dir_all(output)?;
    }
    fs::create_dir_all(output.join("src"))?;
    fs::write(
        output.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{package}\"\nversion = \"{version}\"\nedition = \"2024\"\n\n[dependencies]\n{}\n",
            generate_dependency_toml_line(output, schema_dependency)
        ),
    )?;
    fs::write(output.join("src/lib.rs"), generate_source(settings)?)?;
    Ok(())
}

fn generate_source(settings: &[Setting]) -> Result<String, Box<dyn Error>> {
    let section_descriptors = section_descriptors(settings);
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
                "const DEF_{}: SettingDefinition = SettingDefinition {{ id: {:?}, label: {:?}, kind: SettingType::{}, input: {:?}, default: {}, number_range: {} }};",
                setting.variant.to_uppercase(),
                setting.id,
                setting.label,
                kind_variant(&setting.kind),
                setting.input,
                default_code(&setting.kind, &setting.default),
                number_range_code(setting.min, setting.max),
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
    let sections = settings
        .iter()
        .map(|setting| {
            let value = setting.section.as_ref().map_or_else(
                || "None".to_string(),
                |section| {
                    let label = setting
                        .section_label
                        .clone()
                        .unwrap_or_else(|| humanize_section_segment(section));
                    let parent = section_parent(section)
                        .map(|parent| format!("Some({parent:?})"))
                        .unwrap_or_else(|| "None".to_string());
                    format!(
                        "Some(SettingSection {{ id: {:?}, label: {:?}, parent: {parent} }})",
                        section, label,
                    )
                },
            );
            format!("        SettingKey::{} => {value},", setting.variant)
        })
        .collect::<Vec<_>>()
        .join("\n");
    let all_sections = section_descriptors
        .iter()
        .map(|(id, label)| {
            let parent = section_parent(id)
                .map(|parent| format!("Some({parent:?})"))
                .unwrap_or_else(|| "None".to_string());
            format!("    SettingSection {{ id: {id:?}, label: {label:?}, parent: {parent} }},")
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(format!(
        "use settings_schema_api::{{SettingDefault, SettingDefinition, SettingNumberRange, SettingSection, SettingType, SettingValue}};\n\n\
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n\
pub enum SettingKey {{\n{variants}\n}}\n\n\
pub const ALL_SETTINGS: &[SettingKey] = &[\n{all}\n];\n\n\
pub const ALL_SETTING_SECTIONS: &[SettingSection] = &[\n{all_sections}\n];\n\n\
{definitions}\n\n\
pub fn all_settings() -> &'static [SettingKey] {{ ALL_SETTINGS }}\n\n\
pub fn all_sections() -> &'static [SettingSection] {{ ALL_SETTING_SECTIONS }}\n\n\
pub fn definition(key: SettingKey) -> &'static SettingDefinition {{\n    match key {{\n{definition_match}\n    }}\n}}\n\n\
pub fn key_from_id(id: &str) -> Option<SettingKey> {{\n    match id {{\n{from_id}\n        _ => None,\n    }}\n}}\n\n\
pub fn section(key: SettingKey) -> Option<SettingSection> {{\n    match key {{\n{sections}\n    }}\n}}\n\n\
pub fn id(key: SettingKey) -> &'static str {{ definition(key).id }}\n\n\
pub fn default_value(key: SettingKey) -> SettingValue {{ definition(key).default.to_value() }}\n"
    ))
}

fn validate_section_id(setting_id: &str, section: &str) -> Result<(), Box<dyn Error>> {
    if section.is_empty()
        || section.starts_with('/')
        || section.ends_with('/')
        || section.split('/').any(str::is_empty)
        || section.split('/').any(|segment| {
            segment.chars().any(|character| {
                !character.is_ascii_alphanumeric() && character != '-' && character != '_'
            })
        })
    {
        return Err(format!("setting '{setting_id}' has invalid section path '{section}'").into());
    }
    Ok(())
}

fn section_descriptors(settings: &[Setting]) -> BTreeMap<String, String> {
    let mut sections = BTreeMap::new();
    for setting in settings {
        let Some(section) = &setting.section else {
            continue;
        };
        let segments = section.split('/').collect::<Vec<_>>();
        for end in 1..=segments.len() {
            let id = segments[..end].join("/");
            sections
                .entry(id)
                .or_insert_with(|| humanize_identifier(segments[end - 1]));
        }
        if let Some(label) = &setting.section_label {
            sections.insert(section.clone(), label.clone());
        }
    }
    sections
}

fn section_parent(section: &str) -> Option<&str> {
    section.rsplit_once('/').map(|(parent, _)| parent)
}

fn humanize_section_segment(section: &str) -> String {
    humanize_identifier(section.rsplit('/').next().unwrap_or(section))
}

fn humanize_identifier(identifier: &str) -> String {
    let words = identifier
        .split(|character: char| character == '-' || character == '_')
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    let mut output = words.join(" ");
    if let Some(first) = output.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    output
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

fn number_range_code(min: Option<f64>, max: Option<f64>) -> String {
    if min.is_none() && max.is_none() {
        return "None".to_string();
    }
    let min = min.map_or_else(|| "None".to_string(), |value| format!("Some({value:?})"));
    let max = max.map_or_else(|| "None".to_string(), |value| format!("Some({value:?})"));
    format!("Some(SettingNumberRange {{ min: {min}, max: {max} }})")
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
