//! Parser and baker for Minecraft Java Edition resource-pack models.
//!
//! The crate deliberately separates parsing/resolution from rendering. The core
//! API produces [`BakedQuad`] values that can be appended to a chunk mesh. Enable
//! the `bevy` feature to convert grouped baked geometry into Bevy 0.19 meshes.

mod bake;
mod blockstate;
mod document;
mod error;
mod item;
mod model;
mod resolve;
mod resource_location;
mod source;

#[cfg(feature = "bevy")]
pub mod bevy;

pub use bake::{
    BakeOptions, BakedMeshPart, BakedQuad, ModelTransform, bake_model, bake_model_with_transform,
    group_quads_by_texture,
};
pub use blockstate::{
    BlockStateDefinition, BlockStateModel, BlockStateSelection, ModelChoice, MultipartCase,
    PropertyMatcher, WhenClause,
};
pub use document::{JsonDocument, parse_document};
pub use error::{Error, Result};
pub use item::{
    ItemDefinition, ItemModel, ItemModelCase, ItemModelEntry, ItemModelKind, ItemProperty,
};
pub use model::{
    Axis, Direction, DisplayContext, DisplayTransform, Element, ElementRotation, GuiLight, Model,
    ModelFace,
};
pub use resolve::{ModelResolver, ResolvedModel};
pub use resource_location::ResourceLocation;
pub use source::{BuiltinModels, FsResourcePack, ModAssetsResourcePack, ModelSource};

/// Parses a Minecraft model JSON document.
pub fn parse_model(bytes: &[u8]) -> Result<Model> {
    Ok(serde_json::from_slice(bytes)?)
}

/// Parses a Minecraft blockstate JSON document.
pub fn parse_blockstate(bytes: &[u8]) -> Result<BlockStateDefinition> {
    Ok(serde_json::from_slice(bytes)?)
}

/// Parses a modern `assets/<namespace>/items/*.json` item definition.
pub fn parse_item_definition(bytes: &[u8]) -> Result<ItemDefinition> {
    Ok(serde_json::from_slice(bytes)?)
}
