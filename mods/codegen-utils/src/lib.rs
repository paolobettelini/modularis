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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedDependency {
    pub key: String,
    pub package: Option<String>,
    pub source: GeneratedDependencySource,
    pub features: Vec<String>,
    pub default_features: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratedDependencySource {
    Version(String),
    Path(PathBuf),
    Git {
        repository: String,
        branch: Option<String>,
        tag: Option<String>,
        rev: Option<String>,
    },
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
            default_features: None,
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
            default_features: None,
        }
    }

    pub fn git<K, R>(key: K, repository: R) -> Self
    where
        K: Into<String>,
        R: Into<String>,
    {
        Self {
            key: key.into(),
            package: None,
            source: GeneratedDependencySource::Git {
                repository: repository.into(),
                branch: None,
                tag: None,
                rev: None,
            },
            features: Vec::new(),
            default_features: None,
        }
    }

    /// Copies a Cargo dependency declaration into a generated crate.
    ///
    /// Path sources are canonicalized relative to the declaring manifest and
    /// are later rebased relative to the generated crate. Git and registry
    /// version sources stay distributable and do not require their source
    /// crate to exist in Patchwork's downloaded mod cache.
    pub fn from_manifest(
        key: &str,
        value: &toml::Value,
        manifest_dir: &Path,
    ) -> Result<Self, Box<dyn Error>> {
        if let Some(version) = value.as_str() {
            return Ok(Self::version(key, version));
        }

        let table = value
            .as_table()
            .ok_or_else(|| format!("dependency '{key}' must be a string or table"))?;
        let package = table
            .get("package")
            .and_then(toml::Value::as_str)
            .map(str::to_string);
        let features = table
            .get("features")
            .map(|features| {
                features
                    .as_array()
                    .ok_or_else(|| format!("dependency '{key}' features must be an array"))?
                    .iter()
                    .map(|feature| {
                        feature.as_str().map(str::to_string).ok_or_else(|| {
                            format!("dependency '{key}' contains a non-string feature")
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let default_features = table
            .get("default-features")
            .map(|value| {
                value
                    .as_bool()
                    .ok_or_else(|| format!("dependency '{key}' default-features must be a bool"))
            })
            .transpose()?;

        let source = if let Some(path) = table.get("path").and_then(toml::Value::as_str) {
            let path = PathBuf::from(path);
            let path = if path.is_absolute() {
                path
            } else {
                manifest_dir.join(path)
            };
            GeneratedDependencySource::Path(path.canonicalize()?)
        } else if let Some(repository) = table.get("git").and_then(toml::Value::as_str) {
            GeneratedDependencySource::Git {
                repository: repository.to_string(),
                branch: optional_string(table, key, "branch")?,
                tag: optional_string(table, key, "tag")?,
                rev: optional_string(table, key, "rev")?,
            }
        } else if let Some(version) = table.get("version").and_then(toml::Value::as_str) {
            GeneratedDependencySource::Version(version.to_string())
        } else {
            return Err(
                format!("dependency '{key}' must declare one of path, git, or version").into(),
            );
        };

        Ok(Self {
            key: key.to_string(),
            package,
            source,
            features: features.unwrap_or_default(),
            default_features,
        })
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

    pub fn source_display(&self) -> String {
        match &self.source {
            GeneratedDependencySource::Version(version) => version.clone(),
            GeneratedDependencySource::Path(path) => path.display().to_string(),
            GeneratedDependencySource::Git { repository, .. } => repository.clone(),
        }
    }
}

fn optional_string(
    table: &toml::map::Map<String, toml::Value>,
    dependency: &str,
    field: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    table
        .get(field)
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| format!("dependency '{dependency}' {field} must be a string"))
        })
        .transpose()
        .map_err(Into::into)
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
        if dependency.package.is_none()
            && dependency.features.is_empty()
            && dependency.default_features.is_none()
        {
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
        GeneratedDependencySource::Git {
            repository,
            branch,
            tag,
            rev,
        } => {
            fields.push(format!("git = \"{repository}\""));
            if let Some(branch) = branch {
                fields.push(format!("branch = \"{branch}\""));
            }
            if let Some(tag) = tag {
                fields.push(format!("tag = \"{tag}\""));
            }
            if let Some(rev) = rev {
                fields.push(format!("rev = \"{rev}\""));
            }
        }
    }
    if let Some(default_features) = dependency.default_features {
        fields.push(format!("default-features = {default_features}"));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_git_dependency_is_preserved_for_generated_crates() {
        let manifest: toml::Value = toml::from_str(
            r#"
[dependencies]
helper = { git = "https://example.invalid/repository.git", branch = "main", package = "helper-package", default-features = false, features = ["serde"] }
"#,
        )
        .expect("test manifest must parse");
        let value = &manifest["dependencies"]["helper"];

        let dependency = GeneratedDependency::from_manifest("helper", value, Path::new("."))
            .expect("Git dependency must be accepted");

        assert_eq!(
            generate_dependency_toml_line(Path::new("."), &dependency),
            "helper = { package = \"helper-package\", git = \"https://example.invalid/repository.git\", branch = \"main\", default-features = false, features = [\"serde\"] }"
        );
    }

    #[test]
    fn string_dependency_is_preserved_as_a_registry_version() {
        let dependency = GeneratedDependency::from_manifest(
            "serde",
            &toml::Value::String("1.0".into()),
            Path::new("."),
        )
        .expect("version dependency must be accepted");

        assert_eq!(
            generate_dependency_toml_line(Path::new("."), &dependency),
            "serde = \"1.0\""
        );
    }
}
