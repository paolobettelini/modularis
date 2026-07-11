use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct GeneratedCrate {
    pub package: String,
    pub version: String,
    pub dependencies: Vec<GeneratedDependency>,
    pub lib_rs: String,
    pub generated_files: Vec<GeneratedFile>,
}

#[derive(Debug, Clone)]
pub struct GeneratedDependency {
    pub key: String,
    pub package: Option<String>,
    pub source: GeneratedDependencySource,
    pub features: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum GeneratedDependencySource {
    Version(String),
    Path(PathBuf),
}

#[derive(Debug, Clone)]
pub struct GeneratedFile {
    pub relative_path: PathBuf,
    pub contents: String,
}

impl GeneratedDependency {
    pub fn path<K, P>(key: K, path: P) -> Self
    where
        K: Into<String>,
        P: Into<PathBuf>,
    {
        Self {
            key: key.into(),
            package: None,
            source: GeneratedDependencySource::Path(path.into()),
            features: Vec::new(),
        }
    }

    pub fn version<K, V>(key: K, version: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        Self {
            key: key.into(),
            package: None,
            source: GeneratedDependencySource::Version(version.into()),
            features: Vec::new(),
        }
    }

    pub fn with_package<P>(mut self, package: P) -> Self
    where
        P: Into<String>,
    {
        self.package = Some(package.into());
        self
    }

    pub fn with_feature<F>(mut self, feature: F) -> Self
    where
        F: Into<String>,
    {
        self.features.push(feature.into());
        self
    }
}

impl GeneratedFile {
    pub fn new<P, C>(relative_path: P, contents: C) -> Self
    where
        P: Into<PathBuf>,
        C: Into<String>,
    {
        Self {
            relative_path: relative_path.into(),
            contents: contents.into(),
        }
    }
}

pub fn write_generated_crate(
    crate_dir: &Path,
    generated_crate: &GeneratedCrate,
) -> Result<(), Box<dyn Error>> {
    if crate_dir.exists() {
        fs::remove_dir_all(crate_dir)?;
    }

    fs::create_dir_all(crate_dir.join("src"))?;
    fs::write(
        crate_dir.join("Cargo.toml"),
        generate_cargo_toml(crate_dir, generated_crate),
    )?;
    fs::write(
        crate_dir.join("src").join("lib.rs"),
        &generated_crate.lib_rs,
    )?;

    for file in &generated_crate.generated_files {
        let path = crate_dir.join("src").join(&file.relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, &file.contents)?;
    }

    Ok(())
}

pub fn generate_cargo_toml(crate_dir: &Path, generated_crate: &GeneratedCrate) -> String {
    let dependencies = generated_crate
        .dependencies
        .iter()
        .map(|dependency| generate_dependency_toml_line(crate_dir, dependency))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"[package]
name = "{}"
version = "{}"
edition = "2024"

[dependencies]
{}
"#,
        generated_crate.package, generated_crate.version, dependencies
    )
}

pub fn generate_dependency_toml_line(crate_dir: &Path, dependency: &GeneratedDependency) -> String {
    if let GeneratedDependencySource::Version(version) = &dependency.source {
        if dependency.package.is_none() && dependency.features.is_empty() {
            return format!("{} = \"{version}\"", dependency.key);
        }
    }

    let mut fields = Vec::new();
    if let Some(package) = &dependency.package {
        fields.push(format!("package = \"{package}\""));
    }
    match &dependency.source {
        GeneratedDependencySource::Version(version) => {
            fields.push(format!("version = \"{version}\""))
        }
        GeneratedDependencySource::Path(path) => fields.push(format!(
            "path = \"{}\"",
            path_to_toml_string(&relative_path(crate_dir, path))
        )),
    }
    if !dependency.features.is_empty() {
        fields.push(format!(
            "features = [{}]",
            dependency
                .features
                .iter()
                .map(|feature| format!("\"{feature}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    format!("{} = {{ {} }}", dependency.key, fields.join(", "))
}

pub fn crate_ident_for_type(ty: &str) -> Result<String, Box<dyn Error>> {
    let crate_ident = ty
        .split("::")
        .next()
        .filter(|segment| !segment.is_empty())
        .ok_or_else(|| format!("invalid Rust type path '{ty}'"))?;

    Ok(crate_ident.to_string())
}

pub fn variant_name_for_type(ty: &str) -> Result<String, Box<dyn Error>> {
    let base = ty
        .split('<')
        .next()
        .unwrap_or(ty)
        .rsplit("::")
        .next()
        .ok_or_else(|| format!("invalid Rust type path '{ty}'"))?;

    rust_ident(base)
}

pub fn rust_ident(input: &str) -> Result<String, Box<dyn Error>> {
    let mut ident = String::new();
    for (idx, ch) in input.chars().enumerate() {
        if ch == '_' || ch.is_ascii_alphabetic() || (idx > 0 && ch.is_ascii_digit()) {
            ident.push(ch);
        } else {
            ident.push('_');
        }
    }

    if ident.is_empty() {
        return Err(format!("'{input}' cannot be converted into a Rust identifier").into());
    }

    if ident
        .chars()
        .next()
        .map(|ch| ch == '_' || ch.is_ascii_alphabetic())
        .unwrap_or(false)
    {
        Ok(ident)
    } else {
        Ok(format!("_{ident}"))
    }
}

pub fn normalize_crate_name(name: &str) -> String {
    name.replace('-', "_")
}

pub fn relative_path(from_dir: &Path, to: &Path) -> PathBuf {
    let Ok(from_dir) = from_dir.canonicalize() else {
        return to.to_path_buf();
    };
    let Ok(to) = to.canonicalize() else {
        return to.to_path_buf();
    };

    let from_components = from_dir.components().collect::<Vec<_>>();
    let to_components = to.components().collect::<Vec<_>>();
    let mut common_len = 0;

    while common_len < from_components.len()
        && common_len < to_components.len()
        && from_components.get(common_len) == to_components.get(common_len)
    {
        common_len += 1;
    }

    if common_len == 0 {
        return to;
    }

    let mut path = PathBuf::new();
    for _ in common_len..from_components.len() {
        path.push("..");
    }
    for component in &to_components[common_len..] {
        path.push(component.as_os_str());
    }

    if path.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        path
    }
}

pub fn path_to_toml_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
