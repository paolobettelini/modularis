use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

#[derive(Debug, Clone)]
struct PermissionDeclaration {
    id: String,
    variant: String,
    implies: Vec<String>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("permission registry codegen failed: {error}");
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
    let mut package = "generated-permission-registry".to_string();
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
    let permissions = collect_permissions(&project)?;
    let output = output.ok_or("missing --output-crate")?;
    write_registry(&output, &package, &version, &permissions)?;
    if let Some(dev_crate) = dev_crate {
        write_registry(&dev_crate, &package, &version, &permissions)?;
    }
    Ok(())
}

fn collect_permissions(project: &Path) -> Result<Vec<PermissionDeclaration>, Box<dyn Error>> {
    let manifest = read_toml(&project.join("Cargo.toml"))?;
    let dependencies = manifest
        .get("dependencies")
        .and_then(Value::as_table)
        .ok_or("composed project has no dependencies")?;
    let mut declarations = Vec::new();
    let mut ids = HashSet::new();
    let mut variants = HashSet::new();
    for dependency in dependencies.values() {
        let Some(mod_dir) = dependency_path(project, dependency)? else {
            continue;
        };
        let manifest = read_toml(&mod_dir.join("Cargo.toml"))?;
        let Some(metadata) = manifest
            .get("package")
            .and_then(|value| value.get("metadata"))
            .and_then(|value| value.get("permission"))
            .and_then(Value::as_table)
        else {
            continue;
        };
        let id = metadata
            .get("id")
            .and_then(Value::as_str)
            .ok_or("permission metadata id must be a string")?
            .to_string();
        if !ids.insert(id.clone()) {
            return Err(format!("duplicate permission id '{id}'").into());
        }
        let variant = pascal_identifier(id.split(':').next_back().unwrap_or(&id));
        if !variants.insert(variant.clone()) {
            return Err(format!("duplicate generated permission variant '{variant}'").into());
        }
        let implies = metadata
            .get("implies")
            .map(|value| {
                value
                    .as_array()
                    .ok_or("permission implies must be an array")?
                    .iter()
                    .map(|entry| {
                        entry
                            .as_str()
                            .map(str::to_string)
                            .ok_or("permission implication must be a string")
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?
            .unwrap_or_default();
        declarations.push(PermissionDeclaration { id, variant, implies });
    }
    declarations.sort_by(|left, right| left.id.cmp(&right.id));
    if declarations.is_empty() {
        return Err("permission registry requires at least one contributor".into());
    }
    validate_hierarchy(&declarations)?;
    Ok(declarations)
}

fn validate_hierarchy(declarations: &[PermissionDeclaration]) -> Result<(), Box<dyn Error>> {
    let by_id = declarations
        .iter()
        .map(|permission| (permission.id.as_str(), permission))
        .collect::<HashMap<_, _>>();
    for permission in declarations {
        for implied in &permission.implies {
            if !by_id.contains_key(implied.as_str()) {
                return Err(format!("permission '{}' implies missing permission '{implied}'", permission.id).into());
            }
        }
        visit(&permission.id, &permission.id, &by_id, &mut HashSet::new())?;
    }
    Ok(())
}

fn visit<'a>(
    root: &str,
    current: &str,
    by_id: &HashMap<&'a str, &'a PermissionDeclaration>,
    path: &mut HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    if !path.insert(current.to_string()) {
        return Err(format!("cyclic permission hierarchy reachable from '{root}' through '{current}'").into());
    }
    for implied in &by_id[current].implies {
        visit(root, implied, by_id, path)?;
    }
    path.remove(current);
    Ok(())
}

fn write_registry(
    output: &Path,
    package: &str,
    version: &str,
    permissions: &[PermissionDeclaration],
) -> Result<(), Box<dyn Error>> {
    if output.exists() {
        fs::remove_dir_all(output)?;
    }
    fs::create_dir_all(output.join("src"))?;
    fs::write(
        output.join("Cargo.toml"),
        format!("[package]\nname = \"{package}\"\nversion = \"{version}\"\nedition = \"2024\"\n\n[dependencies]\nserde = {{ version = \"1.0\", features = [\"derive\"] }}\n"),
    )?;
    fs::write(output.join("src/lib.rs"), generate_source(permissions))?;
    Ok(())
}

fn generate_source(permissions: &[PermissionDeclaration]) -> String {
    let variants = permissions.iter().map(|p| format!("    {},", p.variant)).collect::<Vec<_>>().join("\n");
    let all = permissions.iter().map(|p| format!("    PermissionId::{},", p.variant)).collect::<Vec<_>>().join("\n");
    let from_id = permissions.iter().map(|p| format!("        {:?} => Some(PermissionId::{}),", p.id, p.variant)).collect::<Vec<_>>().join("\n");
    let ids = permissions.iter().map(|p| format!("        PermissionId::{} => {:?},", p.variant, p.id)).collect::<Vec<_>>().join("\n");
    let direct = permissions.iter().map(|p| {
        let implied = p.implies.iter().map(|id| permissions.iter().find(|candidate| candidate.id == *id).unwrap()).map(|p| format!("PermissionId::{}", p.variant)).collect::<Vec<_>>().join(", ");
        format!("        PermissionId::{} => &[{}],", p.variant, implied)
    }).collect::<Vec<_>>().join("\n");
    format!("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]\npub enum PermissionId {{\n{variants}\n}}\n\npub const ALL_PERMISSIONS: &[PermissionId] = &[\n{all}\n];\n\npub fn all_permissions() -> &'static [PermissionId] {{ ALL_PERMISSIONS }}\n\npub fn from_str(id: &str) -> Option<PermissionId> {{\n    match id {{\n{from_id}\n        _ => None,\n    }}\n}}\n\npub fn id(permission: PermissionId) -> &'static str {{\n    match permission {{\n{ids}\n    }}\n}}\n\npub fn direct_implications(permission: PermissionId) -> &'static [PermissionId] {{\n    match permission {{\n{direct}\n    }}\n}}\n\npub fn implies(granted: PermissionId, requested: PermissionId) -> bool {{\n    if granted == requested {{ return true; }}\n    let mut pending = direct_implications(granted).to_vec();\n    let mut visited = std::collections::HashSet::new();\n    while let Some(permission) = pending.pop() {{\n        if permission == requested {{ return true; }}\n        if visited.insert(permission) {{ pending.extend_from_slice(direct_implications(permission)); }}\n    }}\n    false\n}}\n")
}

fn next_value(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, Box<dyn Error>> {
    args.next().ok_or_else(|| format!("{flag} requires a value").into())
}

fn dependency_path(base: &Path, value: &Value) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let Some(path) = value.as_table().and_then(|table| table.get("path")).and_then(Value::as_str) else {
        return Ok(None);
    };
    let path = PathBuf::from(path);
    Ok(Some(if path.is_absolute() { path } else { base.join(path) }.canonicalize()?))
}

fn pascal_identifier(input: &str) -> String {
    input
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            chars.next().map(|first| first.to_ascii_uppercase().to_string() + chars.as_str()).unwrap_or_default()
        })
        .collect()
}

fn read_toml(path: &Path) -> Result<Value, Box<dyn Error>> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}
