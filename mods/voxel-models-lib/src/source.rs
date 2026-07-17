use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    BlockStateDefinition, Direction, Element, Error, ItemDefinition, Model, ModelFace,
    ResourceLocation, Result, parse_blockstate, parse_item_definition, parse_model,
};

pub trait ModelSource {
    fn load_model(&self, id: &ResourceLocation) -> Result<Option<Model>>;

    fn load_blockstate(&self, _id: &ResourceLocation) -> Result<Option<BlockStateDefinition>> {
        Ok(None)
    }

    fn load_item_definition(&self, _id: &ResourceLocation) -> Result<Option<ItemDefinition>> {
        Ok(None)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FsResourcePack {
    roots: Vec<PathBuf>,
    builtins: BuiltinModels,
}

/// Filesystem source for Patchwork's composed asset layout:
/// `assets/<mod-namespace>/models/<path>.json`.
///
/// A resource-location namespace is the asset-owning mod name, so a model can
/// inherit a template exported by another mod without merging asset folders.
#[derive(Debug, Clone, Default)]
pub struct ModAssetsResourcePack {
    root: PathBuf,
    builtins: BuiltinModels,
}

impl ModAssetsResourcePack {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            builtins: BuiltinModels::default(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn without_builtins(mut self) -> Self {
        self.builtins.enabled = false;
        self
    }

    fn load<T>(
        &self,
        id: &ResourceLocation,
        section: &str,
        parser: impl Fn(&[u8]) -> Result<T>,
    ) -> Result<Option<T>> {
        let path = mod_asset_path(&self.root, id, section)?;
        match fs::read(&path) {
            Ok(bytes) => parser(&bytes).map(Some),
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(source) => Err(Error::Io { path, source }),
        }
    }
}

impl ModelSource for ModAssetsResourcePack {
    fn load_model(&self, id: &ResourceLocation) -> Result<Option<Model>> {
        if let Some(model) = self.load(id, "models", parse_model)? {
            return Ok(Some(model));
        }
        self.builtins.load_model(id)
    }

    fn load_blockstate(&self, id: &ResourceLocation) -> Result<Option<BlockStateDefinition>> {
        self.load(id, "blockstates", parse_blockstate)
    }

    fn load_item_definition(&self, id: &ResourceLocation) -> Result<Option<ItemDefinition>> {
        self.load(id, "items", parse_item_definition)
    }
}

impl FsResourcePack {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            roots: vec![root.into()],
            builtins: BuiltinModels::default(),
        }
    }

    pub fn from_roots(roots: impl IntoIterator<Item = PathBuf>) -> Self {
        Self {
            roots: roots.into_iter().collect(),
            builtins: BuiltinModels::default(),
        }
    }

    /// Adds a higher-priority pack root. Later roots override earlier roots.
    pub fn push_root(&mut self, root: impl Into<PathBuf>) {
        self.roots.push(root.into());
    }

    pub fn roots(&self) -> &[PathBuf] {
        &self.roots
    }

    pub fn without_builtins(mut self) -> Self {
        self.builtins.enabled = false;
        self
    }

    fn load_from_roots<T>(
        &self,
        id: &ResourceLocation,
        section: &str,
        parser: impl Fn(&[u8]) -> Result<T>,
    ) -> Result<Option<T>> {
        for root in self.roots.iter().rev() {
            let path = resource_path(root, id, section)?;
            match fs::read(&path) {
                Ok(bytes) => return parser(&bytes).map(Some),
                Err(source) if source.kind() == std::io::ErrorKind::NotFound => continue,
                Err(source) => return Err(Error::Io { path, source }),
            }
        }
        Ok(None)
    }
}

impl ModelSource for FsResourcePack {
    fn load_model(&self, id: &ResourceLocation) -> Result<Option<Model>> {
        if let Some(model) = self.load_from_roots(id, "models", parse_model)? {
            return Ok(Some(model));
        }
        self.builtins.load_model(id)
    }

    fn load_blockstate(&self, id: &ResourceLocation) -> Result<Option<BlockStateDefinition>> {
        self.load_from_roots(id, "blockstates", parse_blockstate)
    }

    fn load_item_definition(&self, id: &ResourceLocation) -> Result<Option<ItemDefinition>> {
        self.load_from_roots(id, "items", parse_item_definition)
    }
}

fn resource_path(root: &Path, id: &ResourceLocation, section: &str) -> Result<PathBuf> {
    if id.path().split('/').any(|segment| segment == "..") {
        return Err(Error::UnsafeResourcePath(id.to_string()));
    }
    Ok(root
        .join("assets")
        .join(id.namespace())
        .join(section)
        .join(format!("{}.json", id.path())))
}

fn mod_asset_path(root: &Path, id: &ResourceLocation, section: &str) -> Result<PathBuf> {
    if id.path().split('/').any(|segment| segment == "..") {
        return Err(Error::UnsafeResourcePath(id.to_string()));
    }
    Ok(root
        .join(id.namespace())
        .join(section)
        .join(format!("{}.json", id.path())))
}

#[derive(Debug, Clone)]
pub struct BuiltinModels {
    enabled: bool,
}

impl Default for BuiltinModels {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl BuiltinModels {
    pub fn disabled() -> Self {
        Self { enabled: false }
    }

    fn full_cube(side: &str, end: &str) -> Model {
        let mut faces = std::collections::BTreeMap::new();
        for direction in Direction::ALL {
            let texture = match direction {
                Direction::Up | Direction::Down => end,
                _ => side,
            };
            faces.insert(
                direction,
                ModelFace {
                    uv: None,
                    texture: texture.to_owned(),
                    cullface: Some(direction),
                    rotation: 0,
                    tintindex: None,
                    extra: Default::default(),
                },
            );
        }
        Model {
            elements: Some(vec![Element {
                from: [0.0, 0.0, 0.0],
                to: [16.0, 16.0, 16.0],
                rotation: None,
                shade: true,
                light_emission: None,
                faces,
                extra: Default::default(),
            }]),
            ..Default::default()
        }
    }
}

impl ModelSource for BuiltinModels {
    fn load_model(&self, id: &ResourceLocation) -> Result<Option<Model>> {
        if !self.enabled || id.namespace() != "minecraft" {
            return Ok(None);
        }
        let model = match id.path() {
            "block/cube_all" => Self::full_cube("#all", "#all"),
            "block/cube_column" => Self::full_cube("#side", "#end"),
            "block/cube" => Self::full_cube("#side", "#end"),
            "item/generated" | "item/handheld" | "builtin/generated" => Model::default(),
            _ => return Ok(None),
        };
        Ok(Some(model))
    }
}
