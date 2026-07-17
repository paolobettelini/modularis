use std::collections::{BTreeMap, BTreeSet};

use crate::{
    DisplayContext, DisplayTransform, Element, Error, GuiLight, ModelSource, ResourceLocation,
    Result,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedModel {
    pub id: ResourceLocation,
    pub ambient_occlusion: bool,
    pub gui_light: Option<GuiLight>,
    pub textures: BTreeMap<String, String>,
    pub elements: Vec<Element>,
    pub display: BTreeMap<DisplayContext, DisplayTransform>,
    pub generated_item: bool,
    pub lineage: Vec<ResourceLocation>,
}

pub struct ModelResolver<'a, S: ModelSource + ?Sized> {
    source: &'a S,
    max_depth: usize,
}

impl<'a, S: ModelSource + ?Sized> ModelResolver<'a, S> {
    pub fn new(source: &'a S) -> Self {
        Self {
            source,
            max_depth: 64,
        }
    }

    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn resolve(&self, id: &ResourceLocation) -> Result<ResolvedModel> {
        let mut visiting = BTreeSet::new();
        self.resolve_inner(id, &mut visiting, 0)
    }

    fn resolve_inner(
        &self,
        id: &ResourceLocation,
        visiting: &mut BTreeSet<ResourceLocation>,
        depth: usize,
    ) -> Result<ResolvedModel> {
        if depth > self.max_depth {
            return Err(Error::ParentDepthExceeded(self.max_depth));
        }
        if !visiting.insert(id.clone()) {
            let chain = visiting
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" -> ");
            return Err(Error::ParentCycle(chain));
        }

        let raw = self
            .source
            .load_model(id)?
            .ok_or_else(|| Error::ModelNotFound(id.clone()))?;

        let parent_id = raw.parent.clone();
        let mut resolved = if let Some(parent) = &parent_id {
            self.resolve_inner(parent, visiting, depth + 1)?
        } else {
            ResolvedModel {
                id: id.clone(),
                ambient_occlusion: true,
                gui_light: None,
                textures: BTreeMap::new(),
                elements: Vec::new(),
                display: BTreeMap::new(),
                generated_item: is_generated_parent(id),
                lineage: Vec::new(),
            }
        };

        resolved.id = id.clone();
        resolved.lineage.push(id.clone());
        if let Some(value) = raw.ambientocclusion {
            resolved.ambient_occlusion = value;
        }
        if raw.gui_light.is_some() {
            resolved.gui_light = raw.gui_light;
        }
        resolved.textures.extend(raw.textures);
        if let Some(elements) = raw.elements {
            resolved.elements = elements;
        }
        resolved.display.extend(raw.display);
        resolved.generated_item |=
            parent_id.as_ref().map(is_generated_parent).unwrap_or(false) || is_generated_parent(id);

        visiting.remove(id);
        Ok(resolved)
    }
}

fn is_generated_parent(id: &ResourceLocation) -> bool {
    id.namespace() == "minecraft"
        && matches!(
            id.path(),
            "item/generated" | "item/handheld" | "builtin/generated"
        )
}
