use std::{collections::HashSet, error::Error, fs, path::{Path, PathBuf}};
use toml::Value;

#[derive(Debug)]
struct Declaration {
    id: String,
    short_id: String,
    variant: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("game mode registry codegen failed: {error}");
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
    let mut package = "generated-game-mode-registry".to_string();
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
    let declarations = collect(&project)?;
    let output = output.ok_or("missing --output-crate")?;
    write_registry(&output, &package, &version, &declarations)?;
    if let Some(dev_crate) = dev_crate {
        write_registry(&dev_crate, &package, &version, &declarations)?;
    }
    Ok(())
}

fn collect(project: &Path) -> Result<Vec<Declaration>, Box<dyn Error>> {
    let manifest = read_toml(&project.join("Cargo.toml"))?;
    let dependencies = manifest.get("dependencies").and_then(Value::as_table)
        .ok_or("composed project has no dependencies")?;
    let mut ids = HashSet::new();
    let mut short_ids = HashSet::new();
    let mut variants = HashSet::new();
    let mut declarations = Vec::new();
    for dependency in dependencies.values() {
        let Some(dir) = dependency_path(project, dependency)? else { continue };
        let manifest = read_toml(&dir.join("Cargo.toml"))?;
        let Some(metadata) = manifest.get("package").and_then(|v| v.get("metadata"))
            .and_then(|v| v.get("game_mode")).and_then(Value::as_table) else { continue };
        let id = metadata.get("id").and_then(Value::as_str)
            .ok_or("game mode metadata id must be a string")?.to_ascii_lowercase();
        if !id.contains(':') {
            return Err(format!("game mode id '{id}' must be namespaced").into());
        }
        let short_id = id.rsplit(':').next().unwrap().to_string();
        let variant = pascal_identifier(&short_id);
        if !ids.insert(id.clone()) { return Err(format!("duplicate game mode id '{id}'").into()); }
        if !short_ids.insert(short_id.clone()) {
            return Err(format!("ambiguous game mode short id '{short_id}'").into());
        }
        if !variants.insert(variant.clone()) {
            return Err(format!("duplicate generated game mode variant '{variant}'").into());
        }
        declarations.push(Declaration { id, short_id, variant });
    }
    declarations.sort_by(|a, b| a.id.cmp(&b.id));
    if declarations.is_empty() { return Err("game mode registry requires at least one contributor".into()); }
    Ok(declarations)
}

fn write_registry(output: &Path, package: &str, version: &str, declarations: &[Declaration]) -> Result<(), Box<dyn Error>> {
    if output.exists() { fs::remove_dir_all(output)?; }
    fs::create_dir_all(output.join("src"))?;
    fs::write(output.join("Cargo.toml"), format!(
        "[package]\nname = \"{package}\"\nversion = \"{version}\"\nedition = \"2024\"\n\n[dependencies]\nserde = {{ version = \"1.0\", features = [\"derive\"] }}\n"
    ))?;
    fs::write(output.join("src/lib.rs"), source(declarations))?;
    Ok(())
}

fn source(declarations: &[Declaration]) -> String {
    let variants = declarations.iter().map(|d| format!("    {},", d.variant)).collect::<Vec<_>>().join("\n");
    let all = declarations.iter().map(|d| format!("    GameMode::{},", d.variant)).collect::<Vec<_>>().join("\n");
    let parse = declarations.iter().map(|d| format!(
        "        {:?} | {:?} => Some(GameMode::{}),", d.id, d.short_id, d.variant
    )).collect::<Vec<_>>().join("\n");
    let ids = declarations.iter().map(|d| format!("        GameMode::{} => {:?},", d.variant, d.id)).collect::<Vec<_>>().join("\n");
    let shorts = declarations.iter().map(|d| format!("        GameMode::{} => {:?},", d.variant, d.short_id)).collect::<Vec<_>>().join("\n");
    format!("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]\npub enum GameMode {{\n{variants}\n}}\n\nimpl GameMode {{\n    pub fn parse(value: &str) -> Option<Self> {{ from_str(value) }}\n    pub fn id(self) -> &'static str {{ id(self) }}\n    pub fn short_id(self) -> &'static str {{ short_id(self) }}\n}}\n\npub const ALL_GAME_MODES: &[GameMode] = &[\n{all}\n];\n\npub fn all_game_modes() -> &'static [GameMode] {{ ALL_GAME_MODES }}\n\npub fn from_str(value: &str) -> Option<GameMode> {{\n    match value.to_ascii_lowercase().as_str() {{\n{parse}\n        _ => None,\n    }}\n}}\n\npub fn id(mode: GameMode) -> &'static str {{\n    match mode {{\n{ids}\n    }}\n}}\n\npub fn short_id(mode: GameMode) -> &'static str {{\n    match mode {{\n{shorts}\n    }}\n}}\n")
}

fn next_value(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, Box<dyn Error>> {
    args.next().ok_or_else(|| format!("{flag} requires a value").into())
}

fn dependency_path(base: &Path, value: &Value) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let Some(path) = value.as_table().and_then(|t| t.get("path")).and_then(Value::as_str) else { return Ok(None) };
    let path = PathBuf::from(path);
    Ok(Some(if path.is_absolute() { path } else { base.join(path) }.canonicalize()?))
}

fn pascal_identifier(input: &str) -> String {
    input.split(|c: char| !c.is_ascii_alphanumeric()).filter(|s| !s.is_empty()).map(|s| {
        let mut chars = s.chars();
        chars.next().map(|first| first.to_ascii_uppercase().to_string() + chars.as_str()).unwrap_or_default()
    }).collect()
}

fn read_toml(path: &Path) -> Result<Value, Box<dyn Error>> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}
